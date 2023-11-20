// #[macro_use]
// extern crate serde_json;
// let obj: Value = json!({"foo":1,"bar":2});

use serde::Serialize;
use serde_json::error::Result;
use super::formatter;

#[allow(unused)]
pub fn to_string_pretty<T>(value: &T) -> Result<String> 
    where
        T: ?Sized + Serialize,
    {
    let indent: &[u8] = b"    ";

    let formatter = formatter::PrettyFormatter::with_indent(indent);
    let buf = Vec::new();
    let mut ser = formatter::Serializer::with_formatter(buf, formatter);

    value.serialize(&mut ser).unwrap();
    Ok(String::from_utf8(ser.into_inner()).unwrap())
}


use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use core::result::Result as CoreResult;
use std::error::Error;

#[allow(unused)]
pub fn to_file_pretty<T, P>( path: P, value: &T) -> CoreResult<(), Box<dyn Error>> 
where
    P: AsRef<Path>,
    T: ?Sized + Serialize,
{
    let mut file = File::create(&path)?;

    let json:String = to_string_pretty(value).unwrap();

    file.write_all(json.as_bytes())?;

    Ok(())
}



use serde::de;
use std::io::BufReader;

#[allow(unused)]
pub fn from_file_path<T, P>(path: P) -> CoreResult<T, Box<dyn Error>>
where
    T: de::DeserializeOwned,
    P: AsRef<Path>,
{
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let u = serde_json::from_reader(reader)?;

    Ok(u)
}

