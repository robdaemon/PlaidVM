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
        extract_bits(3, 7, self.header_of(method))
    }

    pub fn large_context_flag_of(&self, method: vm::MethodPointer) -> u16 {
        extract_bits(8, 8, method)
    }

    pub fn literal_count_of(&self, method: vm::MethodPointer) -> u16 {
        self.literal_count_of_header(self.header_of(method))
    }

    pub fn literal_count_of_header(&self, header: u16) -> u16 {
        extract_bits(9, 14, header)
    }

    pub fn object_pointer_count_of(&self, method_pointer: u16) -> u16 {
        self.literal_count_of(method_pointer) + self.literal_start
    }

    pub fn initial_instruction_pointer_of_method(&self, method_pointer: u16) -> u16 {
        (self.literal_count_of(method_pointer) + self.literal_start)) * 2 + 1
    }

    pub fn flag_value_of(&self, method_pointer: u16) -> u16 {
        extract_bits(0, 2, self.header_of(method_pointer))
    }

    pub fn field_index_of(&self, method_pointer: u16) -> u16 {
        extract_bits(3, 7, self.header_of(method_pointer))
    }

    pub fn header_extension_of(&self, method_pointer: u16) -> u16 {
        let literal_count = self.literal_count_of(method_pointer);

        self.literal(literal_count - 2, method_pointer)
    }

    pub fn argument_count_of(&self, method_pointer: u16) -> u16 {
        let flag_value = self.flag_value_of(method_pointer);

        if flag_value < 5 {
            flag_value
        } else if flag_value < 7 {
            10
        } else {
            extract_bits(2, 6, header_extension_of(method_pointer))
        }
    }

    pub fn primitive_index_of(&self, method_pointer: u16) -> u16 {
        let flag_value = self.flag_value_of(method_pointer);

        if flag_value == 7 {
            extract_bits(7, 14, method_pointer)
        } else {
            10
        }
    }
}

pub fn extract_bits(from: u16, to: u16, method_pointer: u16) -> u16 {
    0
}