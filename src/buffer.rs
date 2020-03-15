

use crate::pointer::NoProtoValue;
use crate::error::NoProtoError;
use crate::pointer::NoProtoPointer;
use crate::memory::NoProtoMemory;
use crate::schema::NoProtoSchema;
use crate::PROTOCOL_VERSION;
use std::{rc::Rc, cell::RefCell};


pub struct NoProtoBuffer<'a, T: NoProtoValue + Default> {
    pub memory: Rc<RefCell<NoProtoMemory>>,
    root_model: &'a NoProtoSchema<T>
}

impl<'a, T: NoProtoValue + Default> NoProtoBuffer<'a, T> {

    #[doc(hidden)]
    pub fn new(model: &'a NoProtoSchema<T>, capcity: Option<u32>) -> Self { // make new buffer

        let capacity = match capcity {
            Some(x) => x,
            None => 1024
        };

        let mut new_bytes: Vec<u8> = Vec::with_capacity(capacity as usize);

        new_bytes.extend(vec![
            PROTOCOL_VERSION, // Protocol version (for breaking changes if needed later)
            0, 0, 0, 0        // u32 HEAD for root value (starts at zero)
        ]); 

        NoProtoBuffer {
            memory: Rc::new(RefCell::new(NoProtoMemory { bytes: new_bytes })),
            root_model: model
        }
    }

    #[doc(hidden)]
    pub fn load(model: &'a NoProtoSchema<T>, bytes: Vec<u8>) -> Self { // load existing buffer
        NoProtoBuffer {
            memory: Rc::new(RefCell::new(NoProtoMemory { bytes: bytes})),
            root_model: model
        }
    }

    pub fn open<F>(&mut self, mut callback: F) -> std::result::Result<(), NoProtoError>
        where F: FnMut(NoProtoPointer<T>) -> std::result::Result<(), NoProtoError>
    {        
        let buffer = NoProtoPointer::new(1, self.root_model, Rc::clone(&self.memory));
        callback(buffer)
    }

    pub fn deep_set<X: NoProtoValue + Default, S: AsRef<str>>(&self, path: S, value: X) -> std::result::Result<(), NoProtoError> {
        Ok(())
    }

    pub fn deep_get<X: NoProtoValue + Default>(&self, path: &str) -> std::result::Result<Option<X>, NoProtoError> {
        Ok(Some(X::default()))
    }

    pub fn compact(&self)  {
        
    }

    pub fn close(self) -> std::result::Result<Vec<u8>, NoProtoError> {
        Ok(Rc::try_unwrap(self.memory)?.into_inner().dump())
    }

    pub fn calc_wasted_bytes(&self) -> u32 {

        // let total_bytes = self.memory.borrow().bytes.len() as u32;

        return 0;
    }

    pub fn maybe_compact<F>(&self, mut callback: F) -> bool 
        where F: FnMut(f32, f32) -> bool // wasted bytes, percent of waste
    {
        let wasted_bytes = self.calc_wasted_bytes() as f32;

        let total_bytes = self.memory.borrow().bytes.len() as f32;

        let size_without_waste = total_bytes - wasted_bytes;

        if callback(wasted_bytes, (total_bytes / size_without_waste) as f32) {
            self.compact();
            true
        } else {
            false
        }
    }
}