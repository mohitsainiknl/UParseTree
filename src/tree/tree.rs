
use std::fmt::{Display, Formatter, self};

use serde::{self, ser::{Serialize, Serializer, SerializeMap}};

// extern crate serde_derive;

#[derive(Debug)]
// #[derive(serde::Deserialize)]
// #[derive(serde::Serialize)]
pub enum Value {
    String(String),
    NextNode(Node),
}


#[derive(Debug)]
#[derive(serde::Deserialize)]
#[derive(serde::Serialize)]
pub struct Span {
    pub start: (i32, i32),  // ( Line, Column)
    pub end: (i32, i32),  // ( Line, Column)
}

#[derive(Debug)]
#[derive(serde::Deserialize)]
#[derive(serde::Serialize)]
pub struct Data {
    pub key: String,
    pub value: Value,
}

#[derive(Debug)]
// #[derive(serde::Deserialize)]
// #[derive(serde::Serialize)]
pub struct Node {
    pub id: i32,
    pub name: String,
    pub data: Vec<Data>,
    // #[serde(flatten)]
    pub span: Span
}


impl Node {
    #[allow(unused)]
    pub fn new() -> Node {
        Node {
            id: 0,
            name: String::new(),
            data: Vec::new(),
            span: Span {
                start: (12, 12),
                end: (12, 12)
            }
        }
    }
}


impl Display for Value {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match &self {
            Value::String(ref str) => write!(f, "{}", str),
            // Value::Int32(num) => write!(f, "{}", num),
            Value::NextNode(node) => write!(f, "\n{}", node),
        }
    }
}

impl Display for Span {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let data_str = format!("`{{` start = [{},{}], name = [{},{}] `}}`", &self.start.0, &self.start.1, &self.end.0, &self.end.1);
        write!(f, "{}", data_str)
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut data_str = format!("Node : 
        id = {},
        name = {},"
        , &self.id, &self.name);

        let data_vec = &self.data;
        for val in data_vec.iter() {
            let str = format!("
            {} = {},"
            , val.key, val.value);
            data_str.push_str(&str);
        }
        data_str.push_str(&format!("
        start = [{},{}]", self.span.start.0, self.span.start.1));
        data_str.push_str(&format!("
        end = [{},{}]", self.span.start.0, self.span.start.1));

        data_str.push_str("\n-----------------------------------");

        write!(f, "{}", data_str)
    }
}

impl Serialize for Node {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(&self.data.len() + 4))?;
        map.serialize_entry("id", &self.id)?;
        map.serialize_entry("name", &self.name)?;
        for val in self.data.iter() {
            match val.value {
                Value::NextNode(ref node) => {
                    map.serialize_entry(&val.key, node)?;
                },
                _ => map.serialize_entry(&val.key, &val.value)?
            }
        }
        map.serialize_entry("start", &self.span.start)?;
        map.serialize_entry("end", &self.span.end)?;
        map.end()
    }
}


impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            Value::String(ref str) => serializer.collect_str(str),
            // Value::Int32(num) => serializer.serialize_i32(num),
            Value::NextNode(_) => todo!(), // Unreachable option
        }
    }
}

