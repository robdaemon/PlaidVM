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
struct Context {
    // Class MethodContext
    sender_index: vm::SmallInteger,
    instruction_pointer_index: vm::SmallInteger,
    stack_pointer_index: vm::SmallInteger,
    method_index: vm::SmallInteger,
    receiver_index: vm::SmallInteger,
    temp_frame_start: vm::SmallInteger,

    // Class BlockContext
    caller_index: vm::SmallInteger,
    block_argument_count_index: vm::SmallInteger,
    initial_p_index: vm::SmallInteger,
    home_index: vm::SmallInteger,
}

#[allow(dead_code)]
#[allow(unused_variables)]
impl Context {
    pub fn new() -> Context {
        Context {
            sender_index: vm::SmallInteger::new(0),
            instruction_pointer_index: vm::SmallInteger::new(1),
            stack_pointer_index: vm::SmallInteger::new(2),
            method_index: vm::SmallInteger::new(3),
            receiver_index: vm::SmallInteger::new(5),
            temp_frame_start: vm::SmallInteger::new(6),

            // Class BlockContext
            caller_index: vm::SmallInteger::new(0),
            block_argument_count_index: vm::SmallInteger::new(3),
            initial_p_index: vm::SmallInteger::new(4),
            home_index: vm::SmallInteger::new(5),
        }
    }

    pub fn instruction_pointer_of_context(&self, context_pointer: Context) -> u16 {
        // self.fetch_integer(self.instruction_pointer_index, context_pointer)
        0
    }

    pub fn store_instruction_pointer_value(&self, value: u16, in_context: Context) {
        // self.store_integer(instruction_pointer_index, in_context, value)
    }

    pub fn stack_pointer_of_context(&self, context_pointer: u16) -> u16 {
        // self.fetch_integer(self.stack_pointer_index, context_pointer)
        0
    }

    pub fn store_stacK_pointer_value(&self, value: u16, in_context: Context) {
        // self.store_integer(self.stack_pointer_index, in_context, value)
    }

    pub fn argument_count_of_block(&self, block_pointer: u16) -> u16 {
        // self.fetch_integer(self.block_argument_count_index, block_pointer)
        0
    }

    
}
