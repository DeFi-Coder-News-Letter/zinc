//!
//! The Zinc VM bytecode.
//!

use std::collections::HashMap;

use zinc_bytecode::data::types::DataType;
use zinc_bytecode::data::values::Value as TemplateValue;
use zinc_bytecode::Instruction;
use zinc_bytecode::Program;

use crate::generator::r#type::Type;
use crate::lexical::token::location::Location;

static PANIC_JSON_TEMPLATE_SERIALIZATION: &str =
    "JSON templates serialization must be always successful: ";

///
/// The Zinc VM bytecode, generated by the compiler target code generator.
///
#[derive(Debug, PartialEq)]
pub struct Bytecode {
    input_fields: Vec<(String, Type)>,
    output_type: Type,
    instructions: Vec<Instruction>,

    data_stack_pointer: usize,
    variable_addresses: HashMap<String, usize>,
    function_addresses: HashMap<usize, usize>,

    current_file: String,
    current_location: Location,
}

impl Default for Bytecode {
    fn default() -> Self {
        Self::new()
    }
}

impl Bytecode {
    const INSTRUCTION_VECTOR_INITIAL_SIZE: usize = 1024;
    const FUNCTION_ADDRESSES_HASHMAP_INITIAL_SIZE: usize = 16;
    const VARIABLE_ADDRESSES_HASHMAP_INITIAL_SIZE: usize = 16;

    pub fn new() -> Self {
        let mut instructions = Vec::with_capacity(Self::INSTRUCTION_VECTOR_INITIAL_SIZE);
        instructions.push(Instruction::NoOperation(zinc_bytecode::NoOperation));
        instructions.push(Instruction::NoOperation(zinc_bytecode::NoOperation));

        Self {
            input_fields: vec![],
            output_type: Type::structure(vec![]),
            instructions,

            data_stack_pointer: 0,
            variable_addresses: HashMap::with_capacity(
                Self::VARIABLE_ADDRESSES_HASHMAP_INITIAL_SIZE,
            ),
            function_addresses: HashMap::with_capacity(
                Self::FUNCTION_ADDRESSES_HASHMAP_INITIAL_SIZE,
            ),

            current_file: String::new(),
            current_location: Location::new_beginning(None),
        }
    }

    pub fn start_new_file(&mut self, name: &str) {
        self.current_file = name.to_owned();
    }

    pub fn start_function(&mut self, unique_id: usize, identifier: String) {
        let address = self.instructions.len();
        self.function_addresses.insert(unique_id, address);
        self.data_stack_pointer = 0;

        self.instructions.push(Instruction::FileMarker(
            zinc_bytecode::instructions::FileMarker::new(self.current_file.clone()),
        ));
        self.instructions.push(Instruction::FunctionMarker(
            zinc_bytecode::FunctionMarker::new(identifier),
        ));
    }

    pub fn start_main_function(
        &mut self,
        unique_id: usize,
        input_arguments: Vec<(String, Type)>,
        output_type: Option<Type>,
    ) {
        let input_size = input_arguments
            .iter()
            .map(|(_name, r#type)| r#type.size())
            .sum();
        let output_size = output_type
            .as_ref()
            .map(|r#type| r#type.size())
            .unwrap_or(0);

        self.input_fields = input_arguments;
        self.output_type = output_type.unwrap_or_else(|| Type::structure(vec![]));

        let address = self.instructions.len();
        self.function_addresses.insert(unique_id, address);
        self.instructions[0] = Instruction::Call(zinc_bytecode::Call::new(address, input_size));
        self.instructions[1] = Instruction::Exit(zinc_bytecode::Exit::new(output_size));
        self.data_stack_pointer = 0;

        self.instructions.push(Instruction::FileMarker(
            zinc_bytecode::instructions::FileMarker::new(self.current_file.clone()),
        ));
        self.instructions.push(Instruction::FunctionMarker(
            zinc_bytecode::FunctionMarker::new(
                crate::semantic::element::r#type::function::user::FUNCTION_MAIN_IDENTIFIER
                    .to_owned(),
            ),
        ));
    }

    pub fn declare_variable(&mut self, identifier: Option<String>, r#type: Type) -> usize {
        let start_address = self.data_stack_pointer;
        if let Some(identifier) = identifier {
            self.variable_addresses
                .insert(identifier, self.data_stack_pointer);
        }
        self.data_stack_pointer += r#type.size();
        start_address
    }

    pub fn push_instruction(&mut self, instruction: Instruction, location: Option<Location>) {
        if let Some(location) = location {
            if self.current_location != location {
                if self.current_location.line != location.line {
                    self.instructions.push(Instruction::LineMarker(
                        zinc_bytecode::LineMarker::new(location.line),
                    ));
                }
                if self.current_location.column != location.column {
                    self.instructions.push(Instruction::ColumnMarker(
                        zinc_bytecode::ColumnMarker::new(location.column),
                    ));
                }
                self.current_location = location;
            }
        }

        self.instructions.push(instruction)
    }

    pub fn get_function_address(&self, unique_id: usize) -> Option<usize> {
        self.function_addresses.get(&unique_id).copied()
    }

    pub fn get_variable_address(&self, name: &str) -> Option<usize> {
        self.variable_addresses.get(name).copied()
    }

    pub fn input_template_bytes(&self) -> Vec<u8> {
        let input_type = self.input_types_as_struct();
        let input_template_value = TemplateValue::default_from_type(&input_type);
        match serde_json::to_string_pretty(&input_template_value.to_json()) {
            Ok(json) => (json + "\n").into_bytes(),
            Err(error) => {
                panic!(PANIC_JSON_TEMPLATE_SERIALIZATION.to_owned() + error.to_string().as_str())
            }
        }
    }

    pub fn output_template_bytes(&self) -> Vec<u8> {
        let output_bytecode_type = self.output_type.to_owned().into();
        let output_value_template = TemplateValue::default_from_type(&output_bytecode_type);
        match serde_json::to_string_pretty(&output_value_template.to_json()) {
            Ok(json) => (json + "\n").into_bytes(),
            Err(error) => {
                panic!(PANIC_JSON_TEMPLATE_SERIALIZATION.to_owned() + error.to_string().as_str())
            }
        }
    }

    pub fn into_bytes(self) -> Vec<u8> {
        for (index, instruction) in self.instructions.iter().enumerate() {
            log::debug!("{:03} {:?}", index, instruction)
        }

        let program = Program::new(
            self.input_types_as_struct(),
            self.output_type.into(),
            self.instructions,
        );

        program.to_bytes()
    }

    fn input_types_as_struct(&self) -> DataType {
        DataType::Struct(
            self.input_fields
                .iter()
                .map(|(name, r#type)| (name.to_owned(), r#type.to_owned().into()))
                .collect(),
        )
    }
}

impl Into<Vec<Instruction>> for Bytecode {
    fn into(self) -> Vec<Instruction> {
        self.instructions
    }
}
