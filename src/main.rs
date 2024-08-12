#![allow(dead_code)]
mod instructions;
use byteorder::ByteOrder;
use byteorder::{LittleEndian, ReadBytesExt};
use clap::{Arg, ArgAction, Command};
use std::error::Error;
use std::{fs, io};
use std::fs::File;
use std::io::prelude::*;
use std::io::Read;

pub const SEG_TYPE_NAME: [&str; 5] = ["NONE", "TEXT", "DATA", "BSS", "NUM_SEGMENTS"];
pub const TEXT : usize = 1;
pub const DATA : usize = 2;
pub const BSS : usize = 3;

#[derive(Clone, Copy, Debug)]
pub struct ObjectHeader {
    magic_number: u32,
    text_seg_size: u32,
    data_seg_size: u32,
    bss_seg_size: u32,
    num_references: u32,
    symbol_name_table_size: u32,
}

impl ObjectHeader {
    fn from_reader<R: Read>(reader: &mut R) -> io::Result<Self> {
        let mut buffer = [0u8; 24];
        reader.read_exact(&mut buffer)?;
        Ok(Self {
            magic_number: LittleEndian::read_u32(&buffer[0..4]),
            text_seg_size: LittleEndian::read_u32(&buffer[4..8]),
            data_seg_size: LittleEndian::read_u32(&buffer[8..12]),
            bss_seg_size: LittleEndian::read_u32(&buffer[12..16]),
            num_references: LittleEndian::read_u32(&buffer[16..20]),
            symbol_name_table_size: LittleEndian::read_u32(&buffer[20..24]),
        })
    }
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
    NumSegments,
}

pub struct RelocEntry {
    address: u32,
    symbol_ptr: u32,
    ref_type: ReferenceType,
    seg_type: Option<SegmentType>,
}

pub struct LabelEntry<'a> {
    name: &'a str,
    address: i32,
    seg_type: SegmentType,
    resolved: bool,
    is_global: bool,
    file_no: i32,
}

pub struct Reference<'a> {
    label_entry: Option<&'a mut LabelEntry<'a>>,
    source_seg: SegmentType,
    target_seg: SegmentType,
    address: i32,
}

pub struct FileType<'a> {
    filename: &'a str,
    file_header: ObjectHeader,
    //holds starting addresses of each segment
    segment: Vec<Vec<&'a u32>>,
    segment_address: Vec<u32>,
    reference: Option<&'a mut Reference<'a>>,
}

//label_entry *get_label(char *name);
//return pointer to label_entry struct
//creates new entry if no entry is found
//just use a vec anyway

//label_entry *get_label_address(int address, reference_type type)
//searches for a reference to a label
//create new entry if no entry is found
//just use a vec anyway

pub const MAGIC_NUMBER: u32 = 0xdaa1;

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
        let file = File::open(file_name)?;
        let mut reader = io::BufReader::new(file);
        let header = ObjectHeader::from_reader(&mut reader)?;
        println!("{:#?}", header);
        if header.magic_number == MAGIC_NUMBER {
            println!("magic number is correct");
            let mut file_type: FileType = FileType {
                filename: &file_name,
                file_header: header,
                segment : {
                let mut segment_vec = Vec::new();
                    //text
                    segment_vec.push(Vec::with_capacity(header.text_seg_size as usize));
                    //data
                    segment_vec.push(Vec::with_capacity(header.data_seg_size as usize));
                    segment_vec
                },
                segment_address : {
                let mut segment_address_vec = Vec::new();
                    //text
                    segment_address_vec.push(0);
                    //data
                    segment_address_vec.push(0);
                    //bss
                    segment_address_vec.push(0);
                    segment_address_vec
                },
                reference : None
            };
            if disassemble {}
        } else {
            println!("magic number is not correct");
        }
    } else {
        eprintln!("Error: File '{}' does not exist.", file_name);
    }
    Ok(())
}
