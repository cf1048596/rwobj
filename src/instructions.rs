#[derive(Clone, PartialEq, Eq)]
pub enum InsnDescriptor {
    INSN,
    IType,
    RType,
    JType,
    DIRECTIVE,
    OTHER,
}
#[derive(Clone, PartialEq, Eq)]
pub struct InsnType<'a> {
    mnemonic: Option<&'a str>,
    operands: Option<&'a str>,
    opcode: u32,
    func: u32,
    type_descriptor: InsnDescriptor,
}

pub const GPR_NAME : [&str ; 16] = [
	"$0", "$1", "$2", "$3",
	"$4", "$5", "$6", "$7",
	"$8", "$9", "$10", "$11",
	"$12", "$13", "$sp", "$ra"];

pub const SPR_NAME : [&str; 16]= [
	"$spr0", "$spr1", "$spr2", "$spr3",
	"$cctrl", "$estat", "$icount", "$ccount",
	"$evec", "$ear", "$esp", "$ers",
	"$ptable", "$rbase", "$spr14", "$spr15"];

pub const INSN_TABLE : [InsnType;84] = [
    InsnType {
        mnemonic: Some("add"),
        operands: Some("d,s,t"),
        opcode: 0x0,
        func: 0x0,
        type_descriptor: InsnDescriptor::RType,
    },
    InsnType {
        mnemonic: Some("addi"),
        operands: Some("d,s,i"),
        opcode: 0x1,
        func: 0x0,
        type_descriptor: InsnDescriptor::IType,
    },
    InsnType {
        mnemonic: Some("addu"),
        operands: Some("d,s,t"),
        opcode: 0x0,
        func: 0x1,
        type_descriptor: InsnDescriptor::RType,
    },
    InsnType {
        mnemonic: Some("addui"),
        operands: Some("d,s,i"),
        opcode: 0x1,
        func: 0x1,
        type_descriptor: InsnDescriptor::IType,
    },
    InsnType {
        mnemonic: Some("sub"),
        operands: Some("S,s,t"),
        opcode: 0x0,
        func: 0x2,
        type_descriptor: InsnDescriptor::RType,
    },
    InsnType {
        mnemonic: Some("subi"),
        operands: Some("d,s,i"),
        opcode: 0x1,
        func: 0x2,
        type_descriptor: InsnDescriptor::IType,
    },
    InsnType {
        mnemonic: Some("subu"),
        operands: Some("d,s,t"),
        opcode: 0x0,
        func: 0x3,
        type_descriptor: InsnDescriptor::RType,
    },
    InsnType {
        mnemonic: Some("subui"),
        operands: Some("d,s,i"),
        opcode: 0x1,
        func: 0x3,
        type_descriptor: InsnDescriptor::IType,
    },
    InsnType {
        mnemonic: Some("mult"),
        operands: Some("d,s,t"),
        opcode: 0x0,
        func: 0x4,
        type_descriptor: InsnDescriptor::RType,
    },
    InsnType {
        mnemonic: Some("multi"),
        operands: Some("d,s,i"),
        opcode: 0x1,
        func: 0x4,
        type_descriptor: InsnDescriptor::IType,
    },
    InsnType {
        mnemonic: Some("multu"),
        operands: Some("d,s,t"),
        opcode: 0x0,
        func: 0x5,
        type_descriptor: InsnDescriptor::RType,
    },
    InsnType {
        mnemonic: Some("multui"),
        operands: Some("d,s,i"),
        opcode: 0x1,
        func: 0x5,
        type_descriptor: InsnDescriptor::IType,
    },
    InsnType {
        mnemonic: Some("div"),
        operands: Some("d,s,t"),
        opcode: 0x0,
        func: 0x6,
        type_descriptor: InsnDescriptor::RType,
    },
    InsnType {
        mnemonic: Some("divi"),
        operands: Some("d,s,i"),
        opcode: 0x1,
        func: 0x6,
        type_descriptor: InsnDescriptor::IType,
    },
    InsnType {
        mnemonic: Some("divu"),
        operands: Some("d,s,t"),
        opcode: 0x0,
        func: 0x7,
        type_descriptor: InsnDescriptor::RType,
    },
    InsnType {
        mnemonic: Some("divui"),
        operands: Some("d,s,i"),
        opcode: 0x1,
        func: 0x7,
        type_descriptor: InsnDescriptor::IType,
    },
    InsnType {
        mnemonic: Some("rem"),
        operands: Some("d,s,t"),
        opcode: 0x0,
        func: 0x8,
        type_descriptor: InsnDescriptor::RType,
    },
    InsnType {
        mnemonic: Some("remi"),
        operands: Some("d,s,i"),
        opcode: 0x1,
        func: 0x8,
        type_descriptor: InsnDescriptor::IType,
    },
    InsnType {
        mnemonic: Some("remu"),
        operands: Some("d,s,t"),
        opcode: 0x0,
        func: 0x9,
        type_descriptor: InsnDescriptor::RType,
    },
    InsnType {
        mnemonic: Some("remui"),
        operands: Some("d,s,i"),
        opcode: 0x1,
        func: 0x9,
        type_descriptor: InsnDescriptor::IType,
    },
    InsnType {
        mnemonic: Some("lhi"),
        operands: Some("d,i"),
        opcode: 0x3,
        func: 0xe,
        type_descriptor: InsnDescriptor::IType,
    },
    InsnType {
        mnemonic: Some("la"),
        operands: Some("d,j"),
        opcode: 0xc,
        func: 0x0,
        type_descriptor: InsnDescriptor::JType,
    },
    // Bitwise instructions
    InsnType {
        mnemonic: Some("and"),
        operands: Some("d,s,t"),
        opcode: 0x0,
        func: 0xb,
        type_descriptor: InsnDescriptor::RType,
    },
    InsnType {
        mnemonic: Some("andi"),
        operands: Some("d,s,i"),
        opcode: 0x1,
        func: 0xb,
        type_descriptor: InsnDescriptor::IType,
    },
    InsnType {
        mnemonic: Some("or"),
        operands: Some("d,s,t"),
        opcode: 0x0,
        func: 0xd,
        type_descriptor: InsnDescriptor::RType,
    },
    InsnType {
        mnemonic: Some("ori"),
        operands: Some("d,s,i"),
        opcode: 0x1,
        func: 0xd,
        type_descriptor: InsnDescriptor::IType,
    },
    InsnType {
        mnemonic: Some("xor"),
        operands: Some("d,s,t"),
        opcode: 0x0,
        func: 0xf,
        type_descriptor: InsnDescriptor::RType,
    },
    InsnType {
        mnemonic: Some("xori"),
        operands: Some("d,s,i"),
        opcode: 0x1,
        func: 0xf,
        type_descriptor: InsnDescriptor::IType,
    },
    InsnType {
        mnemonic: Some("sll"),
        operands: Some("d,s,t"),
        opcode: 0x0,
        func: 0xa,
        type_descriptor: InsnDescriptor::RType,
    },
    InsnType {
        mnemonic: Some("slli"),
        operands: Some("d,s,i"),
        opcode: 0x1,
        func: 0xa,
        type_descriptor: InsnDescriptor::IType,
    },
    InsnType {
        mnemonic: Some("srl"),
        operands: Some("d,s,t"),
        opcode: 0x0,
        func: 0xc,
        type_descriptor: InsnDescriptor::RType,
    },
    InsnType {
        mnemonic: Some("srli"),
        operands: Some("d,s,i"),
        opcode: 0x1,
        func: 0xc,
        type_descriptor: InsnDescriptor::IType,
    },
    InsnType {
        mnemonic: Some("sra"),
        operands: Some("d,s,t"),
        opcode: 0x0,
        func: 0xe,
        type_descriptor: InsnDescriptor::RType,
    },
    InsnType {
        mnemonic: Some("srai"),
        operands: Some("d,s,i"),
        opcode: 0x1,
        func: 0xe,
        type_descriptor: InsnDescriptor::IType,
    },
    // Test instructions
    InsnType {
        mnemonic: Some("slt"),
        operands: Some("d,s,t"),
        opcode: 0x2,
        func: 0x0,
        type_descriptor: InsnDescriptor::RType,
    },
    InsnType {
        mnemonic: Some("slti"),
        operands: Some("d,s,i"),
        opcode: 0x3,
        func: 0x0,
        type_descriptor: InsnDescriptor::IType,
    },
    InsnType {
        mnemonic: Some("sltu"),
        operands: Some("d,s,t"),
        opcode: 0x2,
        func: 0x1,
        type_descriptor: InsnDescriptor::RType,
    },
    InsnType {
        mnemonic: Some("sltui"),
        operands: Some("d,s,i"),
        opcode: 0x3,
        func: 0x1,
        type_descriptor: InsnDescriptor::IType,
    },
    InsnType {
        mnemonic: Some("sgt"),
        operands: Some("d,s,t"),
        opcode: 0x2,
        func: 0x2,
        type_descriptor: InsnDescriptor::RType,
    },
    InsnType {
        mnemonic: Some("sgti"),
        operands: Some("d,s,i"),
        opcode: 0x3,
        func: 0x2,
        type_descriptor: InsnDescriptor::IType,
    },
    InsnType {
        mnemonic: Some("sgtu"),
        operands: Some("d,s,t"),
        opcode: 0x2,
        func: 0x3,
        type_descriptor: InsnDescriptor::RType,
    },
    InsnType {
        mnemonic: Some("sgtui"),
        operands: Some("d,s,i"),
        opcode: 0x3,
        func: 0x3,
        type_descriptor: InsnDescriptor::IType,
    },
    InsnType {
        mnemonic: Some("sle"),
        operands: Some("d,s,t"),
        opcode: 0x2,
        func: 0x4,
        type_descriptor: InsnDescriptor::RType,
    },
    InsnType {
        mnemonic: Some("slei"),
        operands: Some("d,s,i"),
        opcode: 0x3,
        func: 0x4,
        type_descriptor: InsnDescriptor::IType,
    },
    InsnType {
        mnemonic: Some("sleu"),
        operands: Some("d,s,t"),
        opcode: 0x2,
        func: 0x5,
        type_descriptor: InsnDescriptor::RType,
    },
    InsnType {
        mnemonic: Some("sleui"),
        operands: Some("d,s,i"),
        opcode: 0x3,
        func: 0x5,
        type_descriptor: InsnDescriptor::IType,
    },
    InsnType {
        mnemonic: Some("sge"),
        operands: Some("d,s,t"),
        opcode: 0x2,
        func: 0x6,
        type_descriptor: InsnDescriptor::RType,
    },
    InsnType {
        mnemonic: Some("sgei"),
        operands: Some("d,s,i"),
        opcode: 0x3,
        func: 0x6,
        type_descriptor: InsnDescriptor::IType,
    },
    InsnType {
        mnemonic: Some("sgeu"),
        operands: Some("d,s,t"),
        opcode: 0x2,
        func: 0x7,
        type_descriptor: InsnDescriptor::RType,
    },
    InsnType {
        mnemonic: Some("sgeui"),
        operands: Some("d,s,i"),
        opcode: 0x3,
        func: 0x7,
        type_descriptor: InsnDescriptor::IType,
    },
    InsnType {
        mnemonic: Some("seq"),
        operands: Some("d,s,t"),
        opcode: 0x2,
        func: 0x8,
        type_descriptor: InsnDescriptor::RType,
    },
    InsnType {
        mnemonic: Some("seqi"),
        operands: Some("d,s,i"),
        opcode: 0x3,
        func: 0x8,
        type_descriptor: InsnDescriptor::IType,
    },
    InsnType {
        mnemonic: Some("sequ"),
        operands: Some("d,s,t"),
        opcode: 0x2,
        func: 0x9,
        type_descriptor: InsnDescriptor::RType,
    },
    InsnType {
        mnemonic: Some("sequi"),
        operands: Some("d,s,i"),
        opcode: 0x3,
        func: 0x9,
        type_descriptor: InsnDescriptor::IType,
    },
    InsnType {
        mnemonic: Some("sne"),
        operands: Some("d,s,t"),
        opcode: 0x2,
        func: 0xa,
        type_descriptor: InsnDescriptor::RType,
    },
    InsnType {
        mnemonic: Some("snei"),
        operands: Some("d,s,i"),
        opcode: 0x3,
        func: 0xa,
        type_descriptor: InsnDescriptor::IType,
    },
    InsnType {
        mnemonic: Some("sneu"),
        operands: Some("d,s,t"),
        opcode: 0x2,
        func: 0xb,
        type_descriptor: InsnDescriptor::RType,
    },
    InsnType {
        mnemonic: Some("sneui"),
        operands: Some("d,s,i"),
        opcode: 0x3,
        func: 0xb,
        type_descriptor: InsnDescriptor::IType,
    },
    // Branch instructions
    InsnType {
        mnemonic: Some("j"),
        operands: Some("j"),
        opcode: 0x4,
        func: 0x0,
        type_descriptor: InsnDescriptor::JType,
    },
    InsnType {
        mnemonic: Some("jr"),
        operands: Some("s"),
        opcode: 0x5,
        func: 0x0,
        type_descriptor: InsnDescriptor::JType,
    },
    InsnType {
        mnemonic: Some("jal"),
        operands: Some("j"),
        opcode: 0x6,
        func: 0x0,
        type_descriptor: InsnDescriptor::JType,
    },
    InsnType {
        mnemonic: Some("jalr"),
        operands: Some("s"),
        opcode: 0x7,
        func: 0x0,
        type_descriptor: InsnDescriptor::JType,
    },
    InsnType {
        mnemonic: Some("beqz"),
        operands: Some("s,b"),
        opcode: 0xa,
        func: 0x0,
        type_descriptor: InsnDescriptor::JType,
    },
    InsnType {
        mnemonic: Some("bnez"),
        operands: Some("s,b"),
        opcode: 0xb,
        func: 0x0,
        type_descriptor: InsnDescriptor::JType,
    },
    // Memory instructions
    InsnType {
        mnemonic: Some("lw"),
        operands: Some("d,o(s)"),
        opcode: 0x8,
        func: 0x0,
        type_descriptor: InsnDescriptor::JType,
    },
    InsnType {
        mnemonic: Some("sw"),
        operands: Some("d,o(s)"),
        opcode: 0x9,
        func: 0x0,
        type_descriptor: InsnDescriptor::JType,
    },
    // Special instructions
    InsnType {
        mnemonic: Some("movgs"),
        operands: Some("D,s"),
        opcode: 0x3,
        func: 0xc,
        type_descriptor: InsnDescriptor::IType,
    },
    InsnType {
        mnemonic: Some("movsg"),
        operands: Some("d,S"),
        opcode: 0x3,
        func: 0xd,
        type_descriptor: InsnDescriptor::IType,
    },
    InsnType {
        mnemonic: Some("break"),
        operands: Some(""),
        opcode: 0x2,
        func: 0xc,
        type_descriptor: InsnDescriptor::IType,
    },
    InsnType {
        mnemonic: Some("syscall"),
        operands: Some(""),
        opcode: 0x2,
        func: 0xd,
        type_descriptor: InsnDescriptor::IType,
    },
    InsnType {
        mnemonic: Some("rfe"),
        operands: Some(""),
        opcode: 0x2,
        func: 0xe,
        type_descriptor: InsnDescriptor::IType,
    },
    InsnType {
        mnemonic: Some(".word"),
        operands: None,
        opcode: 0xfff,
        func: 0xfff,
        type_descriptor: InsnDescriptor::DIRECTIVE,
    },
    InsnType {
        mnemonic: Some(".ascii"),
        operands: None,
        opcode: 0xfff,
        func: 0xfff,
        type_descriptor: InsnDescriptor::DIRECTIVE,
    },
    InsnType {
        mnemonic: Some(".asciiz"),
        operands: None,
        opcode: 0xfff,
        func: 0xfff,
        type_descriptor: InsnDescriptor::DIRECTIVE,
    },
    InsnType {
        mnemonic: Some(".space"),
        operands: None,
        opcode: 0xfff,
        func: 0xfff,
        type_descriptor: InsnDescriptor::DIRECTIVE,
    },
    InsnType {
        mnemonic: Some(".equ"),
        operands: None,
        opcode: 0xfff,
        func: 0xfff,
        type_descriptor: InsnDescriptor::DIRECTIVE,
    },
    InsnType {
        mnemonic: Some(".global"),
        operands: None,
        opcode: 0xfff,
        func: 0xfff,
        type_descriptor: InsnDescriptor::DIRECTIVE,
    },
    InsnType {
        mnemonic: Some(".extern"),
        operands: None,
        opcode: 0xfff,
        func: 0xfff,
        type_descriptor: InsnDescriptor::DIRECTIVE,
    },
    InsnType {
        mnemonic: Some(".data"),
        operands: None,
        opcode: 0xfff,
        func: 0xfff,
        type_descriptor: InsnDescriptor::DIRECTIVE,
    },
    InsnType {
        mnemonic: Some(".text"),
        operands: None,
        opcode: 0xfff,
        func: 0xfff,
        type_descriptor: InsnDescriptor::DIRECTIVE,
    },
    InsnType {
        mnemonic: Some(".bss"),
        operands: None,
        opcode: 0xfff,
        func: 0xfff,
        type_descriptor: InsnDescriptor::DIRECTIVE,
    },
    InsnType {
        mnemonic: Some(".frame"),
        operands: None,
        opcode: 0xfff,
        func: 0xfff,
        type_descriptor: InsnDescriptor::DIRECTIVE,
    },
    InsnType {
        mnemonic: Some(".mask"),
        operands: None,
        opcode: 0xfff,
        func: 0xfff,
        type_descriptor: InsnDescriptor::DIRECTIVE,
    },
    InsnType {
        mnemonic: None,
        operands: None,
        opcode: 0,
        func: 0,
        type_descriptor: InsnDescriptor::DIRECTIVE,
    },
];
fn disassemble(insn_address : u32, instruction : u32) -> () {
    let opcode : u32 = (instruction >> 28) & 0xf;
    let func : u32 = (instruction >> 16) & 0xf;
    let rd : u32 = (instruction >> 24) & 0xf;
    let rs : u32 = (instruction >> 20) & 0xf;
    let rt : u32 = (instruction & 0xf);
    let address : u32 = instruction & 0xfffff;
    let immediate : u32 = instruction & 0xffff;
    let signed_address :i32 = if (address & 0x80000) != 0 {
        (0xfff00000 | address).try_into().unwrap()
    } else {
        address.try_into().unwrap()
    };
    let insn_vec = INSN_TABLE.to_vec();
    let gpr_vec = GPR_NAME.to_vec();
    let spr_vec = SPR_NAME.to_vec();

    if let Some(insn_idx) = insn_vec.iter().position(|insn| {
        insn.mnemonic.is_some() && 
        insn.opcode == opcode && 
        (insn.type_descriptor == InsnDescriptor::JType || insn.func == func)
    }) {
        print!("Found a match at insn_idx: {}", insn_idx);
        let result = format!(":\t{}", INSN_TABLE[insn_idx].mnemonic.expect("real string (real)"));
        print!("{}", result);
        for ch  in INSN_TABLE[insn_idx].operands.unwrap().chars() {
            match ch {
                'd' => {
                    print!("{}", gpr_vec.get(rd as usize).unwrap())
                }

                's' => {
                    print!("{}", gpr_vec.get(rs as usize).unwrap())
                }
                'D' => {
                    print!("{}", spr_vec.get(rd as usize).unwrap())
                }
                'S' => {
                    print!("{}", spr_vec.get(rs as usize).unwrap())
                }
                't' => {
                    print!("{}", gpr_vec.get(rt as usize).unwrap())
                }
                'o' => {
                    match address {
                        0 => print!("{}", 0),
                        _ => {
                            if rs!=0 {
                                print!("{}", signed_address);
                            } else {
                                let formatted = format!("0x{:05X}", address);
                                println!("{}", formatted);
                            }
                        }
                    }
                }
                'b' => {
                    let address_i_think = (((insn_address as i32 + signed_address) & 0xfffff) + 1) as u32;
                    let formatted = format!("0x{:05X}", address_i_think);
                    println!("{}", formatted);
                }
                'i' => {
                    print!("0x{:04x}", immediate);

                }
                'j' => {

                    print!("0x{:05}", address);
                }
                _ => {
                    print!("{}", ch);

                }
            }
        }

    } else {
        println!("No match found.");
    }
}



fn disassemble_view(insn_address : u32, instruction : u32, label_name : Option<&str>) -> () {
    let opcode : u32 = (instruction >> 28) & 0xf;
    let func : u32 = (instruction >> 16) & 0xf;
    let rd : u32 = (instruction >> 24) & 0xf;
    let rs : u32 = (instruction >> 20) & 0xf;
    let rt : u32 = (instruction & 0xf);
    let address : u32 = instruction & 0xfffff;
    let immediate : u32 = instruction & 0xffff;
    let signed_address :i32 = if (address & 0x80000) != 0 {
        (0xfff00000 | address).try_into().unwrap()
    } else {
        address.try_into().unwrap()
    };
    let insn_vec = INSN_TABLE.to_vec();
    let gpr_vec = GPR_NAME.to_vec();
    let spr_vec = SPR_NAME.to_vec();

    if let Some(insn_idx) = insn_vec.iter().position(|insn| {
        insn.mnemonic.is_some() && 
        insn.opcode == opcode && 
        (insn.type_descriptor == InsnDescriptor::JType || insn.func == func)
    }) {
        print!("Found a match at insn_idx: {}", insn_idx);
        let result = format!(":\t{}", INSN_TABLE[insn_idx].mnemonic.expect("real string (real)"));
        print!("{}", result);
        for ch  in INSN_TABLE[insn_idx].operands.unwrap().chars() {
            match ch {
                'd' => {
                    print!("{}", gpr_vec.get(rd as usize).unwrap())
                }
                's' => {
                    print!("{}", gpr_vec.get(rs as usize).unwrap())
                }
                'D' => {
                    print!("{}", spr_vec.get(rd as usize).unwrap())
                }
                'S' => {
                    print!("{}", spr_vec.get(rs as usize).unwrap())
                }
                't' => {
                    print!("{}", gpr_vec.get(rt as usize).unwrap())
                }
                'i' => {
                    print!("0x{:04x}", immediate);
                }
                'j' | 'o' | 'b' => {
                    match label_name {
                        Some(name) => {
                            print!("{}", name);
                        }
                        None => {
                            let formatted = format!("0x{:05X}", address);
                            println!("{}", formatted);
                        }
                    }
                }
                _ => {
                    print!("{}", ch);
                }
            }
        }
    } else {
        println!("No match found.");
    }
}
