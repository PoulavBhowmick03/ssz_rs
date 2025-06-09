use std::fmt::Error;

fn main() {
    println!("Hello, world!");
}
pub trait HashTreeRoot {
    fn hash_tree_root(&self) -> [u8; 32];
}

pub trait Serialize {
    fn serialize (&self, buf: &mut Vec<u8>);
}

pub trait Deserialize: Sized {
    fn deserialize(bytes: &[u8]) -> Result<(Self, usize), Error>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_tree_root() {

        #[derive(Debug)]
        struct Example;

        impl HashTreeRoot for Example {
            fn hash_tree_root(&self) -> [u8; 32] {
                [10; 32]
            }
        }
        let example = Example;
        let root = example.hash_tree_root();
        assert_eq!(root, [10; 32], "Hash tree root should be an array of 32 zeros");
    }

    #[test]
    fn test_serialize () {

        #[derive(Debug)]
        struct Example;

        impl Serialize for Example {
            fn serialize(&self, buf: &mut Vec<u8>) {
                buf.extend_from_slice(&[1, 2, 3, 4]);
            }
        }
        let example = Example;
        let mut buffer: Vec<u8> = Vec::new();
        example.serialize(buffer.as_mut());
        assert_eq!(buffer, vec![1, 2, 3, 4], "Serialized buffer should match expected values");
        
    }

    #[test]
    fn test_deserialize() {
        #[derive(Debug, PartialEq)]
        struct Example;

        impl Deserialize for Example {
            fn deserialize(bytes: &[u8]) -> Result<(Self, usize), Error> {
                if bytes.len() < 4 {
                    return Err(Error);
                }
                Ok((Example, 4)) // Assuming we read 4 bytes
            }
        }

        let bytes = vec![1, 2, 3, 4];
        let (example, size) = Example::deserialize(&bytes).unwrap();
        assert_eq!(size, 4, "Size should be 4");
        assert_eq!(example, Example, "Deserialized object should match expected");
    }
}
