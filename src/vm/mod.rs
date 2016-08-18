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

use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};

mod smallinteger;

pub use self::smallinteger::SmallInteger;

pub struct ObjectPointer {
    ref_count: AtomicUsize
}

impl ObjectPointer {
    pub fn new() -> ObjectPointer {
        ObjectPointer { ref_count: ATOMIC_USIZE_INIT }
    }

    pub fn increase_references_to(&self) {
        let _ = self.ref_count.fetch_add(1, Ordering::SeqCst);
    }

    pub fn decrease_references_to(&self) {
        let _ = self.ref_count.fetch_sub(1, Ordering::SeqCst);
    }
}

pub struct ValuePointer {

}

pub struct ClassPointer {

}

pub struct ObjectMemory {

}

impl ObjectMemory {
    pub fn fetch_pointer(&self, field_index: i32, obj: ObjectPointer) -> ObjectPointer {
        ObjectPointer::new()
    }

    pub fn store_pointer(&self, field_index: i32, obj: ObjectPointer, value: ValuePointer) {

    }

    pub fn fetch_word(&self, field_index: i32, obj: ObjectPointer) -> i16 {
        0
    }

    pub fn store_word(&self, field_index: i32, obj: ObjectPointer, value: i16) {

    }

    pub fn fetch_byte(&self, byte_index: i32, obj: ObjectPointer) -> i8 {
        0
    }

    pub fn store_byte(&self, byte_index: i32, obj: ObjectPointer, value: i8) {

    }

    pub fn increase_references_to(&self, obj: ObjectPointer) {
        obj.increase_references_to()
    }

    pub fn decrease_references_to(&self, obj: ObjectPointer) {
        obj.decrease_references_to()
    }

    pub fn instantiate_class_with_pointers(&self, class: ClassPointer, instance_size: i32) -> ObjectPointer {
        ObjectPointer::new()
    }

    pub fn instantiate_class_with_words(&self, class: ClassPointer, instance_size: i32) -> ObjectPointer {
        ObjectPointer::new()
    }

    pub fn instantiate_class_with_bytes(&self, class: ClassPointer, instance_size: i32) -> ObjectPointer {
        ObjectPointer::new()
    }

    pub fn initial_instance_of(&self, class: ClassPointer) -> ObjectPointer {
        ObjectPointer::new()        
    }

    pub fn instance_after(&self, obj: ObjectPointer) -> ObjectPointer {
        ObjectPointer::new()        
    }

    pub fn swap_pointers_of(&self, first: ObjectPointer, second: ObjectPointer) {
    }

    pub fn integer_value_of(&self, obj: ObjectPointer) -> i16 {
        0
    }

    pub fn integer_object_of(&self, value: i16) -> ObjectPointer {
        ObjectPointer::new()                
    }

    pub fn is_integer_object(&self, obj: ObjectPointer) -> bool {
        false
    }

    pub fn is_integer_value(&self, obj: ObjectPointer) -> bool {
        false
    }
}