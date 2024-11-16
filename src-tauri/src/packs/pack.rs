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
    let object_type: GitPackType = (data[0] >> 4 & 0b0111).into();
    let mut size: usize = (data[0] & 0b0000_1111) as usize;

    let mut shift: usize = 4;
    while data[0] & 0b1000_0000 != 0 {
        data = &data[1..];

        size |= ((data[0] & 0b0111_1111) as usize) << shift;
        shift += 7;
    }

    match object_type {
        GitPackType::ObjectOffsetDelta => parse_offset_delta(path, &data[1..], size),
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

fn parse_offset_delta(path: &PathBuf, data: &[u8], size: usize) -> Vec<u8> {
    let (offset, offset_shift) = variable_length_int(data);
    let (source_size, source_shift) = variable_length_int(&data[offset_shift..]);
    let (target_size, target_shift) = variable_length_int(&data[offset_shift + source_shift..]);

    let mut delta_instructions: Vec<u8> =
        vec![0; size - offset_shift - source_shift - target_shift];
    ZlibDecoder::new(&data[offset_shift + source_shift + target_shift..])
        .read_exact(&mut delta_instructions)
        .unwrap();

    let source = get_encoded_data_from_pack(path, offset);
    if source.len() != source_size {
        panic!();
    }

    parse_delta_instructions(&source, &delta_instructions, target_size)
}

fn parse_reference_delta(data: &[u8], size: usize) -> Vec<u8> {
    let hash = &data[..HASH_SIZE];

    let (source_size, source_shift) = variable_length_int(&data[HASH_SIZE..]);
    let (target_size, target_shift) = variable_length_int(&data[HASH_SIZE + source_shift..]);

    let mut delta_instructions: Vec<u8> = vec![0; size - source_shift - target_shift - HASH_SIZE];
    ZlibDecoder::new(&data[HASH_SIZE + source_shift + target_shift..])
        .read_exact(&mut delta_instructions)
        .unwrap();

    let project = DATABASE.lock().unwrap().get_current_project().unwrap();
    let source = &get_object_encoded_data(&project, &hex::encode(hash)).unwrap_or_default();
    if source.len() != source_size {
        panic!();
    }

    parse_delta_instructions(source, &delta_instructions, target_size)
}

fn parse_delta_instructions(
    source: &[u8],
    delta_instructions: &[u8],
    target_size: usize,
) -> Vec<u8> {
    let mut target = Vec::<u8>::new();

    let mut delta_instructions = delta_instructions;
    while delta_instructions.is_empty() {
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

fn is_header_valid(header: &[u8]) -> bool {
    header == HEADER_BYTES_V2 || header == HEADER_BYTES_V3
}
