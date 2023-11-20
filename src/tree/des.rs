use super::tree::{Node, Data, Span, Value as TValue};

extern crate serde_derive;

use serde::de;


pub enum KeyType {
    Id,
    Name,
    Start,
    End,
    String(String),
}

impl<'de> serde::Deserialize<'de> for KeyType {
    fn deserialize<D>(deserializer: D) -> Result<KeyType, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct KeyVisitor;

        impl<'de> serde::de::Visitor<'de> for KeyVisitor {
            type Value = KeyType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "key must be a string")
            }

            fn visit_str<E>(self, value: &str) -> Result<KeyType, E>
            where
                E: serde::de::Error,
            {
                Ok(match value {
                    "id" => KeyType::Id,
                    "name" => KeyType::Name,
                    "start" => KeyType::Start,
                    "end" => KeyType::End,
                    _ => KeyType::String(String::from(value)),
                })
            }
        }

        deserializer.deserialize_identifier(KeyVisitor)
    }
}


impl<'de> serde::Deserialize<'de> for TValue {
    fn deserialize<D>(deserializer: D) -> Result<TValue, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ValueVisitor;

        impl<'de> serde::de::Visitor<'de> for ValueVisitor {
            type Value = TValue;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "Value must be a String/NextNode")
            }

            fn visit_map<E>(self, map: E) -> Result<TValue, E::Error>
            where
                E:  serde::de::MapAccess<'de>,
            {
                let node = NodeVisitor.visit_map(map)?;
                let value = TValue::NextNode(node);
                Ok(value)
            }
                        
            fn visit_str<E>(self, value: &str) -> Result<TValue, E>
            where
                E: serde::de::Error,
            {
                Ok(TValue::String(String::from(value)))
            }
        }

        deserializer.deserialize_any(ValueVisitor)
    }
}


impl<'de> serde::Deserialize<'de> for Node {
    fn deserialize<D>(deserializer: D) -> Result<Node, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(NodeVisitor)
    }
}

struct NodeVisitor;

impl<'de> serde::de::Visitor<'de> for NodeVisitor {
    type Value = Node;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "Tree node not correct")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Node, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut id:Option<i32> = None;
        let mut name:Option<String> = None;
        let mut start:Option<(i32, i32)> = None;
        let mut end:Option<(i32, i32)> = None;
        let mut data_vec: Vec<Data> = Vec::new();

        while let Some(key_type) = map.next_key()? {
            match key_type {
                KeyType::Id => {
                    if id.is_some() {
                        return Err(de::Error::duplicate_field("id"));
                    }
                    id = Some(map.next_value()?);
                }
                KeyType::Name => {
                    if name.is_some() {
                        return Err(de::Error::duplicate_field("name"));
                    }
                    name = Some(map.next_value()?);
                }
                KeyType::Start => {
                    if start.is_some() {
                        return Err(de::Error::duplicate_field("start"));
                    }
                    start = Some(map.next_value()?);
                }
                KeyType::End => {
                    if end.is_some() {
                        return Err(de::Error::duplicate_field("end"));
                    }
                    end = Some(map.next_value()?);
                }
                KeyType::String(key_string) => {
                    let value:Option<_> = map.next_value()?;

                    if let Option::Some(value) = value {
                        #[allow(unused_assignments)]
                        let mut data_value: Option<TValue> = None;

                        match value {
                            TValue::String(val) => {
                                data_value = Some(TValue::String(val));
                            },
                            TValue::NextNode(node) => {
                                data_value = Some(TValue::NextNode(node));
                            }
                        }

                        let data: Data = Data {
                            key: key_string,
                            value: data_value.unwrap()
                        };        
                        data_vec.push(data);
                    }
                }
            }
        }


        Ok(Node {
            id: id.unwrap(),
            name: name.unwrap(),
            data: data_vec,
            span: Span {
                start: start.unwrap(),
                end: end.unwrap()
            }
        })
    }
}


