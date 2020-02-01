//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use num_bigint::BigInt;

use zinc_bytecode::Add;
use zinc_bytecode::Call;
use zinc_bytecode::Cast;
use zinc_bytecode::Exit;
use zinc_bytecode::Instruction;
use zinc_bytecode::Load;
use zinc_bytecode::LoadGlobal;
use zinc_bytecode::PushConst;
use zinc_bytecode::Return;
use zinc_bytecode::Store;
use zinc_bytecode::StoreGlobal;

#[test]
fn test() {
    let input = r#"
static STATIC: field = 5;

const CONST: field = 42;

fn main(input: (), witness: ()) -> field {
    let var: field = 69;

    STATIC + CONST + var
}
"#;

    let expected = Ok(vec![
        Instruction::Call(Call::new(5, 0)),
        Instruction::Exit(Exit::new(1)),
        Instruction::PushConst(PushConst::new(
            BigInt::from(5),
            false,
            crate::BITLENGTH_BYTE,
        )),
        Instruction::Cast(Cast::new(false, crate::BITLENGTH_FIELD)),
        Instruction::StoreGlobal(StoreGlobal::new(0)),
        Instruction::PushConst(PushConst::new(
            BigInt::from(69),
            false,
            crate::BITLENGTH_BYTE,
        )),
        Instruction::Cast(Cast::new(false, crate::BITLENGTH_FIELD)),
        Instruction::Store(Store::new(0)),
        Instruction::LoadGlobal(LoadGlobal::new(0)),
        Instruction::PushConst(PushConst::new(
            BigInt::from(42),
            false,
            crate::BITLENGTH_FIELD,
        )),
        Instruction::Add(Add),
        Instruction::Load(Load::new(0)),
        Instruction::Add(Add),
        Instruction::Return(Return::new(1)),
    ]);

    let result = super::get_instructions(input);

    assert_eq!(expected, result);
}