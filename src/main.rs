
use std::io::prelude::*;
use std::io::{Write, SeekFrom};
use std::fs::OpenOptions;
use std::env::args;

fn main() {
    let args: Vec<_> = args().collect();
    if args.len() < 2 {
        println!("Please supply a filename to sign...");
        return
    }

    println!("Hello, world!");
    println!("Lets sign a boot loader");

    // get file
    let mut file = OpenOptions::new()
        .append(true)
        .open(&args[1])
        .expect("Cannot open file");
    
    // find out size
    let len = file.metadata().expect("Cannot get file size").len();
    println!("FILE SIZE: {}", len);

    // Pad size with 0x00 until 510 & write 0x55 then 0xaa
    if len < 510 {
        let mut to_pad = 510 - len;
        let pad = [0 as u8; 1];
        println!("Padding {} times", to_pad);
        while to_pad > 0 {
            file.write(&pad).expect("Could not write padding");
            to_pad -= 1;
        }

        file.write(b"\x55").expect("Could not write 0x55");
        file.write(b"\xaa").expect("Could not write 0xaa");
 
    } else {
        println!("File is possibly too big, but I'll still attempt to sign it.  This may break some expected functionality.");
        let mut file = OpenOptions::new()
            .write(true)
            .open(&args[1])
            .expect("Cannot open file");
        file.seek(SeekFrom::Start(510)).expect("Could not seek to byte 510");
        file.write(b"\x55").expect("Could not write 0x55");
        file.write(b"\xaa").expect("Could not write 0xaa");
    }
}
