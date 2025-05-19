#[derive(Debug)]
pub enum Op {
    Cls,
    Rts,
    Jump(u16),
    Call(u16),
    SkipEqLit { v: u8, lit: u8 },
    SkipNeLit { v: u8, lit: u8 },
    SkipEq { v: u8, v2: u8 },
    MviLit { v: u8, lit: u8 },
    AdiLit { v: u8, lit: u8 },
    Mov { v: u8, v2: u8 },
    Or { v: u8, v2: u8 },
    And { v: u8, v2: u8 },
    Xor { v: u8, v2: u8 },
    Add { v: u8, v2: u8 },
    Sub { v: u8, v2: u8 },
    Shr(u8),
    Subb { v: u8, v2: u8 },
    Shl(u8),
    SkipNe { v: u8, v2: u8 },
    SetI(u16),
    JumpPlusV0(u16),
    Rand { v: u8, lit: u8 },
    Draw { v: u8, v2: u8, lit: u8 },
    SkipKey(u8),
    SkipNoKey(u8),
    GetDelay(u8),
    GetKey(u8),
    Delay(u8),
    Sound(u8),
    AddI(u8),
    SpriteChar(u8),
    MovBcd(u8),
    RegDump(u8),
    RegLoad(u8),
}

// Get the 12-bit address part of a 16-bit opcode.
// Example: Byte 1: 0x*X, Byte 2: 0xYY
// Returns: 0x0XYY
fn addr_from_opcode(byte1: u8, byte2: u8) -> u16 {
    let addr_high = u16::from(byte1) & 0x0f;
    let addr_low = u16::from(byte2);
    (addr_high << 8) | addr_low
}

impl Op {
    pub fn new(byte1: u8, byte2: u8) -> Result<Self, String> {
        let bad_instruction_error = Err(format!("bad instruction: {byte1:02x} {byte2:02x}"));
        
        let address = addr_from_opcode(byte1, byte2);
        let lit = byte2;
        
        let nibble1 = byte1 >> 4;
        let nibble2 = byte1 & 0x0f;
        let nibble3 = byte2 >> 4;
        let nibble4 = byte2 & 0x0f;
        
        let op = match (nibble1, nibble2, nibble3, nibble4) {
            (0x0, _, 0xe, 0x0) => Op::Cls,
            (0x0, _, 0xe, 0xe) => Op::Rts,
            (0x1, ..) => Op::Jump(address),
            (0x2, ..) => Op::Call(address),
            (0x3, v, ..)  => Op::SkipEqLit { v, lit },
            (0x4, v, ..) => Op::SkipNeLit { v, lit },
            (0x5, v, v2, _) => Op::SkipEq { v, v2 },
            (0x6, v, ..) => Op::MviLit { v, lit },
            (0x7, v, ..) => Op::AdiLit { v, lit },
            (0x8, v, v2, 0x0) => Op::Mov { v, v2 },
            (0x8, v, v2, 0x1) => Op::Or { v, v2 },
            (0x8, v, v2, 0x2) => Op::And { v, v2 },
            (0x8, v, v2, 0x3) => Op::Xor { v, v2 },
            (0x8, v, v2, 0x4) => Op::Add { v, v2 },
            (0x8, v, v2, 0x5) => Op::Sub { v, v2 },
            (0x8, v, _, 0x6) => Op::Shr(v),
            (0x8, v, v2, 0x7) => Op::Subb { v, v2 },
            (0x8, v, _, 0xe) => Op::Shl(v),
            (0x9, v, v2, ..) => Op::SkipNe { v, v2 },
            (0xa, ..) => Op::SetI(address),
            (0xb, ..) => Op::JumpPlusV0(address),
            (0xc, v, ..) => Op::Rand { v, lit },
            (0xd, v, v2, lit) => Op::Draw { v, v2, lit },
            (0xe, v, 0x9, 0xe) => Op::SkipKey(v),
            (0xe, v, 0xa, 0x1) => Op::SkipNoKey(v),
            (0xf, v, 0x0, 0x7) => Op::GetDelay(v),
            (0xf, v, 0x0, 0xa) => Op::GetKey(v),
            (0xf, v, 0x1, 0x5) => Op::Delay(v),
            (0xf, v, 0x1, 0x8) => Op::Sound(v),
            (0xf, v, 0x1, 0xe) => Op::AddI(v),
            (0xf, v, 0x2, 0x9) => Op::SpriteChar(v),
            (0xf, v, 0x3, 0x3) => Op::MovBcd(v),
            (0xf, v, 0x5, 0x5) => Op::RegDump(v),
            (0xf, v, 0x6, 0x5) => Op::RegLoad(v),
            _ => bad_instruction_error?,
        };
        Ok(op)
    }

    fn instruction(&self) -> &'static str {
        match self {
            Op::Cls => "CLS",
            Op::Rts => "RTS",
            Op::Jump(_) | Op::JumpPlusV0(_) => "JUMP",
            Op::Call(_) => "CALL",
            Op::Shr(_) => "SHR.",
            Op::Shl(_) => "SHL.",
            Op::SetI(_) | Op::MviLit { .. } => "MVI",
            Op::SkipKey(_) => "SKIP.KEY",
            Op::SkipNoKey(_) => "SKIP.NOKEY",
            Op::GetDelay(_)
            | Op::Delay(_)
            | Op::Sound(_)
            | Op::Mov { .. } => "MOV",
            Op::GetKey(_) => "WAITKEY",
            Op::AddI(_) => "ADD",
            Op::SpriteChar(_) => "SPRITECHAR",
            Op::MovBcd(_) => "MOVBCD",
            Op::RegDump(_) | Op::RegLoad(_) => "MOVM",
            Op::Or { .. } => "OR",
            Op::And { .. } => "AND",
            Op::Xor { .. } => "XOR",
            Op::Add { .. } => "ADD.",
            Op::Sub { .. } => "SUB.",
            Op::Subb { .. } => "SUBB.",
            Op::SkipNe { .. } | Op::SkipNeLit { .. } => "SKIP.NE",
            Op::SkipEqLit { .. } | Op::SkipEq { .. } => "SKIP.EQ",
            Op::AdiLit { .. } => "ADI",
            Op::Rand { .. } => "RNDMSK",
            Op::Draw { .. } => "SPRITE",
        }
    }

    pub fn disassemble(self) -> (String, String) {
        let instruction = self.instruction();
        let args = match self {
            Op::Cls | Op::Rts => String::new(),

            Op::Jump(address) | Op::Call(address) => format!("${address:03x}"),

            Op::SkipEqLit { v, lit }
            | Op::SkipNeLit { v, lit }
            | Op::MviLit { v, lit }
            | Op::AdiLit { v, lit }
            | Op::Rand { v, lit } => format!("V{v:01x}, #${lit:02x}"),

            Op::SkipEq { v, v2 }
            | Op::Mov { v, v2 }
            | Op::Or { v, v2 }
            | Op::And { v, v2 }
            | Op::Xor { v, v2 }
            | Op::Add { v, v2 }
            | Op::Sub { v, v2 }
            | Op::Subb { v, v2 }
            | Op::SkipNe { v, v2 } => format!("V{v:01x}, V{v2:01x}"),

            Op::Shr(v)
            | Op::Shl(v)
            | Op::SkipKey(v)
            | Op::SkipNoKey(v)
            | Op::GetKey(v)
            | Op::SpriteChar(v)
            | Op::MovBcd(v) => format!("V{v:01x}"),

            Op::GetDelay(v) => format!("V{v:01x}, DELAY"),
            Op::Delay(v) => format!("DELAY, V{v:01x}"),
            Op::Sound(v) => format!("SOUND, V{v:01x}"),
            Op::AddI(v) => format!("I, V{v:01x}"),
            Op::RegDump(v) => format!("(I), V0-V{v:01x}"),
            Op::RegLoad(v) => format!("V0-V{v:01x}, (I)"),
            Op::SetI(address) => format!("I, #${address:03x}"),
            Op::JumpPlusV0(address) => format!("#${address:03x}(V0)"),
            Op::Draw { v, v2, lit } => format!("V{v:01x}, V{v2:01x}, #${lit:01x}"),
        };
        (format!("{instruction:-10} "), args)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assign_op_disassemble() {
        let op = Op::new(0x81, 0xf0).unwrap();
        assert_eq!(
            (String::from("MOV        "), String::from("V1, Vf")),
            op.disassemble()
        );
    }
}
