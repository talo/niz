use niz::{
    tiny_keccak::{Hasher, Sha3},
    *,
};

#[derive(Hashable)]
struct Test {
    a: u32,
    b: u32,
}

#[test]
fn test_struct_macro() {
    let test = Test { a: 1, b: 2 };
    let actual = test.hash();

    let mut expected = [0u8; 32];
    let mut hasher = Sha3::v256();
    hasher.update(&hash::prefix("Test"));
    {
        let mut field_output = [0u8; 32];
        let mut field_hasher = Sha3::v256();
        field_hasher.update(&hash::prefix("a"));
        field_hasher.update(&1u32.hash());
        field_hasher.finalize(&mut field_output);
        hasher.update(&field_output);
    }
    {
        let mut field_output = [0u8; 32];
        let mut field_hasher = Sha3::v256();
        field_hasher.update(&hash::prefix("b"));
        field_hasher.update(&2u32.hash());
        field_hasher.finalize(&mut field_output);
        hasher.update(&field_output);
    }
    hasher.finalize(&mut expected);

    assert_eq!(actual, expected);
}
