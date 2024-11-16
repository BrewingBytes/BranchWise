use hex::FromHex;
use std::{fs, path::PathBuf};

use crate::git::object::HASH_SIZE;

const HEADER_BYTES: [u8; 8] = [0xff, 0x74, 0x4f, 0x63, 0, 0, 0, 2];

/**
 * Check if the hash is in the index file
 *
 * index: The path to the index file
 * hash: The hash to check
 *
 * Returns a tuple with a boolean if the hash is in the index and the offset of the object in the pack
 */
pub fn is_hash_in_index(index: &PathBuf, hash: &str) -> (bool, usize) {
    let data = fs::read(index);
    if data.is_err() || !is_header_valid(&data.as_ref().unwrap()[..8]) {
        return (false, 0);
    }

    // Skip the header
    let data = &data.unwrap()[8..];

    let last_index = 255;
    let total_objects = u32::from_be_bytes([
        data[last_index * 4],
        data[last_index * 4 + 1],
        data[last_index * 4 + 2],
        data[last_index * 4 + 3],
    ]) as usize;

    // Get the start and end offset of the hash
    let idx = usize::from_str_radix(&hash[..2], 16).unwrap_or_default() * 4;
    let start_offset;
    let end_offset;

    // Check if the index is the first one
    // If it is, the start offset is 0
    // Otherwise, the start offset is the previous end offset
    if idx == 0 {
        start_offset = 0_usize;
        end_offset = u32::from_be_bytes([data[4], data[5], data[6], data[7]]) as usize;
    } else {
        start_offset =
            u32::from_be_bytes([data[idx - 4], data[idx - 3], data[idx - 2], data[idx - 1]])
                as usize;
        end_offset =
            u32::from_be_bytes([data[idx], data[idx + 1], data[idx + 2], data[idx + 3]]) as usize;
    }

    // Skip the fanout table
    let data = &data[1024..];

    // Check if the hash is in the index
    for i in start_offset..end_offset {
        // Compare the hash in the index with the hash
        if data[i * 20..i * 20 + HASH_SIZE] == <[u8; HASH_SIZE]>::from_hex(hash).unwrap() {
            return (true, find_object_offset(data, total_objects, i));
        }
    }

    // The hash is not in the index
    (false, 0)
}

fn find_object_offset(data: &[u8], total_objects: usize, index: usize) -> usize {
    // Skip the object names
    let data = &data[total_objects * 20..];

    // Skip the CRCs
    let data = &data[total_objects * 4..];

    let index = index * 4;
    let offset = u32::from_be_bytes([
        data[index],
        data[index + 1],
        data[index + 2],
        data[index + 3],
    ]) as usize;
    if data[index] & 0x80 == 0 {
        // If the offset has the highest bit not set, it is a small offset
        offset
    } else {
        // If the offset has the highest bit set, it is a large offset
        let index = (offset & 0x7FFFFFFF) * 8;

        // Skip the 4 byte offset table
        let data = &data[total_objects * 4..];

        u64::from_be_bytes([
            data[index],
            data[index + 1],
            data[index + 2],
            data[index + 3],
            data[index + 4],
            data[index + 5],
            data[index + 6],
            data[index + 7],
        ]) as usize
    }
}

fn is_header_valid(header: &[u8]) -> bool {
    header == HEADER_BYTES
}
