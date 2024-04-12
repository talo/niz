pub trait Hashable {
    fn hash(&self) -> [u8; 32];
}

impl<T> Hashable for &T
where
    T: Hashable,
{
    fn hash(&self) -> [u8; 32] {
        (*self).hash()
    }
}

pub fn prefix(label: impl AsRef<str>) -> [u8; 32] {
    use tiny_keccak::{Hasher, Sha3};

    let mut output = [0u8; 32];
    let mut hasher = Sha3::v256();
    hasher.update(label.as_ref().as_bytes());
    hasher.finalize(&mut output);
    output
}
