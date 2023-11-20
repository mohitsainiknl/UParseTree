# UParseTree

## Overview

UParseTree is a Rust library designed to facilitate code generation by representing language code in a standardized format using a universal parser tree. This repository contains the main deserializer and serializer implementations for the parser tree.

## Main Deserializer

The main deserializer code is located in the `main` function in the `lib.rs` file. It demonstrates how to deserialize a JSON representation of a universal parser tree using the UParseTree library. Example JSON data is provided in the `json2` variable for illustration.

### Usage

```rust
use crate::tree::{self, Node};
use std::path::Path;

#[allow(unused)]
pub fn main() {
    // Deserialize from a string
    let json2 = r#"
        {
            // JSON data here
        }
    "#;
    let deserialized: Node = tree::from_str(&json2).expect("Can't deserialize");
    println!("{}", deserialized);

    // Deserialize from a file
    let deserialized: Node = tree::from_file_path(Path::new("output.json")).expect("Can't deserialize");
    println!("{}", deserialized);
}
```
The repository includes `output.json` file showcasing the output of the deserialization process.


## Main Serializer

The main serializer code is also included in the `lib.rs` file. It demonstrates how to create a universal parser tree in Rust and serialize it into a JSON format using the UParseTree library.

### Usage

```rust
use super::str;
use super::tree::{self, Node, Data, Value, Span};
use std::path::Path;

#[allow(unused)]
pub fn main() {
    // Create a sample universal parser tree
    let p_tree: Node = {
        // Node structure definition here
    };

    // Serialize the tree to a file in a pretty format
    tree::to_file_pretty(Path::new("output.json"), &p_tree).unwrap();
}
```

## Project Dependencies

This project utilizes a formatter to serialize a Rust data structure into JSON data. Ensure you have the necessary dependencies installed.

## Integration with Tiler

This repository is part of a larger ecosystem and is intended to be used alongside another repository called "Tiler." Tiler facilitates the transpilation of code from one programming language to another.

Feel free to explore the functionalities provided by UParseTree and its integration with Tiler for seamless code conversion between programming languages.

For more information, refer to the respective repositories: [UParseTree](https://github.com/mohitsainiknl/UParseTree) and [Tiler](https://github.com/mohitsainiknl/Tiler).