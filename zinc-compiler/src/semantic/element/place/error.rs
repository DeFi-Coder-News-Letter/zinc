//!
//! The semantic analyzer place error.
//!

use failure::Fail;

#[derive(Debug, Fail, PartialEq)]
pub enum Error {
    #[fail(
        display = "'[]' operator expected an array as the first operand, but got '{}'",
        _0
    )]
    OperatorIndexFirstOperandExpectedArray(String),
    #[fail(
        display = "'[]' operator expected an integer as the second operand, but got '{}'",
        _0
    )]
    OperatorIndexSecondOperandExpectedIntegerOrRange(String),
    #[fail(display = "left range bound {} cannot be negative", _0)]
    IndexSliceStartOutOfRange(String),
    #[fail(
        display = "right range bound {} is out of range of the array of size {}",
        _0, _1
    )]
    IndexSliceEndOutOfRange(String, String),
    #[fail(
        display = "right range bound {} is lesser than the left one {}",
        _0, _1
    )]
    IndexSliceEndLesserThanStart(String, String),

    #[fail(
        display = "'.' operator expected a tuple as the first operand, but got '{}'",
        _0
    )]
    OperatorFieldFirstOperandExpectedTuple(String),
    #[fail(
        display = "'.' operator expected a structure as the first operand, but got '{}'",
        _0
    )]
    OperatorFieldFirstOperandExpectedStructure(String),
    #[fail(display = "tuple field {} does not exist in '{}'", _0, _1)]
    FieldDoesNotExistInTuple(usize, String),
    #[fail(display = "structure field '{}' does not exist in '{}'", _0, _1)]
    FieldDoesNotExistInStructure(String, String),
}