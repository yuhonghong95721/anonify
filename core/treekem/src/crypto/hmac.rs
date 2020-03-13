use std::vec::Vec;

#[derive(Debug, Clone)]
pub struct HmacKey(Vec<u8>);

impl HmacKey {
    pub fn zero(len: usize) -> Self {
        HmacKey(vec![0u8; len])
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0[..]
    }

    pub fn as_mut_bytes(&mut self) -> &mut [u8] {
        &mut self.0[..]
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.0
    }
}

impl From<Vec<u8>> for HmacKey {
    fn from(vec: Vec<u8>) -> Self {
        HmacKey(vec)
    }
}

impl From<&[u8]> for HmacKey {
    fn from(bytes: &[u8]) -> Self {
        HmacKey(bytes.to_vec())
    }
}
