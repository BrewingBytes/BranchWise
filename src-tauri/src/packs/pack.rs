use std::{fs, io::Read, path::PathBuf};

use flate2::bufread::ZlibDecoder;

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
        GitPackType::ObjectOffsetDelta => todo!(),
        GitPackType::ObjectReferenceDelta => todo!(),
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

fn is_header_valid(header: &[u8]) -> bool {
    header == HEADER_BYTES_V2 || header == HEADER_BYTES_V3
}
