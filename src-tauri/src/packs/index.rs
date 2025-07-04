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
    log::debug!("Checking {hash} is in pack-idx {}", index.to_string_lossy());

    let data = fs::read(index);
    if data.is_err() || !is_header_valid(&data.as_ref().unwrap()[..8]) {
        log::debug!("Invalid pack-idx");
        
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

#[cfg(test)]
pub mod tests {
    use std::path::Path;

    use super::*;

    use tempdir::TempDir;

    fn mocked_header() -> [u8; 8] {
        HEADER_BYTES
    }

    fn mocked_fanout_table(el: usize) -> Vec<u8> {
        let mut fanout_table = Vec::new();
        for _ in 0..256 {
            fanout_table.push(0);
            fanout_table.push(0);
            fanout_table.push(0);
            fanout_table.push(0);
        }

        fanout_table[el * 4 + 3] = 1;
        fanout_table[255 * 4 + 3] = 1;

        fanout_table
    }

    fn mocked_object_table(hash: &str) -> Vec<u8> {
        let mut object_table = Vec::new();
        object_table.extend_from_slice(&<[u8; 20]>::from_hex(hash).unwrap());

        object_table
    }

    fn mocked_crc_table() -> Vec<u8> {
        let mut crc_table = Vec::new();
        crc_table.push(0);
        crc_table.push(0);
        crc_table.push(0);
        crc_table.push(0);

        crc_table
    }

    fn mocked_small_offset_table(offset: usize) -> Vec<u8> {
        let mut offset_table = Vec::new();
        offset_table.extend_from_slice(&u32::to_be_bytes(offset as u32));

        offset_table
    }

    fn mocked_big_offset_table(offset: usize) -> Vec<u8> {
        let mut offset_table = Vec::new();
        offset_table.push((offset >> 56) as u8);
        offset_table.push((offset >> 48) as u8);
        offset_table.push((offset >> 40) as u8);
        offset_table.push((offset >> 32) as u8);
        offset_table.push((offset >> 24) as u8);
        offset_table.push((offset >> 16) as u8);
        offset_table.push((offset >> 8) as u8);
        offset_table.push(offset as u8);

        offset_table
    }

    pub fn create_mocked_index_file(
        temp_dir: &Path,
        small_offset: bool,
        set_offset: usize,
        index_name: &str,
        hash: &str,
    ) -> PathBuf {
        let index = temp_dir.join(index_name);
        let el = usize::from_str_radix(&hash[..2], 16).unwrap_or_default();

        let mut data = Vec::new();
        data.extend_from_slice(&mocked_header());
        data.extend_from_slice(&mocked_fanout_table(el));
        data.extend_from_slice(&mocked_object_table(hash));
        data.extend_from_slice(&mocked_crc_table());
        if small_offset {
            data.extend_from_slice(&mocked_small_offset_table(set_offset));
        } else {
            data.extend_from_slice(&mocked_small_offset_table(0x80000000));
            data.extend_from_slice(&mocked_big_offset_table(set_offset));
        }

        fs::write(&index, data).unwrap();
        index
    }

    #[test]
    fn test_hash_in_index_offset_big() {
        let temp_dir = TempDir::new("test_hash_in_index_offset_big").unwrap();

        let hash = "1234567890123456789012345678901234567890";
        let index = create_mocked_index_file(&temp_dir.path(), false, 12, "index.idx", hash);

        let (is_hash_in_index, offset) = is_hash_in_index(&index, hash);
        assert_eq!(is_hash_in_index, true);
        assert_eq!(offset, 12);
    }

    #[test]
    fn test_hash_in_index_offset_small() {
        let temp_dir = TempDir::new("test_hash_in_index_offset_small").unwrap();

        let hash = "1234567890123456789012345678901234567890";
        let index = create_mocked_index_file(&temp_dir.path(), true, 1, "index.idx", hash);

        let (is_hash_in_index, offset) = is_hash_in_index(&index, hash);
        assert_eq!(is_hash_in_index, true);
        assert_eq!(offset, 1);
    }

    #[test]
    fn test_hash_not_in_index() {
        let temp_dir = TempDir::new("test_hash_not_in_index").unwrap();

        let hash = "1234567890123456789012345678901234567890";
        let index = create_mocked_index_file(&temp_dir.path(), true, 1, "index.idx", hash);
        let hash = "1234567890123456789012345678901234567891";

        let (is_hash_in_index, offset) = is_hash_in_index(&index, hash);
        assert_eq!(is_hash_in_index, false);
        assert_eq!(offset, 0);
    }

    #[test]
    fn test_invalid_header() {
        let temp_dir = TempDir::new("test_invalid_header").unwrap();

        let index = temp_dir.path().join("index");
        fs::write(&index, vec![0, 1, 2, 3, 4, 5, 6, 7]).unwrap();

        let hash = "1234567890123456789012345678901234567890";

        let (is_hash_in_index, _) = is_hash_in_index(&index, hash);
        assert_eq!(is_hash_in_index, false);
    }
}
