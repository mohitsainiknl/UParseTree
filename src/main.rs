mod tree;
mod utils;
// mod hello;

extern crate itoa;
extern crate ryu;
extern crate serde_derive;

mod main_ser;

mod main_des;
mod parser;

fn main() {

    // main_ser::main();
    
    // main_des::main();
    parser::main();
}