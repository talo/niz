use std::collections::HashMap;

use crate::hash::{self, Hashable};

impl<A, B> Hashable for (A, B)
where
    A: Hashable,
    B: Hashable,
{
    fn hash(&self) -> [u8; 32] {
        use tiny_keccak::{Hasher, Sha3};

        let mut output = [0u8; 32];
        let mut hasher = Sha3::v256();
        hasher.update(&hash::prefix("tuple"));
        hasher.update(&self.0.hash());
        hasher.update(&self.1.hash());
        hasher.finalize(&mut output);
        output
    }
}

impl<T> Hashable for [T]
where
    T: Hashable,
{
    fn hash(&self) -> [u8; 32] {
        use tiny_keccak::{Hasher, Sha3};

        let mut output = [0u8; 32];
        let mut hasher = Sha3::v256();
        hasher.update(&hash::prefix("list"));
        if self.is_empty() {
            hasher.update(&[0u8; 32]);
        } else {
            for item in self {
                hasher.update(&item.hash());
            }
        }
        hasher.finalize(&mut output);
        output
    }
}

impl<A, B> Hashable for HashMap<A, B>
where
    A: Hashable,
    B: Hashable,
{
    fn hash(&self) -> [u8; 32] {
        use tiny_keccak::{Hasher, Sha3};

        let mut output = [0u8; 32];
        let mut hasher = Sha3::v256();
        hasher.update(&hash::prefix("map"));
        if self.is_empty() {
            hasher.update(&[0u8; 32]);
        } else {
            for (key, value) in self {
                hasher.update(&(key, value).hash());
            }
        }
        hasher.finalize(&mut output);
        output
    }
}

impl<A> Hashable for Option<A>
where
    A: Hashable,
{
    fn hash(&self) -> [u8; 32] {
        use tiny_keccak::{Hasher, Sha3};

        let mut output = [0u8; 32];
        let mut hasher = Sha3::v256();
        hasher.update(&hash::prefix("optional"));

        let mut value_output = [0u8; 32];
        if let Some(value) = self {
            let mut value_hasher = Sha3::v256();
            value_hasher.update(&value.hash());
            value_hasher.finalize(&mut value_output);
        }
        hasher.update(&value_output);

        hasher.finalize(&mut output);
        output
    }
}

impl<A, B> Hashable for Result<A, B>
where
    A: Hashable,
    B: Hashable,
{
    fn hash(&self) -> [u8; 32] {
        use tiny_keccak::{Hasher, Sha3};

        let mut output = [0u8; 32];
        let mut hasher = Sha3::v256();
        hasher.update(&hash::prefix("result"));

        let mut value_output = [0u8; 32];
        let mut value_hasher = Sha3::v256();
        match self {
            Ok(value) => {
                value_hasher.update(&hash::prefix("ok"));
                value_hasher.update(&value.hash());
            }
            Err(e) => {
                value_hasher.update(&hash::prefix("err"));
                value_hasher.update(&e.hash());
            }
        }
        value_hasher.finalize(&mut value_output);

        hasher.update(&value_output);
        hasher.finalize(&mut output);
        output
    }
}
