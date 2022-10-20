// main logic for parsing text files and generating random metadata structs for them
use crate::metadata::Trait;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

pub fn get_trait_names() -> Result<Vec<String>, std::io::Error> {
    Ok(vec![String::from("WAH")])
}

pub fn idk_what_to_name_this_yet() -> std::io::Result<()> {
    let path = Path::new("./src/traits/hats.txt");
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    
    for line in reader.lines() {
        println!("{}", line?);
    }

    Ok(())
}
