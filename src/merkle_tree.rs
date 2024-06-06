use std::fmt;
use std::hash::Hasher;
use sha2::{Digest, Sha256};
use merkletree::hash::Algorithm;
use merkletree::merkle::Element;

pub const SIZE: usize = 0x20;

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug, Default)]
pub struct TestItem([u8; SIZE]);

impl AsRef<[u8]> for TestItem {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl Element for TestItem {
    fn byte_len() -> usize {
        SIZE
    }

    fn from_slice(bytes: &[u8]) -> Self {
        assert_eq!(bytes.len(), Self::byte_len());
        let mut el = [0u8; SIZE];
        el[..].copy_from_slice(bytes);
        TestItem(el)
    }

    fn copy_to_slice(&self, bytes: &mut [u8]) {
        bytes.copy_from_slice(&self.0);
    }
}

pub struct TestSha256Hasher {
    engine: Sha256,
}

impl TestSha256Hasher {
    pub fn new() -> TestSha256Hasher {
        TestSha256Hasher {
            engine: Sha256::new(),
        }
    }
}

impl fmt::Debug for TestSha256Hasher {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Sha256Hasher")
    }
}

impl Default for TestSha256Hasher {
    fn default() -> Self {
        TestSha256Hasher::new()
    }
}

impl Hasher for TestSha256Hasher {
    fn finish(&self) -> u64 {
        unimplemented!("Hasher's contract (finish function is not used) is deliberately broken by design")
    }
    fn write(&mut self, bytes: &[u8]) {
        self.engine.update(bytes)
    }
}

impl Algorithm<TestItem> for TestSha256Hasher {
    fn hash(&mut self) -> TestItem {
        let mut result = TestItem::default();
        let item_size = result.0.len();
        let hash_output = self.engine.clone().finalize().to_vec();
        self.engine.reset();
        if item_size < hash_output.len() {
            result.0.copy_from_slice(&hash_output.as_slice()[0..item_size]);
        } else {
            result.0.copy_from_slice(hash_output.as_slice())
        }
        result
    }
}

