use std::{fs, io::Read, path::PathBuf};

use flate2::bufread::ZlibDecoder;

use crate::{database::storage::DATABASE, git::object::HASH_SIZE, packs::get_object_encoded_data};

const HEADER_BYTES_V2: [u8; 8] = [0x50, 0x41, 0x43, 0x4B, 0, 0, 0, 2];
const HEADER_BYTES_V3: [u8; 8] = [0x50, 0x41, 0x43, 0x4B, 0, 0, 0, 3];

#[derive(Debug)]
enum GitPackType {
    Commit,
    Tree,
    Blob,
    Tag,
    ObjectOffsetDelta,
    ObjectReferenceDelta,
    Invalid,
}

impl From<u8> for GitPackType {
    fn from(byte: u8) -> Self {
        match byte {
            1 => GitPackType::Commit,
            2 => GitPackType::Tree,
            3 => GitPackType::Blob,
            4 => GitPackType::Tag,
            6 => GitPackType::ObjectOffsetDelta,
            7 => GitPackType::ObjectReferenceDelta,
            _ => GitPackType::Invalid,
        }
    }
}

pub fn get_encoded_data_from_pack(path: &PathBuf, offset: usize) -> Vec<u8> {
    let data = fs::read(path);
    if data.is_err() || !is_header_valid(&data.as_ref().unwrap()[..8]) {
        return Vec::new();
    }

    // Skip to the offset
    let mut data = &data.unwrap()[offset..];
    let object_type: GitPackType = ((data[0] >> 4) & 0b0111).into();
    let mut size: usize = (data[0] & 0b0000_1111) as usize;

    let mut shift: usize = 4;
    while data[0] & 0b1000_0000 != 0 {
        data = &data[1..];

        size |= ((data[0] & 0b0111_1111) as usize) << shift;
        shift += 7;
    }

    match object_type {
        GitPackType::ObjectOffsetDelta => parse_offset_delta(path, &data[1..], size, offset),
        GitPackType::ObjectReferenceDelta => parse_reference_delta(&data[1..], size),
        GitPackType::Invalid => panic!(),
        _ => {
            let mut decoded_data: Vec<u8> = vec![0; size];
            ZlibDecoder::new(&data[1..])
                .read_exact(&mut decoded_data)
                .unwrap();

            decoded_data
        }
    }
}

fn parse_offset_delta(path: &PathBuf, data: &[u8], _size: usize, current_offset: usize) -> Vec<u8> {
    let Some((relative_offset, offset_shift)) = offset_delta_base_distance(data) else {
        panic!("pack: truncated offset-delta distance encoding");
    };
    if relative_offset == 0 {
        panic!("pack: offset-delta base distance is zero (would recurse into same object)");
    }
    let mut delta_data = Vec::new();
    ZlibDecoder::new(&data[offset_shift..])
        .read_to_end(&mut delta_data)
        .unwrap();
    let (source_size, source_shift) = variable_length_int(&delta_data);
    let (target_size, target_shift) = variable_length_int(&delta_data[source_shift..]);
    let delta_instructions = &delta_data[source_shift + target_shift..];

    let Some(base_offset) = current_offset.checked_sub(relative_offset) else {
        panic!("pack: offset-delta base offset underflow at object offset {current_offset}");
    };
    let source = get_encoded_data_from_pack(path, base_offset);
    if source.len() != source_size {
        panic!();
    }

    parse_delta_instructions(&source, delta_instructions, target_size)
}

fn parse_reference_delta(data: &[u8], size: usize) -> Vec<u8> {
    let hash = &data[..HASH_SIZE];
    let mut delta_data = Vec::with_capacity(size);
    ZlibDecoder::new(&data[HASH_SIZE..])
        .read_to_end(&mut delta_data)
        .unwrap();
    let (source_size, source_shift) = variable_length_int(&delta_data);
    let (target_size, target_shift) = variable_length_int(&delta_data[source_shift..]);
    let delta_instructions = &delta_data[source_shift + target_shift..];

    let project = DATABASE.lock().unwrap().get_current_project().unwrap();
    let source = &get_object_encoded_data(&project, &hex::encode(hash)).unwrap_or_default();
    if source.len() != source_size {
        panic!();
    }

    parse_delta_instructions(source, delta_instructions, target_size)
}

fn parse_delta_instructions(
    source: &[u8],
    delta_instructions: &[u8],
    target_size: usize,
) -> Vec<u8> {
    let mut target = Vec::<u8>::new();

    let mut delta_instructions = delta_instructions;
    while !delta_instructions.is_empty() {
        let instruction = delta_instructions[0];

        if instruction & 0b1000_0000 != 0 {
            // Copy Instruction
            let offset_1 = (instruction & 0b1) == 0b1;
            let offset_2 = (instruction & 0b10) == 0b10;
            let offset_3 = (instruction & 0b100) == 0b100;
            let offset_4 = (instruction & 0b1000) == 0b1000;
            let size_1 = (instruction & 0b1_0000) == 0b1_0000;
            let size_2 = (instruction & 0b10_0000) == 0b10_0000;
            let size_3 = (instruction & 0b100_0000) == 0b100_0000;

            let mut offset: usize = 0;
            let mut size: usize = 0;
            // Skip the instruction byte
            delta_instructions = &delta_instructions[1..];

            // Parse the offset and size
            // and advance the delta instructions
            if offset_1 {
                offset = delta_instructions[0] as usize;
                delta_instructions = &delta_instructions[1..];
            }

            if offset_2 {
                offset |= (delta_instructions[0] as usize) << 8;
                delta_instructions = &delta_instructions[1..];
            }

            if offset_3 {
                offset |= (delta_instructions[0] as usize) << 16;
                delta_instructions = &delta_instructions[1..];
            }

            if offset_4 {
                offset |= (delta_instructions[0] as usize) << 24;
                delta_instructions = &delta_instructions[1..];
            }

            if size_1 {
                size = delta_instructions[0] as usize;
                delta_instructions = &delta_instructions[1..];
            }

            if size_2 {
                size |= (delta_instructions[0] as usize) << 8;
                delta_instructions = &delta_instructions[1..];
            }

            if size_3 {
                size |= (delta_instructions[0] as usize) << 16;
                delta_instructions = &delta_instructions[1..];
            }

            // If the size is set to 0, it means max size
            if size == 0 {
                size = 0x10000;
            }

            // Copy the data from the source to the target
            target.extend_from_slice(&source[offset..offset + size]);
        } else if instruction != 0 {
            // Insert Instruction
            let add_size = (instruction & 0b0111_1111) as usize;
            target.extend_from_slice(&delta_instructions[1..add_size + 1]);

            delta_instructions = &delta_instructions[add_size + 1..];
        } else {
            // instruction == 0 is reserved/invalid; advance to avoid infinite loop
            delta_instructions = &delta_instructions[1..];
        }
    }

    if target.len() != target_size {
        panic!();
    }

    target
}

fn variable_length_int(data: &[u8]) -> (usize, usize) {
    let mut data = data;

    let mut size: usize = (data[0] & 0b0111_1111) as usize;
    let mut shift: usize = 7;

    while data[0] & 0b1000_0000 != 0 {
        data = &data[1..];
        size |= ((data[0] & 0b0111_1111) as usize) << shift;
        shift += 7;
    }

    (size, shift / 7)
}

fn offset_delta_base_distance(data: &[u8]) -> Option<(usize, usize)> {
    if data.is_empty() {
        return None;
    }
    let mut distance = (data[0] & 0b0111_1111) as usize;
    let mut bytes_read = 1;

    while data[bytes_read - 1] & 0b1000_0000 != 0 {
        if bytes_read >= data.len() {
            return None;
        }
        let byte = data[bytes_read];
        distance = ((distance + 1) << 7) | (byte & 0b0111_1111) as usize;
        bytes_read += 1;
    }

    Some((distance, bytes_read))
}

fn is_header_valid(header: &[u8]) -> bool {
    header == HEADER_BYTES_V2 || header == HEADER_BYTES_V3
}

#[cfg(test)]
pub mod tests {
    use std::io::Write;

    use flate2::write::ZlibEncoder;
    use fs::File;

    use crate::git::git_commit::GitCommit;

    use super::*;

    fn mocked_header_v2() -> [u8; 8] {
        HEADER_BYTES_V2
    }

    #[test]
    fn test_offset_delta_base_distance() {
        assert_eq!(offset_delta_base_distance(&[0x01]), Some((1, 1)));
        assert_eq!(offset_delta_base_distance(&[0x81, 0x00]), Some((256, 2)));
        assert_eq!(offset_delta_base_distance(&[0x81, 0x01]), Some((257, 2)));
        assert_eq!(offset_delta_base_distance(&[]), None);
        assert_eq!(offset_delta_base_distance(&[0x81]), None); // truncated multi-byte
    }

    #[test]
    fn test_parse_delta_instructions_insert_only() {
        let source = b"ignored";
        let delta = [3u8, b'a', b'b', b'c'];
        assert_eq!(parse_delta_instructions(source, &delta, 3), b"abc");
    }

    fn mocked_commit(commit: &GitCommit) -> Vec<u8> {
        let mut data = Vec::new();

        let commit_string = format!("{}\n", commit);
        let mut cmt = commit_string.as_bytes();
        let mut zlib = ZlibEncoder::new(Vec::new(), flate2::Compression::default());
        while !cmt.is_empty() {
            let len = cmt.len().min(0x10000);
            zlib.write_all(&cmt[..len]).unwrap();
            cmt = &cmt[len..];
        }
        let encoded_data = zlib.finish().unwrap();

        let size = encoded_data.len();
        data.push(0b1001_0000 | (size & 0b1111) as u8);

        let mut size = size >> 4;
        loop {
            data.push((size & 0b0111_1111) as u8 | 0b1000_0000);
            size >>= 7;

            if size == 0 {
                let datalen = data.len();
                data[datalen - 1] &= 0b0111_1111;
                break;
            }
        }

        data.extend_from_slice(&encoded_data);
        data
    }

    pub fn mocked_pack_file_with_commit(file: &mut File, commit: &GitCommit) {
        let mut data = Vec::new();
        data.extend_from_slice(&mocked_header_v2());
        data.extend_from_slice(&mocked_commit(commit));

        file.write(&data).unwrap();
    }

    fn encode_size_varint(mut value: usize) -> Vec<u8> {
        let mut bytes = Vec::new();
        loop {
            let mut byte = (value & 0b0111_1111) as u8;
            value >>= 7;
            if value != 0 {
                byte |= 0b1000_0000;
            }
            bytes.push(byte);
            if value == 0 {
                return bytes;
            }
        }
    }

    fn encode_offset_delta_base_distance(mut distance: usize) -> Vec<u8> {
        let mut bytes = vec![(distance & 0b0111_1111) as u8];
        distance >>= 7;

        while distance > 0 {
            distance -= 1;
            bytes.push((distance & 0b0111_1111) as u8 | 0b1000_0000);
            distance >>= 7;
        }

        bytes.reverse();
        bytes
    }

    #[test]
    fn test_read_offset_delta_from_packfile() {
        let path = std::env::temp_dir().join(format!(
            "branchwise-pack-{}.pack",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));

        let source = b"abcde";
        let target = b"abXYe";

        let mut base_object = vec![0b0011_0000 | source.len() as u8];
        let mut source_encoder = ZlibEncoder::new(Vec::new(), flate2::Compression::default());
        source_encoder.write_all(source).unwrap();
        base_object.extend_from_slice(&source_encoder.finish().unwrap());

        let delta_instructions = vec![
            0b1001_0000,
            2, // copy 2 bytes from source offset 0
            2,
            b'X',
            b'Y', // insert "XY"
            0b1001_0001,
            4,
            1, // copy 1 byte from source offset 4
        ];

        let mut delta_data = Vec::new();
        delta_data.extend_from_slice(&encode_size_varint(source.len()));
        delta_data.extend_from_slice(&encode_size_varint(target.len()));
        delta_data.extend_from_slice(&delta_instructions);

        let mut delta_encoder = ZlibEncoder::new(Vec::new(), flate2::Compression::default());
        delta_encoder.write_all(&delta_data).unwrap();
        let compressed_delta = delta_encoder.finish().unwrap();

        let base_offset = 8usize;
        let delta_offset = base_offset + base_object.len();
        let base_distance = delta_offset - base_offset;
        let encoded_distance = encode_offset_delta_base_distance(base_distance);

        let mut delta_object = vec![0b0110_0000 | target.len() as u8];
        delta_object.extend_from_slice(&encoded_distance);
        delta_object.extend_from_slice(&compressed_delta);

        let mut file = File::create(&path).unwrap();
        file.write_all(&mocked_header_v2()).unwrap();
        file.write_all(&base_object).unwrap();
        file.write_all(&delta_object).unwrap();
        drop(file);

        let decoded = get_encoded_data_from_pack(&path, delta_offset);
        assert_eq!(decoded, target);

        fs::remove_file(path).unwrap();
    }
}
