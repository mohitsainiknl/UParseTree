
use super::str;
use super::tree::{self, Node, Data, Value, Span};
use std::path::Path;

#[allow(unused)]
pub fn main() {
    let p_tree: Node = Node {
        id: 1,
        name: str!("root"),
        data: vec![
            Data {
                key: str!("line"),
                value: Value::String(String::from("21")),
            },
            Data {
                key: str!("key"),
                value: Value::String(str!("column")),
            },
            Data {
                key: str!("node-two"),
                value: Value::NextNode(Node {
                    id: 1,
                    name: str!("node2"),
                    data: vec![
                        Data {
                            key: str!("line"),
                            value: Value::String(String::from("21")),
                        },
                        Data {
                            key: str!("key"),
                            value: Value::String(str!("anything")),
                        },
                        Data {
                            key: str!("node-three"),
                            value: Value::NextNode(Node {
                                id: 1,
                                name: str!("node3"),
                                data: vec![
                                    Data {
                                        key: str!("line"),
                                        value: Value::String(String::from("21")),
                                    },
                                    Data {
                                        key: str!("key"),
                                        value: Value::String(str!("number")),
                                    },
                                ],
                                span: Span {
                                    start: (12, 12),
                                    end: (12, 12)
                                }
                            }),
                        },
                    ],
                    span: Span {
                        start: (12, 12),
                        end: (12, 12)
                    }
                }),
            },
        ],
        span: Span {
            start: (12, 12),
            end: (12, 12)
        }
    };

    // let json = tree::to_string_json(&p_tree).unwrap();
    // println!("{}", json);

    // let json_pretty = tree::to_string_pretty(&p_tree).unwrap();
    // println!("{}", json_pretty);

    tree::to_file_pretty(Path::new("output.json"), &p_tree).unwrap();

}

