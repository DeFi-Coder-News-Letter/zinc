//!
//! The semantic analyzer standard library `std::crypto::schnorr::Signature::verify` function element.
//!

use std::fmt;
use std::ops::Deref;

use zinc_bytecode::builtins::BuiltinIdentifier;

use crate::semantic::element::r#type::function::error::Error;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::Element;
use crate::semantic::scope::builtin::BuiltInItems;

#[derive(Debug, Clone)]
pub struct Function {
    builtin_identifier: BuiltinIdentifier,
    identifier: &'static str,
    return_type: Box<Type>,
}

impl Function {
    pub const ARGUMENT_INDEX_SIGNATURE: usize = 0;
    pub const ARGUMENT_INDEX_MESSAGE: usize = 1;
    pub const ARGUMENT_COUNT: usize = 2;

    pub fn new(builtin_identifier: BuiltinIdentifier) -> Self {
        Self {
            builtin_identifier,
            identifier: "verify",
            return_type: Box::new(Type::boolean()),
        }
    }

    pub fn identifier(&self) -> &'static str {
        self.identifier
    }

    pub fn builtin_identifier(&self) -> BuiltinIdentifier {
        self.builtin_identifier
    }

    pub fn call(self, actual_elements: Vec<Element>) -> Result<Type, Error> {
        let mut actual_params = Vec::with_capacity(actual_elements.len());
        for (index, element) in actual_elements.into_iter().enumerate() {
            let r#type = match element {
                Element::Value(value) => value.r#type(),
                Element::Constant(constant) => constant.r#type(),
                element => {
                    return Err(Error::argument_not_evaluable(
                        self.identifier.to_owned(),
                        index + 1,
                        element.to_string(),
                    ))
                }
            };
            actual_params.push(r#type);
        }

        match actual_params.get(Self::ARGUMENT_INDEX_SIGNATURE) {
            Some(Type::Structure(structure))
                if structure.unique_id == BuiltInItems::TYPE_ID_STD_CRYPTO_SCHNORR_SIGNATURE => {}
            Some(r#type) => {
                return Err(Error::argument_type(
                    self.identifier.to_owned(),
                    "signature".to_owned(),
                    Self::ARGUMENT_INDEX_SIGNATURE + 1,
                    "std::crypto::schnorr::Signature { r: std::crypto::ecc::Point, s: field, pk: std::crypto::ecc::Point }".to_owned(),
                    r#type.to_string(),
                ))
            }
            None => {
                return Err(Error::argument_count(
                    self.identifier.to_owned(),
                    Self::ARGUMENT_COUNT,
                    actual_params.len(),
                ))
            }
        }

        match actual_params.get(Self::ARGUMENT_INDEX_MESSAGE) {
            Some(Type::Array { r#type, size }) => match (r#type.deref(), *size) {
                (Type::Boolean, size)
                    if size % crate::BITLENGTH_BYTE == 0
                        && size > 0
                        && size <= crate::LIMIT_SCHNORR_MESSAGE_BITS => {}
                (r#type, size) => {
                    return Err(Error::argument_type(
                        self.identifier.to_owned(),
                        "message".to_owned(),
                        Self::ARGUMENT_INDEX_MESSAGE + 1,
                        format!(
                            "[bool; N], 0 < N <= {}, N % {} == 0",
                            crate::BITLENGTH_MAX_INT,
                            crate::BITLENGTH_BYTE
                        ),
                        format!("[{}; {}]", r#type, size),
                    ));
                }
            },
            Some(r#type) => {
                return Err(Error::argument_type(
                    self.identifier.to_owned(),
                    "message".to_owned(),
                    Self::ARGUMENT_INDEX_MESSAGE + 1,
                    format!(
                        "[bool; N], 0 < N <= {}, N % {} == 0",
                        crate::BITLENGTH_MAX_INT,
                        crate::BITLENGTH_BYTE
                    ),
                    r#type.to_string(),
                ));
            }
            None => {
                return Err(Error::argument_count(
                    self.identifier.to_owned(),
                    Self::ARGUMENT_COUNT,
                    actual_params.len(),
                ));
            }
        }

        if actual_params.len() > Self::ARGUMENT_COUNT {
            return Err(Error::argument_count(
                self.identifier.to_owned(),
                Self::ARGUMENT_COUNT,
                actual_params.len(),
            ));
        }

        Ok(*self.return_type)
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "fn std::crypto::schnorr::{}(signature: std::crypto::schnorr::Signature, message: [bool; N]) -> bool", self.identifier)
    }
}
