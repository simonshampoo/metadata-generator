// main logic for parsing text files and generating random metadata structs for them
use std::fs;

pub fn get_trait_names() {
    let paths = fs::read_dir("./src/traits").unwrap();

    for path in paths {
        println!("{}", path.unwrap().path().display());
    }
}
