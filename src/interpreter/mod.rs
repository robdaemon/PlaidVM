// Copyright 2016 Robert Roland
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

use vm;

#[allow(dead_code)]
#[allow(unused_variables)]
struct Interpreter {
    // SmallIntegers
    minus_one_pointer: vm::SmallInteger,
    zero_pointer: vm::SmallInteger,
    one_pointer: vm::SmallInteger,
    two_pointer: vm::SmallInteger,

    // UndefinedObject and Booleans
    nil_pointer: vm::SmallInteger,
    false_pointer: vm::SmallInteger,
    true_pointer: vm::SmallInteger,

    // Root
    scheduler_association_pointer: vm::SmallInteger,

    // Classes
    class_string_pointer: vm::SmallInteger,
    class_array_pointer: vm::SmallInteger,
    class_method_context_pointer: vm::SmallInteger,
    class_block_context_pointer: vm::SmallInteger,
    class_point_pointer: vm::SmallInteger,
    class_large_positive_integer_pointer: vm::SmallInteger,
    class_message_pointer: vm::SmallInteger,
    class_character_pointer: vm::SmallInteger,

    // Selectors
    does_not_understand_selector: vm::SmallInteger,
    cannot_return_selector: vm::SmallInteger,
    must_be_boolean_selector: vm::SmallInteger,

    // Tables
    special_selectors_pointer: vm::SmallInteger,
    character_table_pointer: vm::SmallInteger,
}

#[allow(dead_code)]
#[allow(unused_variables)]
impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            // SmallIntegers
            minus_one_pointer: vm::SmallInteger::new(65535),
            zero_pointer: vm::SmallInteger::new(1),
            one_pointer: vm::SmallInteger::new(3),
            two_pointer: vm::SmallInteger::new(5),

            // UndefinedObject and Booleans
            nil_pointer: vm::SmallInteger::new(2),
            false_pointer: vm::SmallInteger::new(4),
            true_pointer: vm::SmallInteger::new(6),

            // Root
            scheduler_association_pointer: vm::SmallInteger::new(8),

            // Classes
            class_string_pointer: vm::SmallInteger::new(14),
            class_array_pointer: vm::SmallInteger::new(16),
            class_method_context_pointer: vm::SmallInteger::new(22),
            class_block_context_pointer: vm::SmallInteger::new(24),
            class_point_pointer: vm::SmallInteger::new(26),
            class_large_positive_integer_pointer: vm::SmallInteger::new(28),
            class_message_pointer: vm::SmallInteger::new(32),
            class_character_pointer: vm::SmallInteger::new(40),

            // Selectors
            does_not_understand_selector: vm::SmallInteger::new(42),
            cannot_return_selector: vm::SmallInteger::new(44),
            must_be_boolean_selector: vm::SmallInteger::new(52),

            // Tables
            special_selectors_pointer: vm::SmallInteger::new(48),
            character_table_pointer: vm::SmallInteger::new(50),
        }
    }

    pub fn fetch_integer(&self, field_index: i32, obj: vm::ObjectPointer) -> i16 {
        0
    }

    pub fn store_integer(&self, field_index: i32, obj: vm::ObjectPointer, value: i16) {}

    pub fn transfer(&self,
                    from_index: i32,
                    of_object: vm::ObjectPointer,
                    to_index: i32,
                    to_object: vm::ObjectPointer) {

    }

    pub fn extract_bits(&self,
                        first_index: i8,
                        to_index: i8,
                        of: vm::SmallInteger)
                        -> vm::SmallInteger {
        vm::SmallInteger::new(0)
    }

    pub fn high_byte_of(&self, of: vm::SmallInteger) -> vm::SmallInteger {
        vm::SmallInteger::new(0)
    }

    pub fn low_byte_of(&self, of: vm::SmallInteger) -> vm::SmallInteger {
        vm::SmallInteger::new(0)
    }
}