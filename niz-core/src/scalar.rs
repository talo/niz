use crate::hash::{self, Hashable};

macro_rules! impl_hashable_for_to_be_bytes {
    ($($t:ty),*) => {
        $(
            impl Hashable for $t {
                fn hash(&self) -> [u8; 32] {
                    use tiny_keccak::{Hasher, Sha3};

                    let mut output = [0u8; 32];
                    let mut hasher = Sha3::v256();
                    hasher.update(&self.to_be_bytes());
                    hasher.finalize(&mut output);

                    output
                }
            }
        )*
    };
}

impl_hashable_for_to_be_bytes!(u8);
impl_hashable_for_to_be_bytes!(u16);
impl_hashable_for_to_be_bytes!(u32);
impl_hashable_for_to_be_bytes!(u64);
impl_hashable_for_to_be_bytes!(u128);

impl_hashable_for_to_be_bytes!(i8);
impl_hashable_for_to_be_bytes!(i16);
impl_hashable_for_to_be_bytes!(i32);
impl_hashable_for_to_be_bytes!(i64);
impl_hashable_for_to_be_bytes!(i128);

impl Hashable for str {
    fn hash(&self) -> [u8; 32] {
        use tiny_keccak::{Hasher, Sha3};

        let mut val_output = [0u8; 32];
        let mut val_hasher = Sha3::v256();
        val_hasher.update(self.as_bytes());
        val_hasher.finalize(&mut val_output);

        let mut output = [0u8; 32];
        let mut hasher = Sha3::v256();
        hasher.update(&hash::prefix("string"));
        hasher.update(&val_output);
        hasher.finalize(&mut output);
        output
    }
}
