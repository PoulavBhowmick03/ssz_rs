use crate::{Deserialize, HashTreeRoot, Serialize};
use std::fmt::Error;

impl Serialize for bool {
    fn serialize(&self, buf: &mut Vec<u8>) {
        buf.push(if *self { 1 } else { 0 });
    }
}

impl Deserialize for bool {
    fn deserialize(bytes: &[u8]) -> Result<(Self, usize), Error> {
        if bytes.is_empty() {
            Err(Error)
        } else {
            match bytes.len() {
                0 => Err(Error),
                1 => match bytes[0] {
                    0 => Ok((false, 1)),
                    1 => Ok((true, 1)),
                    _ => Err(Error),
                },
                _ => Err(Error),
            }
        }
    }
}

impl HashTreeRoot for bool {
    fn hash_tree_root(&self) -> [u8; 32] {
        let mut root = [0u8; 32];
        root[0] = if *self { 1 } else { 0 };
        root
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_bool() {
        let mut buffer: Vec<u8> = Vec::new();
        true.serialize(&mut buffer);
        assert!(buffer == vec![1], "Serialized true should be [1]");

        buffer.clear();

        false.serialize(&mut buffer);
        assert!(buffer == vec![0], "Serialized false should be [0]");
    }

    #[test]
    fn deserialize_bool() {
        let bytes_true = vec![1];
        let (value_true, size_true) = bool::deserialize(&bytes_true).unwrap();
        assert!(value_true, "Deserialized value should be true");
        assert_eq!(size_true, 1, "Size should be 1 for true");

        let bytes_false = vec![0];
        let (value_false, size_false) = bool::deserialize(&bytes_false).unwrap();
        assert!(!value_false, "Deserialized value should be false");
        assert_eq!(size_false, 1, "Size should be 1 for false");
    }

    #[test]
    fn hash_tree_root_bool() {
        let true_root = true.hash_tree_root();
        let false_root = false.hash_tree_root();
        assert_eq!(
            true_root[0], 1,
            "Hash tree root for true should start with 1"
        );
        assert_eq!(
            false_root[0], 0,
            "Hash tree root for false should start with 0"
        );
        assert_eq!(
            &true_root[1..],
            &[0; 31],
            "Remaining bytes should be zero for true"
        );
        assert_eq!(
            &false_root[1..],
            &[0; 31],
            "Remaining bytes should be zero for false"
        );
    }
}
