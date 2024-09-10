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
pub const TEXT: usize = 1;
pub const DATA: usize = 2;
pub const BSS: usize = 3;

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
    // holds starting addresses of each segment
    segment: Vec<Vec<u32>>,
    segment_address: Vec<u32>,
    reference: Option<&'a mut Reference<'a>>,
    label_entries: Vec<LabelEntry<'a>>,
    reloc_entries: Vec<RelocEntry>,
}

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
    let mut text_size = 0;
    let mut data_size = 0;
    let mut bss_size = 0;

    // Check if the file exists
    if fs::metadata(file_name).is_ok() {
        println!("Processing file: {}", file_name);
        let file = File::open(file_name)?;
        let mut reader = io::BufReader::new(file);
        let header = ObjectHeader::from_reader(&mut reader)?;
        println!("{:#?}", header);
        if header.magic_number == MAGIC_NUMBER {
            println!("Magic number is correct");
            let mut file_type: FileType = FileType {
                filename: file_name,
                file_header: header,
                segment: {
                    let mut segment_vec = Vec::new();
                    // text
                    segment_vec.push(Vec::with_capacity(header.text_seg_size as usize));
                    // data
                    segment_vec.push(Vec::with_capacity(header.data_seg_size as usize));
                    // bss
                    segment_vec.push(Vec::with_capacity(header.bss_seg_size as usize));
                    segment_vec
                },
                segment_address: {
                    let mut segment_address_vec = Vec::new();
                    // text
                    segment_address_vec.push(0);
                    // data
                    segment_address_vec.push(header.text_seg_size as u32);
                    // bss
                    segment_address_vec.push( header.text_seg_size as u32 + header.data_seg_size as u32,
                    );
                    segment_address_vec
                },
                reference: None,
                label_entries: Vec::new(),
                reloc_entries: Vec::new(),
            };

            // Read the text segment
            reader.seek(io::SeekFrom::Start(24))?;
            for i in 0..header.text_seg_size {
                let value = reader.read_u32::<LittleEndian>()?;
                file_type.segment[TEXT].push(value);
                println!("text seg found {}", value);
            }
            println!("text vec segment size {}", file_type.segment[TEXT].len());


            // Read the data segment
            reader.seek(io::SeekFrom::Start(24 + header.text_seg_size as u64))?;
            for i in 0..header.data_seg_size {
                let value = reader.read_u32::<LittleEndian>()?;
                file_type.segment[DATA].push(value);
                println!("data seg found {}", value);
            }
            println!("data vec segment size {}", file_type.segment[DATA].len());

            //Increment the size counters
            text_size += header.text_seg_size;
            data_size += header.data_seg_size;
            bss_size += header.bss_seg_size;

            let num_reloc_entries = header.num_references;
            reader.seek(io::SeekFrom::Start(24 + header.text_seg_size as u64 + header.data_seg_size as u64))?;
            for i in 0..num_reloc_entries {
                let read_address = reader.read_u32::<LittleEndian>()?;
                let sym_ptr = reader.read_u32::<LittleEndian>()?;
                let read_ref_type = reader.read_u8::<LittleEndian>()?;
                let sym_ptr = reader.read_u8::<LittleEndian>()?;
                file_type.reloc_entries.push(
                    RelocEntry {
                        address : read_address,
                        symbol_ptr : sym_ptr,
                        ref_type : let  match {

                        }

                    }
                );
                println!("data seg found {}", value);
                */
            }

            if disassemble {}
        } else {
            println!("Magic number is not correct");
        }
    } else {
        eprintln!("Error: File '{}' does not exist.", file_name);
    }
    Ok(())
}
