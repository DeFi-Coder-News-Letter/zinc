//!
//! The integer constant element tests.
//!

#![cfg(test)]

use std::str::FromStr;

use num_bigint::BigInt;

use crate::error::Error;
use crate::lexical::Location;
use crate::semantic::element::constant::error::Error as ConstantError;
use crate::semantic::element::constant::integer::error::Error as IntegerConstantError;
use crate::semantic::element::constant::integer::Integer;
use crate::semantic::element::error::Error as ElementError;
use crate::semantic::element::r#type::Type;
use crate::semantic::Error as SemanticError;

#[test]
fn minimal_bitlength() {
    assert_eq!(
        Integer::minimal_bitlength(&BigInt::from_str("0").unwrap_or_default(), false),
        Ok(crate::BITLENGTH_BYTE * 1),
    );
    assert_eq!(
        Integer::minimal_bitlength(&BigInt::from_str("255").unwrap_or_default(), false),
        Ok(crate::BITLENGTH_BYTE * 1),
    );
    assert_eq!(
        Integer::minimal_bitlength(&BigInt::from_str("256").unwrap_or_default(), false),
        Ok(crate::BITLENGTH_BYTE * 2),
    );
    assert_eq!(
        Integer::minimal_bitlength(&BigInt::from_str("65535").unwrap_or_default(), false),
        Ok(crate::BITLENGTH_BYTE * 2),
    );
    assert_eq!(
        Integer::minimal_bitlength(&BigInt::from_str("65536").unwrap_or_default(), false),
        Ok(crate::BITLENGTH_BYTE * 3),
    );
    assert_eq!(
        Integer::minimal_bitlength(&BigInt::from_str("4294967295").unwrap_or_default(), false),
        Ok(crate::BITLENGTH_BYTE * 4),
    );
    assert_eq!(
        Integer::minimal_bitlength(&BigInt::from_str("4294967296").unwrap_or_default(), false),
        Ok(crate::BITLENGTH_BYTE * 5),
    );
    assert_eq!(
        Integer::minimal_bitlength(
            &BigInt::from_str("18446744073709551615").unwrap_or_default(),
            false
        ),
        Ok(crate::BITLENGTH_BYTE * 8),
    );
    assert_eq!(
        Integer::minimal_bitlength(
            &BigInt::from_str("18446744073709551616").unwrap_or_default(),
            false
        ),
        Ok(crate::BITLENGTH_BYTE * 9),
    );
    assert_eq!(
        Integer::minimal_bitlength(&BigInt::from_str("-128").unwrap_or_default(), true),
        Ok(crate::BITLENGTH_BYTE * 1),
    );
    assert_eq!(
        Integer::minimal_bitlength(&BigInt::from_str("127").unwrap_or_default(), true),
        Ok(crate::BITLENGTH_BYTE * 1),
    );
    assert_eq!(
        Integer::minimal_bitlength(&BigInt::from_str("128").unwrap_or_default(), true),
        Ok(crate::BITLENGTH_BYTE * 2),
    );
    assert_eq!(
        Integer::minimal_bitlength(&BigInt::from_str("32767").unwrap_or_default(), true),
        Ok(crate::BITLENGTH_BYTE * 2),
    );
    assert_eq!(
        Integer::minimal_bitlength(&BigInt::from_str("32768").unwrap_or_default(), true),
        Ok(crate::BITLENGTH_BYTE * 3),
    );
    assert_eq!(
        Integer::minimal_bitlength(&BigInt::from_str("2147483647").unwrap_or_default(), true),
        Ok(crate::BITLENGTH_BYTE * 4),
    );
    assert_eq!(
        Integer::minimal_bitlength(&BigInt::from_str("2147483648").unwrap_or_default(), true),
        Ok(crate::BITLENGTH_BYTE * 5),
    );
    assert_eq!(
        Integer::minimal_bitlength(
            &BigInt::from_str("9223372036854775807").unwrap_or_default(),
            true
        ),
        Ok(crate::BITLENGTH_BYTE * 8),
    );
    assert_eq!(
        Integer::minimal_bitlength(
            &BigInt::from_str("9223372036854775808").unwrap_or_default(),
            true
        ),
        Ok(crate::BITLENGTH_BYTE * 9),
    );
}

#[test]
fn error_element_constant_integer_inference_constant() {
    let input = r#"
fn main() {
    let invalid = 0xffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 19),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::IntegerTooLarge {
                value: BigInt::from_str("115792089237316195423570985008687907853269984665640564039457584007913129639935").expect(crate::semantic::tests::PANIC_TEST_DATA),
                bitlength: crate::BITLENGTH_FIELD,
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_integer_inference_constant_loop_bounds() {
    let input = r#"
fn main() {
    for i in 0..0xffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff {}
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 17),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::IntegerTooLarge {
                value: BigInt::from_str("115792089237316195423570985008687907853269984665640564039457584007913129639935").expect(crate::semantic::tests::PANIC_TEST_DATA),
                bitlength: crate::BITLENGTH_FIELD,
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_integer_inference_constant_pattern_match() {
    let input = r#"
fn main() {
    let scrutinee = 42;
    let result = match scrutinee {
        0xffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff => 10,
        2 => 20,
        _ => 30,
    };
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 9),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::IntegerTooLarge {
                value: BigInt::from_str("115792089237316195423570985008687907853269984665640564039457584007913129639935").expect(crate::semantic::tests::PANIC_TEST_DATA),
                bitlength: crate::BITLENGTH_FIELD,
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_integer_types_mismatch_greater_equals() {
    let input = r#"
fn main() {
    let value = 42 as u64 >= 69 as u128;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 27),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchGreaterEquals(
                Type::integer_unsigned(crate::BITLENGTH_BYTE * 8).to_string(),
                Type::integer_unsigned(crate::BITLENGTH_BYTE * 16).to_string(),
            ),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_integer_types_mismatch_greater_equals_enumeration() {
    let input = r#"
enum Default {
    Value = 42,
}

fn main() {
    let value = Default::Value >= 69;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(7, 32),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchGreaterEquals(
                "enum Default".to_owned(),
                Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
            ),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_integer_types_mismatch_greater_equals_two_enumerations() {
    let input = r#"
enum One {
    Value = 42,
}

enum Two {
    Value = 69,
}

fn main() {
    let value = One::Value >= Two::Value;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(11, 28),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchGreaterEquals(
                "enum One".to_owned(),
                "enum Two".to_owned(),
            ),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_integer_types_mismatch_lesser_equals() {
    let input = r#"
fn main() {
    let value = 42 as u64 <= 69 as u128;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 27),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchLesserEquals(
                Type::integer_unsigned(crate::BITLENGTH_BYTE * 8).to_string(),
                Type::integer_unsigned(crate::BITLENGTH_BYTE * 16).to_string(),
            ),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_integer_types_mismatch_lesser_equals_enumeration() {
    let input = r#"
enum Default {
    Value = 42,
}

fn main() {
    let value = Default::Value <= 69;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(7, 32),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchLesserEquals(
                "enum Default".to_owned(),
                Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
            ),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_integer_types_mismatch_lesser_equals_two_enumerations() {
    let input = r#"
enum One {
    Value = 42,
}

enum Two {
    Value = 69,
}

fn main() {
    let value = One::Value <= Two::Value;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(11, 28),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchLesserEquals(
                "enum One".to_owned(),
                "enum Two".to_owned(),
            ),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_integer_types_mismatch_greater() {
    let input = r#"
fn main() {
    let value = 42 as u64 > 69 as u128;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 27),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchGreater(
                Type::integer_unsigned(crate::BITLENGTH_BYTE * 8).to_string(),
                Type::integer_unsigned(crate::BITLENGTH_BYTE * 16).to_string(),
            ),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_integer_types_mismatch_greater_enumeration() {
    let input = r#"
enum Default {
    Value = 42,
}

fn main() {
    let value = Default::Value > 69;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(7, 32),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchGreater(
                "enum Default".to_owned(),
                Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
            ),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_integer_types_mismatch_greater_two_enumerations() {
    let input = r#"
enum One {
    Value = 42,
}

enum Two {
    Value = 69,
}

fn main() {
    let value = One::Value > Two::Value;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(11, 28),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchGreater(
                "enum One".to_owned(),
                "enum Two".to_owned(),
            ),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_integer_types_mismatch_lesser() {
    let input = r#"
fn main() {
    let value = 42 as u64 < 69 as u128;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 27),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchLesser(
                Type::integer_unsigned(crate::BITLENGTH_BYTE * 8).to_string(),
                Type::integer_unsigned(crate::BITLENGTH_BYTE * 16).to_string(),
            ),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_integer_types_mismatch_lesser_enumeration() {
    let input = r#"
enum Default {
    Value = 42,
}

fn main() {
    let value = Default::Value < 69;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(7, 32),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchLesser(
                "enum Default".to_owned(),
                Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
            ),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_integer_types_mismatch_lesser_two_enumerations() {
    let input = r#"
enum One {
    Value = 42,
}

enum Two {
    Value = 69,
}

fn main() {
    let value = One::Value < Two::Value;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(11, 28),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchLesser("enum One".to_owned(), "enum Two".to_owned()),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_integer_types_mismatch_addition() {
    let input = r#"
fn main() {
    let value = 42 as u64 + 69 as u128;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 27),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchAddition(
                Type::integer_unsigned(crate::BITLENGTH_BYTE * 8).to_string(),
                Type::integer_unsigned(crate::BITLENGTH_BYTE * 16).to_string(),
            ),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_integer_types_mismatch_addition_enumeration() {
    let input = r#"
enum Default {
    Value = 42,
}

fn main() {
    let value = Default::Value + 69;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(7, 32),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchAddition(
                "enum Default".to_owned(),
                Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
            ),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_integer_types_mismatch_addition_two_enumerations() {
    let input = r#"
enum One {
    Value = 42,
}

enum Two {
    Value = 69,
}

fn main() {
    let value = One::Value + Two::Value;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(11, 28),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchAddition(
                "enum One".to_owned(),
                "enum Two".to_owned(),
            ),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_integer_types_mismatch_subtraction() {
    let input = r#"
fn main() {
    let value = 42 as u64 - 69 as u128;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 27),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchSubtraction(
                Type::integer_unsigned(crate::BITLENGTH_BYTE * 8).to_string(),
                Type::integer_unsigned(crate::BITLENGTH_BYTE * 16).to_string(),
            ),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_integer_types_mismatch_subtraction_enumeration() {
    let input = r#"
enum Default {
    Value = 42,
}

fn main() {
    let value = Default::Value - 69;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(7, 32),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchSubtraction(
                "enum Default".to_owned(),
                Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
            ),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_integer_types_mismatch_subtraction_two_enumerations() {
    let input = r#"
enum One {
    Value = 42,
}

enum Two {
    Value = 69,
}

fn main() {
    let value = One::Value - Two::Value;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(11, 28),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchSubtraction(
                "enum One".to_owned(),
                "enum Two".to_owned(),
            ),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_integer_types_mismatch_multiplication() {
    let input = r#"
fn main() {
    let value = 42 as u64 * 69 as u128;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 27),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchMultiplication(
                Type::integer_unsigned(crate::BITLENGTH_BYTE * 8).to_string(),
                Type::integer_unsigned(crate::BITLENGTH_BYTE * 16).to_string(),
            ),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_integer_types_mismatch_multiplication_enumeration() {
    let input = r#"
enum Default {
    Value = 42,
}

fn main() {
    let value = Default::Value * 69;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(7, 32),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchMultiplication(
                "enum Default".to_owned(),
                Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
            ),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_integer_types_mismatch_multiplication_two_enumerations() {
    let input = r#"
enum One {
    Value = 42,
}

enum Two {
    Value = 69,
}

fn main() {
    let value = One::Value * Two::Value;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(11, 28),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchMultiplication(
                "enum One".to_owned(),
                "enum Two".to_owned(),
            ),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_integer_types_mismatch_division() {
    let input = r#"
fn main() {
    let value = 42 as u64 / 69 as u128;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 27),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchDivision(
                Type::integer_unsigned(crate::BITLENGTH_BYTE * 8).to_string(),
                Type::integer_unsigned(crate::BITLENGTH_BYTE * 16).to_string(),
            ),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_integer_types_mismatch_division_enumeration() {
    let input = r#"
enum Default {
    Value = 42,
}

fn main() {
    let value = Default::Value / 69;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(7, 32),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchDivision(
                "enum Default".to_owned(),
                Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
            ),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_integer_types_mismatch_division_two_enumerations() {
    let input = r#"
enum One {
    Value = 42,
}

enum Two {
    Value = 69,
}

fn main() {
    let value = One::Value / Two::Value;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(11, 28),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchDivision(
                "enum One".to_owned(),
                "enum Two".to_owned(),
            ),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_integer_types_mismatch_remainder() {
    let input = r#"
fn main() {
    let value = 42 as u64 % 69 as u128;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 27),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchRemainder(
                Type::integer_unsigned(crate::BITLENGTH_BYTE * 8).to_string(),
                Type::integer_unsigned(crate::BITLENGTH_BYTE * 16).to_string(),
            ),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_integer_types_mismatch_remainder_enumeration() {
    let input = r#"
enum Default {
    Value = 42,
}

fn main() {
    let value = Default::Value % 69;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(7, 32),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchRemainder(
                "enum Default".to_owned(),
                Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
            ),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_integer_types_mismatch_remainder_two_enumerations() {
    let input = r#"
enum One {
    Value = 42,
}

enum Two {
    Value = 69,
}

fn main() {
    let value = One::Value % Two::Value;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(11, 28),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchRemainder(
                "enum One".to_owned(),
                "enum Two".to_owned(),
            ),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_integer_overflow_addition_signed_negative() {
    let input = r#"
fn main() {
    let value = -120 + (-50);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 22),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::OverflowAddition {
                value: BigInt::from(-170),
                r#type: Type::integer(true, crate::BITLENGTH_BYTE).to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_integer_overflow_addition_signed_positive() {
    let input = r#"
fn main() {
    let value = 42 as i8 + 100 as i8;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 26),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::OverflowAddition {
                value: BigInt::from(142),
                r#type: Type::integer(true, crate::BITLENGTH_BYTE).to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_integer_overflow_addition_unsigned_positive() {
    let input = r#"
fn main() {
    let value = 42 + 255;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 20),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::OverflowAddition {
                value: BigInt::from(297),
                r#type: Type::integer(false, crate::BITLENGTH_BYTE).to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_integer_overflow_subtraction_signed_negative() {
    let input = r#"
fn main() {
    let value = -42 - 100 as i8;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 21),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::OverflowSubtraction {
                value: BigInt::from(-142),
                r#type: Type::integer(true, crate::BITLENGTH_BYTE).to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_integer_overflow_subtraction_signed_positive() {
    let input = r#"
fn main() {
    let value = (50 as i8) - (-100);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 28),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::OverflowSubtraction {
                value: BigInt::from(150),
                r#type: Type::integer(true, crate::BITLENGTH_BYTE).to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_integer_overflow_subtraction_unsigned_negative() {
    let input = r#"
fn main() {
    let value = 42 - 255;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 20),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::OverflowSubtraction {
                value: BigInt::from(-213),
                r#type: Type::integer(false, crate::BITLENGTH_BYTE).to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_integer_overflow_multiplication_signed_negative() {
    let input = r#"
fn main() {
    let value = -100 * (2 as i8);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 22),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::OverflowMultiplication {
                value: BigInt::from(-200),
                r#type: Type::integer(true, crate::BITLENGTH_BYTE).to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_integer_overflow_multiplication_signed_positive() {
    let input = r#"
fn main() {
    let value = 100 as i8 * 2 as i8;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 27),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::OverflowMultiplication {
                value: BigInt::from(200),
                r#type: Type::integer(true, crate::BITLENGTH_BYTE).to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_integer_overflow_multiplication_unsigned_positive() {
    let input = r#"
fn main() {
    let value = 42 * 10;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 20),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::OverflowMultiplication {
                value: BigInt::from(420),
                r#type: Type::integer(false, crate::BITLENGTH_BYTE).to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_integer_overflow_division_signed_positive() {
    let input = r#"
fn main() {
    let value = -128 / (-1);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 22),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::OverflowDivision {
                value: BigInt::from(128),
                r#type: Type::integer(true, crate::BITLENGTH_BYTE).to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_integer_overflow_casting_signed_positive() {
    let input = r#"
fn main() {
    let value = 200 as i8;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 21),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::OverflowCasting {
                value: BigInt::from(200),
                r#type: Type::integer(true, crate::BITLENGTH_BYTE).to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_integer_overflow_casting_unsigned_negative() {
    let input = r#"
fn main() {
    let value = (-100 as u8);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 23),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::OverflowCasting {
                value: BigInt::from(-100),
                r#type: Type::integer(false, crate::BITLENGTH_BYTE).to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_integer_overflow_negation_signed_positive() {
    let input = r#"
fn main() {
    let value = --128;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 17),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::OverflowNegation {
                value: BigInt::from(128),
                r#type: Type::integer(true, crate::BITLENGTH_BYTE).to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_integer_overflow_negation_unsigned_negative() {
    let input = r#"
fn main() {
    let value = -200;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 17),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::OverflowNegation {
                value: BigInt::from(-200),
                r#type: Type::integer(true, crate::BITLENGTH_BYTE).to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_integer_zero_division() {
    let input = r#"
fn main() {
    let value = 42 / 0;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 20),
        ElementError::Constant(ConstantError::Integer(IntegerConstantError::ZeroDivision)),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_integer_zero_remainder() {
    let input = r#"
fn main() {
    let value = 42 % 0;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 20),
        ElementError::Constant(ConstantError::Integer(IntegerConstantError::ZeroRemainder)),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}
