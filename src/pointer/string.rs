use crate::error::NoProtoError;
use crate::memory::NoProtoMemory;
use std::{cell::RefCell, rc::Rc};
use crate::pointer::NoProtoValue;
use super::NoProtoPointerKinds;

impl NoProtoValue for String {

    fn new() -> Self {
        String::default()
    }

    fn is_type(&self, type_str: &str) -> bool {
        "string" == type_str || "str" == type_str || "utf8" == type_str
    }

    fn type_idx() -> (i64, &'static str) { (1, "string") }
    fn self_type_idx(&self) -> (i64, &'static str) { (1, "string") }

    fn buffer_read(&mut self, address: u32, kind: &NoProtoPointerKinds, buffer: Rc<RefCell<NoProtoMemory>>) -> std::result::Result<Option<String>, NoProtoError> {

        let addr = kind.get_value() as usize;

        // empty value
        if addr == 0 {
            return Ok(None);
        }
        
        // get size of string
        let mut size: [u8; 4] = [0; 4];
        let memory = buffer.try_borrow()?;
        size.copy_from_slice(&memory.bytes[addr..(addr+4)]);
        let str_size = u32::from_le_bytes(size) as usize;

        // get string bytes
        let array_bytes = &memory.bytes[(addr+4)..(addr+4+str_size)];

        // convert to string
        let newString = String::from_utf8(array_bytes.to_vec())?;

        Ok(Some(newString))
    }

    fn buffer_write<S: AsRef<str>>(&mut self, address: u32, kind: &NoProtoPointerKinds, buffer: Rc<RefCell<NoProtoMemory>>, value: String) -> std::result::Result<NoProtoPointerKinds, NoProtoError> {

        let bytes = value.as_bytes();
        let str_size = bytes.len() as u64;

        if str_size >= std::u32::MAX as u64 { 
            Err(NoProtoError::new("String too large!"))
        } else {

            let mut addr = kind.get_value() as usize;
            let mut set_addr = false;

            {
                let mut memory = buffer.try_borrow_mut()?;

                let prev_size: usize = if addr != 0 {
                    let mut size_bytes: [u8; 4] = [0; 4];
                    size_bytes.copy_from_slice(&memory.bytes[addr..(addr+4)]);
                    u32::from_le_bytes(size_bytes) as usize
                } else {
                    0 as usize
                };

                if prev_size >= str_size as usize { // previous string is larger than this one, use existing memory
            
                    let size_bytes = (str_size as u32).to_le_bytes();
                    // set string size
                    for x in 0..size_bytes.len() {
                        memory.bytes[(addr + x) as usize] = size_bytes[x as usize];
                    }

                    // set bytes
                    for x in 0..bytes.len() {
                        memory.bytes[(addr + x + 4) as usize] = bytes[x as usize];
                    }

                } else { // not enough space or space has not been allocted yet
                    

                    // first 4 bytes are string length
                    addr = memory.malloc((str_size as u32).to_le_bytes().to_vec())? as usize;

                    set_addr = true;

                    // then string content
                    memory.malloc(bytes.to_vec())?;
                }
            }

            if set_addr { 
                Ok(kind.set_value_address(address, addr as u32, buffer)?)
            } else {
                Ok(*kind)
            }
        }
    }
}


impl NoProtoValue for &str {

    fn new() -> Self {
        ""
    }

    fn is_type(&self, type_str: &str) -> bool {
        "string" == type_str || "str" == type_str || "utf8" == type_str
    }

    fn type_idx() -> (i64, &'static str) { (1, "string") }
    fn self_type_idx(&self) -> (i64, &'static str) { (1, "string") }

    fn buffer_read(&mut self, address: u32, kind: &NoProtoPointerKinds, buffer: Rc<RefCell<NoProtoMemory>>) -> std::result::Result<Option<&str>, NoProtoError> {

        let addr = kind.get_value() as usize;

        // empty value
        if addr == 0 {
            return Ok(None);
        }
        
        // get size of string
        let mut size: [u8; 4] = [0; 4];
        let memory = buffer.try_borrow()?;
        size.copy_from_slice(&memory.bytes[addr..(addr+4)]);
        let str_size = u32::from_le_bytes(size) as usize;

        // get string bytes
        let array_bytes = &memory.bytes[(addr+4)..(addr+4+str_size)];

        // convert to string
        let newString = String::from_utf8(array_bytes.to_vec())?;

        Ok(Some(newString.as_mut_str()))
    }

    fn buffer_write(&mut self, address: u32, kind: &NoProtoPointerKinds, buffer: Rc<RefCell<NoProtoMemory>>, value: &str) -> std::result::Result<NoProtoPointerKinds, NoProtoError> {

        let bytes = value.as_bytes();
        let str_size = bytes.len() as u64;

        if str_size >= std::u32::MAX as u64 { 
            Err(NoProtoError::new("String too large!"))
        } else {

            let mut addr = kind.get_value() as usize;
            let mut set_addr = false;

            {
                let mut memory = buffer.try_borrow_mut()?;

                let prev_size: usize = if addr != 0 {
                    let mut size_bytes: [u8; 4] = [0; 4];
                    size_bytes.copy_from_slice(&memory.bytes[addr..(addr+4)]);
                    u32::from_le_bytes(size_bytes) as usize
                } else {
                    0 as usize
                };

                if prev_size >= str_size as usize { // previous string is larger than this one, use existing memory
            
                    let size_bytes = (str_size as u32).to_le_bytes();
                    // set string size
                    for x in 0..size_bytes.len() {
                        memory.bytes[(addr + x) as usize] = size_bytes[x as usize];
                    }

                    // set bytes
                    for x in 0..bytes.len() {
                        memory.bytes[(addr + x + 4) as usize] = bytes[x as usize];
                    }

                } else { // not enough space or space has not been allocted yet
                    

                    // first 4 bytes are string length
                    addr = memory.malloc((str_size as u32).to_le_bytes().to_vec())? as usize;

                    set_addr = true;

                    // then string content
                    memory.malloc(bytes.to_vec())?;
                }
            }

            if set_addr { 
                Ok(kind.set_value_address(address, addr as u32, buffer)?)
            } else {
                Ok(*kind)
            }
        }
    }
}