extern crate dcdio;

use std::fs::File;
use dcdio::*;

fn main() {
    let file = File::open("./data/000.dcd").unwrap();
    let reader = DcdReader::new(file).unwrap();

    println!("num_frames: {}", reader.num_frames());
    println!("version: {}", reader.version());
    println!("title: {}", reader.title());
}
