use crate::repr::Repr;
use std::collections::BTreeSet;

pub enum Instr {
    R(RInstr),
    I(IInstr),
    S(SInstr),
    U(UInstr),
    B(BInstr),
    J(JInstr),
}

impl Instr {
    pub fn from_u32(instr: u32) -> Result<Self, &'static str> {
        match Repr(instr).opcode() {
            0b0110011 => Ok(Self::R(
                RInstr::from_u32(instr).ok_or("invalid R instruction")?,
            )),
            0b0010011 | 0b0000011 | 0b1100111 => Ok(Self::I(
                IInstr::from_u32(instr).ok_or("invalid I instruction")?,
            )),
            0b0100011 => Ok(Self::S(
                SInstr::from_u32(instr).ok_or("invalid S instruction")?,
            )),
            0b0110111 | 0b0010111 => Ok(Self::U(
                UInstr::from_u32(instr).ok_or("invalid U instruction")?,
            )),
            0b1100011 => Ok(Self::B(
                BInstr::from_u32(instr).ok_or("invalid J instruction")?,
            )),
            0b1101111 => Ok(Self::J(JInstr::from_u32(instr))),
            _ => return Err("invalid opcode"),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::R(r) => r.to_string(),
            Self::I(i) => i.to_string(),
            Self::S(s) => s.to_string(),
            Self::U(u) => u.to_string(),
            Self::B(b) => b.to_string(),
            Self::J(j) => j.to_string(),
        }
    }

    pub fn to_string_labelled(&self, curr_line: i32) -> String {
        match self {
            Self::R(r) => r.to_string(),
            Self::I(i) => i.to_string(),
            Self::S(s) => s.to_string(),
            Self::U(u) => u.to_string(),
            Self::B(b) => b.to_string_labelled(curr_line),
            Self::J(j) => j.to_string_labelled(curr_line),
        }
    }

    // for B/J instructions, returns the offset
    pub fn offset(&self) -> Option<i32> {
        match self {
            Self::B(b) => Some(b.imm13 as i32),
            Self::J(j) => Some(j.imm21),
            _ => None,
        }
    }
}

pub fn print_unlabelled(instr_list: &[Instr]) {
    for instruction in instr_list.iter() {
        println!("{}", instruction.to_string());
    }
}

pub fn print_labelled(instr_list: &[Instr]) {
    let labels: BTreeSet<_> = instr_list
        .iter()
        .enumerate()
        .flat_map(|(i, instr)| {
            let target = (i as i32) + instr.offset()? / 4;
            if target < 0 {
                None
            } else if target >= instr_list.len() as i32 {
                None
            } else {
                Some(target)
            }
        })
        .collect();

    for (i, instruction) in instr_list.iter().enumerate() {
        let i = i as i32;
        if labels.contains(&i) {
            println!("L{}:", i);
        }
        println!("  {}", instruction.to_string_labelled(i));
    }
}

pub struct RInstr {
    name: &'static str,
    rs1: u8,
    rs2: u8,
    rd: u8,
}

impl RInstr {
    fn from_u32(instr: u32) -> Option<Self> {
        let instr = Repr(instr);
        let name = match (instr.funct3(), instr.funct7()) {
            (0x0, 0x00) => "add",
            (0x0, 0x20) => "sub",
            (0x4, 0x00) => "xor",
            (0x6, 0x00) => "or",
            (0x7, 0x00) => "and",
            (0x1, 0x00) => "sll",
            (0x5, 0x00) => "srl",
            (0x5, 0x20) => "sra",
            (0x2, 0x00) => "slt",
            (0x3, 0x00) => "sltu",
            _ => return None,
        };
        Some(Self {
            name,
            rs1: instr.rs1(),
            rs2: instr.rs2(),
            rd: instr.rd(),
        })
    }

    fn to_string(&self) -> String {
        format!("{} x{}, x{}, x{}", self.name, self.rd, self.rs1, self.rs2)
    }
}

pub struct IInstr {
    name: &'static str,
    rs1: u8,
    rd: u8,
    imm12: i16,
}

impl IInstr {
    pub fn from_u32(instr: u32) -> Option<Self> {
        let instr = Repr(instr);
        let name = if instr.opcode() == 0b0010011 {
            match instr.funct3() {
                0x0 => "addi",
                0x4 => "xori",
                0x6 => "ori",
                0x7 => "andi",
                0x1 => (instr.funct7() == 0x00).then_some("slli")?,
                0x5 => match instr.funct7() & !1 {
                    0x00 => "srli",
                    0x20 => "srai",
                    _ => return None,
                },
                0x2 => "slti",
                0x3 => "sltiu",
                _ => return None,
            }
        } else if instr.opcode() == 0b0000011 {
            match instr.funct3() {
                0x0 => "lb",
                0x1 => "lh",
                0x2 => "lw",
                0x3 => "ld",
                0x4 => "lbu",
                0x5 => "lhu",
                0x6 => "lwu",
                _ => return None,
            }
        } else if instr.opcode() == 0b1100111 && instr.funct3() == 0 {
            "jalr"
        } else {
            return None;
        };
        let imm12 = if name == "sltiu" {
            instr.imm_iu()
        } else {
            instr.imm_i()
        };
        Some(Self {
            name,
            rs1: instr.rs1(),
            rd: instr.rd(),
            imm12,
        })
    }

    pub fn to_string(&self) -> String {
        if self.name.starts_with('l') {
            format!("{} x{}, {}(x{})", self.name, self.rd, self.imm12, self.rs1)
        } else {
            let imm = match self.name {
                "slli" | "srli" | "srai" => self.imm12 & 0b111111,
                _ => self.imm12,
            };
            format!("{} x{}, x{}, {}", self.name, self.rd, self.rs1, imm)
        }
    }
}

pub struct SInstr {
    name: &'static str,
    rs1: u8,
    rs2: u8,
    imm12: i16,
}

impl SInstr {
    pub fn from_u32(instr: u32) -> Option<Self> {
        let instr = Repr(instr);
        let name = match instr.funct3() {
            0x0 => "sb",
            0x1 => "sh",
            0x2 => "sw",
            0x3 => "sd",
            _ => return None,
        };
        Some(Self {
            name,
            rs1: instr.rs1(),
            rs2: instr.rs2(),
            imm12: instr.imm_s(),
        })
    }

    pub fn to_string(&self) -> String {
        format!("{} x{}, {}(x{})", self.name, self.rs2, self.imm12, self.rs1)
    }
}

pub struct UInstr {
    name: &'static str,
    rd: u8,
    imm32: i32,
}

impl UInstr {
    pub fn from_u32(instr: u32) -> Option<Self> {
        let instr = Repr(instr);
        let name = match instr.opcode() {
            0b0110111 => "lui",
            0b0010111 => "auipc",
            _ => return None,
        };
        Some(Self {
            name,
            rd: instr.rd(),
            imm32: instr.imm_u(),
        })
    }

    pub fn to_string(&self) -> String {
        format!("{} x{}, 0x{:x}", self.name, self.rd, self.imm32 >> 12)
    }
}

pub struct BInstr {
    name: &'static str,
    rs1: u8,
    rs2: u8,
    imm13: i16,
}

impl BInstr {
    pub fn from_u32(instr: u32) -> Option<Self> {
        let instr = Repr(instr);
        let name = match instr.funct3() {
            0x0 => "beq",
            0x1 => "bne",
            0x4 => "blt",
            0x5 => "bge",
            0x6 => "bltu",
            0x7 => "bgeu",
            _ => return None,
        };
        Some(Self {
            name,
            rs1: instr.rs1(),
            rs2: instr.rs2(),
            imm13: instr.imm_b(),
        })
    }

    pub fn to_string(&self) -> String {
        format!("{} x{}, x{}, {}", self.name, self.rs1, self.rs2, self.imm13)
    }

    pub fn to_string_labelled(&self, curr_line: i32) -> String {
        format!(
            "{} x{}, x{}, L{}",
            self.name,
            self.rs1,
            self.rs2,
            curr_line + self.imm13 as i32 / 4
        )
    }
}

pub struct JInstr {
    rd: u8,
    imm21: i32,
}

impl JInstr {
    pub fn from_u32(instr: u32) -> Self {
        let instr = Repr(instr);
        Self {
            rd: instr.rd(),
            imm21: instr.imm_j(),
        }
    }

    pub fn to_string(&self) -> String {
        format!("jal x{}, {}", self.rd, self.imm21)
    }

    pub fn to_string_labelled(&self, curr_line: i32) -> String {
        format!("jal x{}, L{}", self.rd, curr_line + self.imm21 / 4)
    }
}
