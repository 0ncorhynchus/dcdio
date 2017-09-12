extern crate dcdio;

use std::fs::File;
use dcdio::*;

fn main() {
    let file = File::open("./data/000.dcd").unwrap();
    let reader = DcdReader::new(file).unwrap();

    println!("num_frames: {}", reader.header.num_frames);
    println!("version: {}", reader.header.version);
    println!("title: {}", reader.header.title);

    for frame in reader.frames() {
        let frame = frame.unwrap();
        println!("step: {}", frame.step);
        println!("time: {}", frame.time);
    }
}
