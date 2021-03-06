//! Clone type for recursive or duplicating data types.
//! 

use crate::{memory::NP_Memory, schema::{NP_Parsed_Schema}};
use alloc::vec::Vec;

use crate::json_flex::{JSMAP, NP_JSON};
use crate::schema::{NP_TypeKeys};
use crate::{pointer::NP_Value, error::NP_Error};


use alloc::string::String;
use alloc::boxed::Box;
use alloc::string::ToString;
use alloc::borrow::ToOwned;

use super::{NP_Cursor};

/// Defines the behavior of the portal data type
pub struct NP_Portal();


impl<'value> NP_Value<'value> for NP_Portal {
    fn type_idx() -> (&'value str, NP_TypeKeys) {
        ("portal", NP_TypeKeys::Portal)
    }

    fn self_type_idx(&self) -> (&'value str, NP_TypeKeys) {
        ("portal", NP_TypeKeys::Portal)
    }

    fn schema_to_json(schema: &Vec<NP_Parsed_Schema>, address: usize)-> Result<NP_JSON, NP_Error> {
        match &schema[address] {
            NP_Parsed_Schema::Portal { path, ..} => {
                let mut schema_json = JSMAP::new();
                schema_json.insert(
                    "type".to_owned(),
                    NP_JSON::String(Self::type_idx().0.to_string()),
                );

                schema_json.insert(
                    "to".to_owned(),
                    NP_JSON::String(path.clone())
                );

                Ok(NP_JSON::Dictionary(schema_json))
            },
            _ => Ok(NP_JSON::Null)
        }
    }

    fn from_json_to_schema(mut schema: Vec<NP_Parsed_Schema>, json_schema: &Box<NP_JSON>) -> Result<(bool, Vec<u8>, Vec<NP_Parsed_Schema>), NP_Error> {
        let mut schema_vec: Vec<u8> = Vec::new();
        schema_vec.push(NP_TypeKeys::Portal as u8);
        match &json_schema["to"] {
            NP_JSON::String(path) => {
                schema.push(NP_Parsed_Schema::Portal {
                    i: NP_TypeKeys::Portal,
                    sortable: false,
                    path: path.clone(),
                    schema: 0,
                    parent_schema: 0
                });
                let path_bytes = path.as_bytes();
                schema_vec.extend(&(path_bytes.len() as u16).to_be_bytes()[..]);
                schema_vec.extend(path_bytes);
            },
            _ => return Err(NP_Error::new("Portal types require a 'to' parameter!"))
        }

        Ok((false, schema_vec, schema)) 
    }

    fn from_bytes_to_schema(mut schema: Vec<NP_Parsed_Schema>, address: usize, bytes: &[u8]) -> (bool, Vec<NP_Parsed_Schema>) {
        let path_size: [u8; 2] = unsafe { *((&bytes[(address+1)..(address+3)]) as *const [u8] as *const [u8; 2]) };

        let path_size = u16::from_be_bytes(path_size) as usize;

        let path = &bytes[(address+3)..(address+3+path_size)];

        let path_str = unsafe { core::str::from_utf8_unchecked(path) };

        schema.push(NP_Parsed_Schema::Portal {
            i: NP_TypeKeys::Portal,
            sortable: false,
            path: String::from(path_str),
            schema: 0,
            parent_schema: 0
        });

        (false, schema)
    }

    fn default_value(_depth: usize, _schema_addr: usize, _schemas: &Vec<NP_Parsed_Schema>) -> Option<Self> {
        None
    }

    fn to_json<M: NP_Memory>(depth:usize, cursor: &NP_Cursor, memory: &'value M) -> NP_JSON {
        match memory.get_schema(cursor.schema_addr) {
            NP_Parsed_Schema::Portal { schema, parent_schema, .. } => {
                let mut next = cursor.clone();
                next.schema_addr = *schema;
                next.parent_schema_addr = *parent_schema;
                NP_Cursor::json_encode(depth + 1, &next, memory)
            },
            _ => NP_JSON::Null
        }
    }

    fn set_from_json<'set, M: NP_Memory>(depth: usize, apply_null: bool, cursor: NP_Cursor, memory: &'set M, value: &Box<NP_JSON>) -> Result<(), NP_Error> where Self: 'set + Sized {
        match memory.get_schema(cursor.schema_addr) {
            NP_Parsed_Schema::Portal { schema, parent_schema, .. } => {
                let mut next = cursor.clone();
                next.schema_addr = *schema;
                next.parent_schema_addr = *parent_schema;
                NP_Cursor::set_from_json(depth + 1, apply_null, next, memory, value)
            },
            _ => { Ok(()) }
        }
    }

    fn get_size<M: NP_Memory>(depth:usize, cursor: &'value NP_Cursor, memory: &'value M) -> Result<usize, NP_Error> {
        match memory.get_schema(cursor.schema_addr) {
            NP_Parsed_Schema::Portal { schema, parent_schema, .. } => {
                let mut next = cursor.clone();
                next.schema_addr = *schema;
                next.parent_schema_addr = *parent_schema;
                NP_Cursor::calc_size(depth + 1, &next, memory)
            },
            _ => Err(NP_Error::new("unreachable"))
        }
    }

    fn do_compact<M: NP_Memory, M2: NP_Memory>(depth:usize, mut from_cursor: NP_Cursor, from_memory: &'value M, mut to_cursor: NP_Cursor, to_memory: &'value M2) -> Result<NP_Cursor, NP_Error> where Self: 'value + Sized {
        match from_memory.get_schema(from_cursor.schema_addr) {
            NP_Parsed_Schema::Portal { schema, parent_schema, .. } => {
                from_cursor.schema_addr = *schema;
                from_cursor.parent_schema_addr = *parent_schema;
                to_cursor.schema_addr = *schema;
                to_cursor.parent_schema_addr = *parent_schema;
                NP_Cursor::compact(depth + 1, from_cursor, from_memory, to_cursor, to_memory)
            },
            _ => Err(NP_Error::new("unreachable"))
        }
    }
}



#[test]
fn schema_parsing_works() -> Result<(), NP_Error> {

    let schema = "{\"type\":\"portal\",\"to\":\"\"}";
    let factory = crate::NP_Factory::new_json(schema)?;
    assert_eq!(schema, factory.schema.to_json()?.stringify());
    let factory2 = crate::NP_Factory::new_compiled(factory.compile_schema())?;
    assert_eq!(schema, factory2.schema.to_json()?.stringify());

    Ok(())
}


#[test]
fn infinite_recursion() -> Result<(), NP_Error> {
    let schema = r#"{
        "type": "struct",
        "fields": [
            ["street", {"type": "string"}],
            ["city"  , {"type": "string"}],
            ["nested", {"type": "portal", "to": "nested"}]
        ]
    }"#;
    let factory = crate::NP_Factory::new_json(schema)?;
    let mut buffer = factory.empty_buffer(None);

    match buffer.set(&["nested","nested", "nested"], "hello infinite") {
        Ok(_done) => {
            panic!()
        },
        Err(_e) => {
            // should hit select overflow, if it doesn't we have a problem
        }
    }

    match buffer.get::<&str>(&["nested","nested", "nested"]) {
        Ok(_done) => {
            panic!()
        },
        Err(_e) => {
            // should hit select overflow, if it doesn't we have a problem
        }
    }

    Ok(())
}

#[test]
fn set_clear_value_and_compaction_works() -> Result<(), NP_Error> {
    let schema = r#"{
        "type": "struct",
        "fields": [
            ["street", {"type": "string"}],
            ["city"  , {"type": "string"}],
            ["nested", {"type": "portal", "to": ""}]
        ]
    }"#;
    let factory = crate::NP_Factory::new_json(schema)?;
    let mut buffer = factory.empty_buffer(None);

    buffer.set(&["nested", "street"], "hello street")?;
    buffer.set(&["nested", "nested", "nested", "nested", "street"], "hello street 2")?;

    assert_eq!("hello street", buffer.get::<&str>(&["nested", "street"])?.unwrap());
    assert_eq!("hello street 2", buffer.get::<&str>(&["nested", "nested", "nested", "nested", "street"])?.unwrap());
    assert_eq!(buffer.calc_bytes()?.current_buffer, buffer.calc_bytes()?.after_compaction);
    buffer.del(&["nested", "street"])?;
    buffer.compact(None)?;
    assert_eq!("hello street 2", buffer.get::<&str>(&["nested", "nested", "nested", "nested", "street"])?.unwrap());
    assert_eq!(None, buffer.get::<&str>(&["nested", "street"])?);

    // testing set with JSON
    buffer.set_with_json(&[], r#"{"value":{"street": "foo", "nested": {"street": "foo2"}}}"#)?;

    assert_eq!(Some("foo"), buffer.get::<&str>(&["street"])?);
    assert_eq!(Some("foo2"), buffer.get::<&str>(&["nested", "street"])?);


    let schema = r#"{
        "type": "struct",
        "fields": [
            ["username", {"type": "string"}],
            ["email"  , {"type": "string"}],
            ["address", {"type": "struct", "fields": [
                ["street", {"type": "string"}],
                ["city", {"type": "string"}],
                ["more", {"type": "portal", "to": "address"}]
            ]}]
        ]
    }"#;
    let factory = crate::NP_Factory::new_json(schema)?;
    let mut buffer = factory.empty_buffer(None);

    buffer.set(&["address", "more", "more","more", "more","more", "more","more", "more", "street"], "hello")?;

    assert_eq!("hello", buffer.get::<&str>(&["address", "more", "more","more", "more","more", "more","more", "more", "street"])?.unwrap());

    Ok(())
}