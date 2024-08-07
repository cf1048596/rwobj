#![allow(dead_code)]
mod instructions;
use clap::{Arg, ArgAction, Command};
use std::fs;

pub const SEG_TYPE_NAME : [&str;5] =  ["NONE", "TEXT", "DATA", "BSS", "NUM_SEGMENTS"];

pub struct ObjectHeader {
    magic_number : u32,
    text_seg_size : u32,
    data_seg_size : u32,
    bss_seg_size : u32,
    num_references : u32,
    symbol_name_table_size : u32,
}

pub enum ReferenceType {
    GlobalData,
    GlobalText,
    GlobalBss,
    TextLabelRef,
    DataLabelRef,
    BssLabelRef,
    ExternalRef,
}

pub struct RelocEntry{
    address: u32,
    symbol_ptr: u32,
}

pub const : i32 MAGIC_NUMBER = 0xdaa1








fn main() {
    let matches = Command::new("wobj")
        .version("1.0")
        .author("cf1048596")
        .about("A tool to view WRAMP obj files")
        .arg(
            Arg::new("file")
                .help("The file to process")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("disassemble")
                .short('d')
                .long("disassemble")
                .help("Display disassembly")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    let file = matches.get_one::<String>("file").expect("File is required");
    let disassemble = *matches.get_one::<bool>("disassemble").unwrap_or(&false);

    // Check if the file exists
    if fs::metadata(file).is_ok() {
        println!("Processing file: {}", file);

        if disassemble {
            println!("Displaying disassembly...");
            // Add disassembly logic here
        }
    } else {
        eprintln!("Error: File '{}' does not exist.", file);
    }
}




