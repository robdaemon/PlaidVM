use vm;

#[allow(dead_code)]
#[allow(unused_variables)]
struct CompiledMethod {
    header_index: vm::SmallInteger,
    literal_start: vm::SmallInteger,
}

const no_primitive_zero_arguments: u16 = 0;
const no_primitive_one_arguments: u16 = 1;
const no_primitive_two_arguments: u16 = 2;
const no_primitive_three_arguments: u16 = 3;
const no_primitive_four_arguments: u16 = 4;
const primitive_return_of_self: u16 = 5;
const primitive_return_of_instance_variable: u16 = 6;
const header_extension_with_arguments_and_primitive_index: u16 = 7;

#[allow(dead_code)]
#[allow(unused_variables)]
impl CompiledMethod {
    pub fn new() -> CompiledMethod {
        CompiledMethod {
            header_index: vm::SmallInteger::new(0),
            literal_start: vm::SmallInteger::new(1),
        }
    }

    pub fn header_of(&self, method: vm::MethodPointer) -> u16 {
        0
    }

    pub fn literal(&self, offset: u16, method: vm::MethodPointer) -> u16 {
        0
    }

    pub fn temporary_count_of(&self, method: vm::MethodPointer) -> u16 {
        0
    }

    pub fn large_context_flag_of(&self, method: vm::MethodPointer) -> u16 {
        0
    }

    pub fn literal_count_of(&self, method: vm::MethodPointer) -> u16 {
        0
    }

    pub fn literal_count_of_header(&self, header: u16) -> u16 {
        0
    }

    pub fn object_pointer_count_of(&self, method_pointer: u16) -> u16 {
        0
    }

    pub fn initial_instruction_pointer_of_method(&self, method_pointer: u16) -> u16 {
        0
    }

    pub fn flag_value_of(&self, method_pointer: u16) -> u16 {
        0
    }
}