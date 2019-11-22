use crate::{InstructionInfo, InstructionCode, DecodingError};
use crate::instructions::utils::decode_simple_instruction;

#[derive(Debug,PartialEq,Default)]
pub struct Neg;

impl InstructionInfo for Neg {
    fn to_assembly(&self) -> String {
        "neg".into()
    }

    fn code() -> InstructionCode {
        InstructionCode::Neg
    }

    fn encode(&self) -> Vec<u8> {
        vec![InstructionCode::Neg as u8]
    }

    fn decode(bytes: &[u8]) -> Result<(Neg, usize), DecodingError> {
        decode_simple_instruction(bytes)
    }

    fn inputs_count(&self) -> usize {
        1
    }

    fn outputs_count(&self) -> usize {
        1
    }
}
