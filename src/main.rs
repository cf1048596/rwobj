#![allow(dead_code)]
mod instructions;
use clap::{Arg, ArgAction, Command};
use std::fs;
use std::error::Error;
use std::io::Read;
use std::io::prelude::*;
use std::fs::File;
use byteorder::{LittleEndian, ReadBytesExt};

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

pub enum SegmentType {
    Text,
    Data,
    Bss,
    NumSegments
}

pub struct RelocEntry {
    address: u32,
    symbol_ptr: u32,
    ref_type: ReferenceType,
    seg_type : Option<SegmentType>
}

pub struct LabelEntry<'a> {
    name : &'a str,
    address : i32,
    seg_type : SegmentType,
    resolved : bool,
    is_global : bool,
    file_no : i32,
}

pub struct Reference<'a> {
   label_entry : Option<&'a mut LabelEntry<'a>>,
   source_seg : SegmentType,
   target_seg : SegmentType,
   address : i32,
}

pub struct FileType<'a> {
    filename : &'a str,
    file_header : ObjectHeader,
    segment : Vec::<u32>,
    segment_address : Vec::<u32>,
    reference : Option<&'a mut Reference<'a>>
}

//label_entry *get_label(char *name);
//return pointer to label_entry struct 
//creates new entry if no entry is found
//just use a vec anyway

//label_entry *get_label_address(int address, reference_type type)
//searches for a reference to a label
//create new entry if no entry is found
//just use a vec anyway

pub const MAGIC_NUMBER : u32 = 0xdaa1;

fn main() -> Result<(), Box<dyn Error>> {
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

    let file_name = matches.get_one::<String>("file").expect("File is required");
    let disassemble = *matches.get_one::<bool>("disassemble").unwrap_or(&false);

    // Check if the file exists
    if fs::metadata(file_name).is_ok() {
        println!("Processing file: {}", file_name);
        let mut file = File::open(file_name)?;
        let magic_number = file.read_u32::<LittleEndian>()?;
        println!("{:X}", magic_number);
        if magic_number == MAGIC_NUMBER {
            println!("magic number is correct");

            if disassemble {

            }
        } else {
            println!("magic number is not correct");
        }
    } else {
        eprintln!("Error: File '{}' does not exist.", file_name);
    }
    Ok(())
}

