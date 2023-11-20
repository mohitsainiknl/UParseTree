
use crate::tree::{self, Node};

#[allow(unused)]
pub fn main() {
    
    #[allow(unused)]
    let json2 = r#"
        {
            "id": 1,
            "name": "root",
            "line": "21",
            "key": "column",
            "node-One": {
                "id": 2,
                "name": "node3",
                "line": "21",
                "key": "number"
            },
            "node-two": {
                "id": 3,
                "name": "node2",
                "line": "21",
                "key": "anything",
                "node-three": {
                    "id": 4,
                    "name": "node3",
                    "line": "21",
                    "key": "number"
                }
            }
        }
    "#;


    let deserialized: Node = tree::from_str(&json2).expect("Can't deserialize");
    println!("{}", deserialized);

    use std::path::Path;
    let deserialized: Node = tree::from_file_path(Path::new("output.json")).expect("Can't deserialize");
    println!("{}", deserialized);

}
