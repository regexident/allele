use super::*;

#[test]
fn abs_diff_of_signed_1_and_0() {
    assert_eq!(Fitness::abs_diff(&1i8, &0i8), 1i8);
    assert_eq!(Fitness::abs_diff(&1i16, &0i16), 1i16);
    assert_eq!(Fitness::abs_diff(&1i32, &0i32), 1i32);
    assert_eq!(Fitness::abs_diff(&1i64, &0i64), 1i64);
    assert_eq!(Fitness::abs_diff(&1isize, &0isize), 1isize);
}

#[test]
fn abs_diff_of_signed_0_and_1() {
    assert_eq!(Fitness::abs_diff(&0i8, &1i8), 1i8);
    assert_eq!(Fitness::abs_diff(&0i16, &1i16), 1i16);
    assert_eq!(Fitness::abs_diff(&0i32, &1i32), 1i32);
    assert_eq!(Fitness::abs_diff(&0i64, &1i64), 1i64);
    assert_eq!(Fitness::abs_diff(&0isize, &1isize), 1isize);
}

#[test]
fn abs_diff_of_signed_0_and_0() {
    assert_eq!(Fitness::abs_diff(&0i8, &0i8), 0i8);
    assert_eq!(Fitness::abs_diff(&0i16, &0i16), 0i16);
    assert_eq!(Fitness::abs_diff(&0i32, &0i32), 0i32);
    assert_eq!(Fitness::abs_diff(&0i64, &0i64), 0i64);
    assert_eq!(Fitness::abs_diff(&0isize, &0isize), 0isize);
}

#[test]
fn abs_diff_of_signed_neg1_and_0() {
    assert_eq!(Fitness::abs_diff(&-1i8, &0i8), 1i8);
    assert_eq!(Fitness::abs_diff(&-1i16, &0i16), 1i16);
    assert_eq!(Fitness::abs_diff(&-1i32, &0i32), 1i32);
    assert_eq!(Fitness::abs_diff(&-1i64, &0i64), 1i64);
    assert_eq!(Fitness::abs_diff(&-1isize, &0isize), 1isize);
}

#[test]
fn abs_diff_of_signed_0_and_neg1() {
    assert_eq!(Fitness::abs_diff(&0i8, &-1i8), 1i8);
    assert_eq!(Fitness::abs_diff(&0i16, &-1i16), 1i16);
    assert_eq!(Fitness::abs_diff(&0i32, &-1i32), 1i32);
    assert_eq!(Fitness::abs_diff(&0i64, &-1i64), 1i64);
    assert_eq!(Fitness::abs_diff(&0isize, &-1isize), 1isize);
}

#[test]
fn abs_diff_of_signed_neg1_and_1() {
    assert_eq!(Fitness::abs_diff(&-1i8, &1i8), 2i8);
    assert_eq!(Fitness::abs_diff(&-1i16, &1i16), 2i16);
    assert_eq!(Fitness::abs_diff(&-1i32, &1i32), 2i32);
    assert_eq!(Fitness::abs_diff(&-1i64, &1i64), 2i64);
    assert_eq!(Fitness::abs_diff(&-1isize, &1isize), 2isize);
}

#[test]
fn abs_diff_of_signed_1_and_neg1() {
    assert_eq!(Fitness::abs_diff(&1i8, &-1i8), 2i8);
    assert_eq!(Fitness::abs_diff(&1i16, &-1i16), 2i16);
    assert_eq!(Fitness::abs_diff(&1i32, &-1i32), 2i32);
    assert_eq!(Fitness::abs_diff(&1i64, &-1i64), 2i64);
    assert_eq!(Fitness::abs_diff(&1isize, &-1isize), 2isize);
}

#[test]
fn abs_diff_of_signed_neg19_and_23() {
    assert_eq!(Fitness::abs_diff(&-19i8, &23i8), 42i8);
    assert_eq!(Fitness::abs_diff(&-19i16, &23i16), 42i16);
    assert_eq!(Fitness::abs_diff(&-19i32, &23i32), 42i32);
    assert_eq!(Fitness::abs_diff(&-19i64, &23i64), 42i64);
    assert_eq!(Fitness::abs_diff(&-19isize, &23isize), 42isize);
}

#[test]
fn abs_diff_of_signed_19_and_neg23() {
    assert_eq!(Fitness::abs_diff(&19i8, &-23i8), 42i8);
    assert_eq!(Fitness::abs_diff(&19i16, &-23i16), 42i16);
    assert_eq!(Fitness::abs_diff(&19i32, &-23i32), 42i32);
    assert_eq!(Fitness::abs_diff(&19i64, &-23i64), 42i64);
    assert_eq!(Fitness::abs_diff(&19isize, &-23isize), 42isize);
}

#[test]
fn abs_diff_of_signed_61_and_19() {
    assert_eq!(Fitness::abs_diff(&61i8, &19i8), 42i8);
    assert_eq!(Fitness::abs_diff(&61i16, &19i16), 42i16);
    assert_eq!(Fitness::abs_diff(&61i32, &19i32), 42i32);
    assert_eq!(Fitness::abs_diff(&61i64, &19i64), 42i64);
    assert_eq!(Fitness::abs_diff(&61isize, &19isize), 42isize);
}

#[test]
fn abs_diff_of_signed_19_and_61() {
    assert_eq!(Fitness::abs_diff(&19i8, &61i8), 42i8);
    assert_eq!(Fitness::abs_diff(&19i16, &61i16), 42i16);
    assert_eq!(Fitness::abs_diff(&19i32, &61i32), 42i32);
    assert_eq!(Fitness::abs_diff(&19i64, &61i64), 42i64);
    assert_eq!(Fitness::abs_diff(&19isize, &61isize), 42isize);
}

#[test]
fn abs_diff_of_signed_max_and_1() {
    assert_eq!(Fitness::abs_diff(&i8::MAX, &1i8), i8::MAX - 1);
    assert_eq!(Fitness::abs_diff(&i16::MAX, &1i16), i16::MAX - 1);
    assert_eq!(Fitness::abs_diff(&i32::MAX, &1i32), i32::MAX - 1);
    assert_eq!(Fitness::abs_diff(&i64::MAX, &1i64), i64::MAX - 1);
    assert_eq!(Fitness::abs_diff(&isize::MAX, &1isize), isize::MAX - 1);
}

#[test]
fn abs_diff_of_signed_1_and_max() {
    assert_eq!(Fitness::abs_diff(&1i8, &i8::MAX), i8::MAX - 1);
    assert_eq!(Fitness::abs_diff(&1i16, &i16::MAX), i16::MAX - 1);
    assert_eq!(Fitness::abs_diff(&1i32, &i32::MAX), i32::MAX - 1);
    assert_eq!(Fitness::abs_diff(&1i64, &i64::MAX), i64::MAX - 1);
    assert_eq!(Fitness::abs_diff(&1isize, &isize::MAX), isize::MAX - 1);
}

#[test]
fn abs_diff_of_signed_max_and_neg1i8() {
    assert_eq!(Fitness::abs_diff(&i8::MAX, &-1i8), i8::MIN);
}

#[test]
fn abs_diff_of_signed_max_and_neg1i16() {
    assert_eq!(Fitness::abs_diff(&i16::MAX, &-1i16), i16::MIN);
}
#[test]
fn abs_diff_of_signed_max_and_neg1i32() {
    assert_eq!(Fitness::abs_diff(&i32::MAX, &-1i32), i32::MIN);
}
#[test]
fn abs_diff_of_signed_max_and_neg1i64() {
    assert_eq!(Fitness::abs_diff(&i64::MAX, &-1i64), i64::MIN);
}

#[test]
fn abs_diff_of_signed_neg1i8_and_max() {
    assert_eq!(Fitness::abs_diff(&-1i8, &i8::MAX), i8::MIN);
}

#[test]
fn abs_diff_of_signed_neg1i16_and_max() {
    assert_eq!(Fitness::abs_diff(&-1i16, &i16::MAX), i16::MIN);
}

#[test]
fn abs_diff_of_signed_neg1i32_and_max() {
    assert_eq!(Fitness::abs_diff(&-1i32, &i32::MAX), i32::MIN);
}

#[test]
fn abs_diff_of_signed_neg1i64_and_max() {
    assert_eq!(Fitness::abs_diff(&-1i64, &i64::MAX), i64::MIN);
}

#[test]
fn abs_diff_of_unsigned_1_and_0() {
    assert_eq!(Fitness::abs_diff(&1u8, &0u8), 1u8);
    assert_eq!(Fitness::abs_diff(&1u16, &0u16), 1u16);
    assert_eq!(Fitness::abs_diff(&1u32, &0u32), 1u32);
    assert_eq!(Fitness::abs_diff(&1u64, &0u64), 1u64);
    assert_eq!(Fitness::abs_diff(&1usize, &0usize), 1usize);
}

#[test]
fn abs_diff_of_unsigned_0_and_1() {
    assert_eq!(Fitness::abs_diff(&0u8, &1u8), 1u8);
    assert_eq!(Fitness::abs_diff(&0u16, &1u16), 1u16);
    assert_eq!(Fitness::abs_diff(&0u32, &1u32), 1u32);
    assert_eq!(Fitness::abs_diff(&0u64, &1u64), 1u64);
    assert_eq!(Fitness::abs_diff(&0usize, &1usize), 1usize);
}

#[test]
fn abs_diff_of_unsigned_0_and_0() {
    assert_eq!(Fitness::abs_diff(&0u8, &0u8), 0u8);
    assert_eq!(Fitness::abs_diff(&0u16, &0u16), 0u16);
    assert_eq!(Fitness::abs_diff(&0u32, &0u32), 0u32);
    assert_eq!(Fitness::abs_diff(&0u64, &0u64), 0u64);
    assert_eq!(Fitness::abs_diff(&0usize, &0usize), 0usize);
}

#[test]
fn abs_diff_of_unsigned_61_and_19() {
    assert_eq!(Fitness::abs_diff(&61u8, &19u8), 42u8);
    assert_eq!(Fitness::abs_diff(&61u16, &19u16), 42u16);
    assert_eq!(Fitness::abs_diff(&61u32, &19u32), 42u32);
    assert_eq!(Fitness::abs_diff(&61u64, &19u64), 42u64);
    assert_eq!(Fitness::abs_diff(&61usize, &19usize), 42usize);
}

#[test]
fn abs_diff_of_unsigned_19_and_61() {
    assert_eq!(Fitness::abs_diff(&19u8, &61u8), 42u8);
    assert_eq!(Fitness::abs_diff(&19u16, &61u16), 42u16);
    assert_eq!(Fitness::abs_diff(&19u32, &61u32), 42u32);
    assert_eq!(Fitness::abs_diff(&19u64, &61u64), 42u64);
    assert_eq!(Fitness::abs_diff(&19usize, &61usize), 42usize);
}

#[test]
fn abs_diff_of_unsigned_max_and_1() {
    assert_eq!(Fitness::abs_diff(&u8::MAX, &1u8), u8::MAX - 1);
    assert_eq!(Fitness::abs_diff(&u16::MAX, &1u16), u16::MAX - 1);
    assert_eq!(Fitness::abs_diff(&u32::MAX, &1u32), u32::MAX - 1);
    assert_eq!(Fitness::abs_diff(&u64::MAX, &1u64), u64::MAX - 1);
    assert_eq!(Fitness::abs_diff(&usize::MAX, &1usize), usize::MAX - 1);
}

#[test]
fn abs_diff_of_unsigned_1_and_max() {
    assert_eq!(Fitness::abs_diff(&1u8, &u8::MAX), u8::MAX - 1);
    assert_eq!(Fitness::abs_diff(&1u16, &u16::MAX), u16::MAX - 1);
    assert_eq!(Fitness::abs_diff(&1u32, &u32::MAX), u32::MAX - 1);
    assert_eq!(Fitness::abs_diff(&1u64, &u64::MAX), u64::MAX - 1);
    assert_eq!(Fitness::abs_diff(&1usize, &usize::MAX), usize::MAX - 1);
}

mod fitness_properties {
    use proptest::prelude::ProptestConfig;
    use test_strategy::proptest;

    use super::*;

    #[proptest(ProptestConfig { cases: 500, failure_persistence: None, ..ProptestConfig::default() })]
    fn unsigned_abs_diff_is_symmetric(a: u32, b: u32) {
        assert_eq!(Fitness::abs_diff(&a, &b), Fitness::abs_diff(&b, &a));
    }

    #[proptest(ProptestConfig { cases: 500, failure_persistence: None, ..ProptestConfig::default() })]
    fn unsigned_abs_diff_of_equal_values_is_zero(a: u32) {
        assert_eq!(Fitness::abs_diff(&a, &a), u32::zero());
    }

    #[proptest(ProptestConfig { cases: 500, failure_persistence: None, ..ProptestConfig::default() })]
    fn signed_abs_diff_is_symmetric(a: i32, b: i32) {
        assert_eq!(Fitness::abs_diff(&a, &b), Fitness::abs_diff(&b, &a));
    }

    #[proptest(ProptestConfig { cases: 500, failure_persistence: None, ..ProptestConfig::default() })]
    fn signed_abs_diff_of_equal_values_is_zero(a: i32) {
        assert_eq!(Fitness::abs_diff(&a, &a), i32::zero());
    }
}
