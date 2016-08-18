/*
Copyright 2016 Robert Roland

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

use vm;
use vm::SmallInteger;

struct Interpreter {

}

impl Interpreter {
    pub fn fetch_integer(&self, field_index: i32, obj: vm::ObjectPointer) -> i16 {
        0
    }

    pub fn store_integer(&self, field_index: i32, obj: vm::ObjectPointer, value: i16) {
        
    }

    pub fn transfer(&self, from_index: i32, of_object: vm::ObjectPointer, to_index: i32, to_object: vm::ObjectPointer) {

    }

    pub fn extract_bits(&self, first_index: i8, to_index: i8, of: vm::SmallInteger) -> vm::SmallInteger {
        vm::SmallInteger::new(0)
    }    
}