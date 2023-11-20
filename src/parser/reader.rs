
use std::io::{self, Read};
use std::path::Path;
use std::fs::File;

fn read_from(path: &Path) -> Result<String, io::Error>{

    let mut file = File::open(&path)?;

    let mut code = String::new();

    file.read_to_string(&mut code)?;

    Ok(code)
}



pub fn main() {
    let code = read_from(Path::new("./src/hello/hello.rs")).unwrap();

    println!("Code :\n{}", code);
}