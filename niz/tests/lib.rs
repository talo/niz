use niz::{
    tiny_keccak::{Hasher, Sha3},
    *,
};

#[derive(Hashable)]
struct TestStruct {
    a: u32,
    b: u32,
    #[allow(dead_code)]
    #[niz(skip)]
    c: String,
    #[niz(json)]
    j: String,
}

#[derive(Copy, Clone, Hashable)]
enum TestEnum {
    A,
    B,
}

#[derive(Copy, Clone, Hashable)]
enum TestEnumDiscriminant {
    A = 2,
    B = 5,
}

#[test]
fn test_struct() {
    let test = TestStruct {
        a: 1,
        b: 2,
        c: "c".to_string(),
        j: "json_string".to_string(),
    };
    let actual = test.hash();

    let mut expected = [0u8; 32];
    let mut hasher = Sha3::v256();
    hasher.update(&hash::prefix("TestStruct"));

    let mut field_output = [0u8; 32];
    let mut field_hasher = Sha3::v256();
    field_hasher.update(&hash::prefix("a"));
    field_hasher.update(&1u32.hash());
    field_hasher.finalize(&mut field_output);
    hasher.update(&field_output);

    let mut field_output = [0u8; 32];
    let mut field_hasher = Sha3::v256();
    field_hasher.update(&hash::prefix("b"));
    field_hasher.update(&2u32.hash());
    field_hasher.finalize(&mut field_output);
    hasher.update(&field_output);

    let mut field_output = [0u8; 32];
    let mut field_hasher = Sha3::v256();
    field_hasher.update(&hash::prefix("j"));
    field_hasher.update(&"json_string".hash());
    field_hasher.finalize(&mut field_output);
    hasher.update(&field_output);

    hasher.finalize(&mut expected);
    assert_eq!(actual, expected);
}

#[test]
fn test_enum() {
    let test = TestEnum::A;
    let actual = test.hash();

    let mut expected = [0u8; 32];
    let mut hasher = Sha3::v256();
    hasher.update(&hash::prefix("TestEnum"));

    let mut variant_output = [0u8; 32];
    let mut variant_hasher = Sha3::v256();
    variant_hasher.update(&hash::prefix("A"));
    variant_hasher.update(&0u8.hash());
    variant_hasher.finalize(&mut variant_output);
    hasher.update(&variant_output);

    hasher.finalize(&mut expected);
    assert_eq!(actual, expected);

    let test = TestEnum::B;
    let actual = test.hash();

    let mut expected = [0u8; 32];
    let mut hasher = Sha3::v256();
    hasher.update(&hash::prefix("TestEnum"));

    let mut variant_output = [0u8; 32];
    let mut variant_hasher = Sha3::v256();
    variant_hasher.update(&hash::prefix("B"));
    variant_hasher.update(&1u8.hash());
    variant_hasher.finalize(&mut variant_output);
    hasher.update(&variant_output);

    hasher.finalize(&mut expected);
    assert_eq!(actual, expected);
}

#[test]
fn test_enum_discriminant() {
    let test = TestEnumDiscriminant::A;
    let actual = test.hash();

    let mut expected = [0u8; 32];
    let mut hasher = Sha3::v256();
    hasher.update(&hash::prefix("TestEnumDiscriminant"));

    let mut variant_output = [0u8; 32];
    let mut variant_hasher = Sha3::v256();
    variant_hasher.update(&hash::prefix("A"));
    variant_hasher.update(&2u32.hash());
    variant_hasher.finalize(&mut variant_output);
    hasher.update(&variant_output);

    hasher.finalize(&mut expected);
    assert_eq!(actual, expected);

    let test = TestEnumDiscriminant::B;
    let actual = test.hash();

    let mut expected = [0u8; 32];
    let mut hasher = Sha3::v256();
    hasher.update(&hash::prefix("TestEnumDiscriminant"));

    let mut variant_output = [0u8; 32];
    let mut variant_hasher = Sha3::v256();
    variant_hasher.update(&hash::prefix("B"));
    variant_hasher.update(&5u32.hash());
    variant_hasher.finalize(&mut variant_output);
    hasher.update(&variant_output);

    hasher.finalize(&mut expected);
    assert_eq!(actual, expected);
}
