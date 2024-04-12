use std::collections::BTreeMap;

use chrono::{DateTime, Utc};
use serde_json::{Map, Value};
use uuid::Uuid;

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

impl_hashable_for_to_be_bytes!(f32);
impl_hashable_for_to_be_bytes!(f64);

impl Hashable for bool {
    fn hash(&self) -> [u8; 32] {
        use tiny_keccak::{Hasher, Sha3};

        let mut output = [0u8; 32];
        let mut hasher = Sha3::v256();
        hasher.update(if *self { &[1u8] } else { &[0u8] });
        hasher.finalize(&mut output);

        output
    }
}

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

impl Hashable for String {
    fn hash(&self) -> [u8; 32] {
        <String as AsRef<str>>::as_ref(self).hash()
    }
}

impl Hashable for Uuid {
    fn hash(&self) -> [u8; 32] {
        use tiny_keccak::{Hasher, Sha3};

        let mut output = [0u8; 32];
        let mut hasher = Sha3::v256();
        hasher.update(self.as_bytes());
        hasher.finalize(&mut output);
        output
    }
}

impl Hashable for DateTime<Utc> {
    fn hash(&self) -> [u8; 32] {
        use tiny_keccak::{Hasher, Sha3};

        let mut val_output = [0u8; 32];
        let mut val_hasher = Sha3::v256();
        val_hasher.update(self.to_rfc3339().as_bytes());
        val_hasher.finalize(&mut val_output);

        let mut output = [0u8; 32];
        let mut hasher = Sha3::v256();
        hasher.update(&hash::prefix("datetime"));
        hasher.update(&val_output);
        hasher.finalize(&mut output);
        output
    }
}

impl Hashable for Map<String, Value> {
    fn hash(&self) -> [u8; 32] {
        self.iter().collect::<BTreeMap<_, _>>().hash()
    }
}

impl Hashable for Value {
    fn hash(&self) -> [u8; 32] {
        match self {
            Value::Null => [0u8; 32],
            Value::Bool(val) => val.hash(),
            Value::Number(val) => {
                if let Some(val) = val.as_u64() {
                    val.hash()
                } else if let Some(val) = val.as_i64() {
                    val.hash()
                } else if let Some(val) = val.as_f64() {
                    val.hash()
                } else {
                    panic!("unsupported number type")
                }
            }
            Value::String(val) => val.hash(),
            Value::Array(val) => val.hash(),
            Value::Object(val) => val.hash(),
        }
    }
}
