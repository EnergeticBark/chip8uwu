use std::error::Error;

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

impl Op {
    pub fn new(byte1: u8, byte2: u8) -> Result<Self, Box<dyn Error>> {
        let first_nibble = byte1 >> 4;
        match first_nibble {
            0x00 => match byte2 {
                0xE0 => Ok(Op::Cls),
                0xEE => Ok(Op::Rts),
                _ => Err(format!("bad instruction: {:02x} {:02x}", byte1, byte2).into()),
            },
            0x01 => {
                let addr_high = byte1 as u16 & 0x0f;
                let addr_low = byte2 as u16;
                let address = (addr_high << 8) | addr_low;

                Ok(Op::Jump(address))
            }
            0x02 => {
                let addr_high = byte1 as u16 & 0x0f;
                let addr_low = byte2 as u16;
                let address = (addr_high << 8) | addr_low;

                Ok(Op::Call(address))
            }
            0x03 => {
                let register = byte1 & 0x0f;
                Ok(Op::SkipEqLit {
                    v: register,
                    lit: byte2,
                })
            }
            0x04 => {
                let register = byte1 & 0x0f;
                Ok(Op::SkipNeLit {
                    v: register,
                    lit: byte2,
                })
            }
            0x05 => {
                let register = byte1 & 0x0f;
                let register2 = byte2 >> 4;
                Ok(Op::SkipEq {
                    v: register,
                    v2: register2,
                })
            }
            0x06 => {
                let register = byte1 & 0x0f;
                Ok(Op::MviLit {
                    v: register,
                    lit: byte2,
                })
            }
            0x07 => {
                let register = byte1 & 0x0f;
                Ok(Op::AdiLit {
                    v: register,
                    lit: byte2,
                })
            }
            0x08 => {
                let register = byte1 & 0x0f;
                let register2 = byte2 >> 4;
                let last_nibble = byte2 & 0x0f;
                match last_nibble {
                    0x00 => Ok(Op::Mov {
                        v: register,
                        v2: register2,
                    }),
                    0x01 => Ok(Op::Or {
                        v: register,
                        v2: register2,
                    }),
                    0x02 => Ok(Op::And {
                        v: register,
                        v2: register2,
                    }),
                    0x03 => Ok(Op::Xor {
                        v: register,
                        v2: register2,
                    }),
                    0x04 => Ok(Op::Add {
                        v: register,
                        v2: register2,
                    }),
                    0x05 => Ok(Op::Sub {
                        v: register,
                        v2: register2,
                    }),
                    0x06 => Ok(Op::Shr(register)),
                    0x07 => Ok(Op::Subb {
                        v: register,
                        v2: register2,
                    }),
                    0x0e => Ok(Op::Shl(register)),
                    _ => Err(format!("bad instruction: {:02x} {:02x}", byte1, byte2).into()),
                }
            }
            0x09 => {
                let register = byte1 & 0x0f;
                let register2 = byte2 >> 4;
                Ok(Op::SkipNe {
                    v: register,
                    v2: register2,
                })
            }
            0x0a => {
                let addr_high = byte1 as u16 & 0x0f;
                let addr_low = byte2 as u16;
                let address = (addr_high << 8) | addr_low;

                Ok(Op::SetI(address))
            }
            0x0b => {
                let addr_high = byte1 as u16 & 0x0f;
                let addr_low = byte2 as u16;
                let address = (addr_high << 8) | addr_low;

                Ok(Op::JumpPlusV0(address))
            }
            0x0c => {
                let register = byte1 & 0x0f;
                Ok(Op::Rand {
                    v: register,
                    lit: byte2,
                })
            }
            0x0d => {
                let register = byte1 & 0x0f;
                let register2 = byte2 >> 4;
                let n = byte2 & 0x0f;
                Ok(Op::Draw {
                    v: register,
                    v2: register2,
                    lit: n,
                })
            }
            0x0e => {
                let register = byte1 & 0x0f;
                match byte2 {
                    0x9e => Ok(Op::SkipKey(register)),
                    0xa1 => Ok(Op::SkipNoKey(register)),
                    _ => Err(format!("bad instruction: {:02x} {:02x}", byte1, byte2).into()),
                }
            }
            0x0f => {
                let register = byte1 & 0x0f;
                match byte2 {
                    0x07 => Ok(Op::GetDelay(register)),
                    0x0A => Ok(Op::GetKey(register)),
                    0x15 => Ok(Op::Delay(register)),
                    0x18 => Ok(Op::Sound(register)),
                    0x1e => Ok(Op::AddI(register)),
                    0x29 => Ok(Op::SpriteChar(register)),
                    0x33 => Ok(Op::MovBcd(register)),
                    0x55 => Ok(Op::RegDump(register)),
                    0x65 => Ok(Op::RegLoad(register)),
                    _ => Err(format!("bad instruction: {:02x} {:02x}", byte1, byte2).into()),
                }
            }
            _ => Err(format!("bad instruction: {:02x} {:02x}", byte1, byte2).into()),
        }
    }

    fn instruction(&self) -> &'static str {
        match self {
            Op::Cls => "CLS",
            Op::Rts => "RTS",
            Op::Jump(_) => "JUMP",
            Op::Call(_) => "CALL",
            Op::Shr(_) => "SHR.",
            Op::Shl(_) => "SHL.",
            Op::SetI(_) => "MVI",
            Op::JumpPlusV0(_) => "JUMP",
            Op::SkipKey(_) => "SKIP.KEY",
            Op::SkipNoKey(_) => "SKIP.NOKEY",
            Op::GetDelay(_) => "MOV",
            Op::GetKey(_) => "WAITKEY",
            Op::Delay(_) => "MOV",
            Op::Sound(_) => "MOV",
            Op::AddI(_) => "ADD",
            Op::SpriteChar(_) => "SPRITECHAR",
            Op::MovBcd(_) => "MOVBCD",
            Op::RegDump(_) => "MOVM",
            Op::RegLoad(_) => "MOVM",
            Op::Mov { .. } => "MOV",
            Op::Or { .. } => "OR",
            Op::And { .. } => "AND",
            Op::Xor { .. } => "XOR",
            Op::Add { .. } => "ADD.",
            Op::Sub { .. } => "SUB.",
            Op::Subb { .. } => "SUBB.",
            Op::SkipNe { .. } => "SKIP.NE",
            Op::SkipEqLit { .. } => "SKIP.EQ",
            Op::SkipNeLit { .. } => "SKIP.NE",
            Op::SkipEq { .. } => "SKIP.EQ",
            Op::MviLit { .. } => "MVI",
            Op::AdiLit { .. } => "ADI",
            Op::Rand { .. } => "RNDMSK",
            Op::Draw { .. } => "SPRITE",
        }
    }

    pub fn disassemble(&self) -> (String, String) {
        let instruction = self.instruction();
        let args = match self {
            Op::Cls | Op::Rts => String::new(),

            Op::Jump(address) | Op::Call(address) => format!("${:03x}", address),

            Op::SkipEqLit { v, lit }
            | Op::SkipNeLit { v, lit }
            | Op::MviLit { v, lit }
            | Op::AdiLit { v, lit }
            | Op::Rand { v, lit } => format!("V{:01x}, #${:02x}", v, lit),

            Op::SkipEq { v, v2 }
            | Op::Mov { v, v2 }
            | Op::Or { v, v2 }
            | Op::And { v, v2 }
            | Op::Xor { v, v2 }
            | Op::Add { v, v2 }
            | Op::Sub { v, v2 }
            | Op::Subb { v, v2 }
            | Op::SkipNe { v, v2 } => format!("V{:01x}, V{:01x}", v, v2),

            Op::Shr(v)
            | Op::Shl(v)
            | Op::SkipKey(v)
            | Op::SkipNoKey(v)
            | Op::GetKey(v)
            | Op::SpriteChar(v)
            | Op::MovBcd(v) => format!("V{:01x}", v),

            Op::GetDelay(v) => format!("V{:01x}, DELAY", v),
            Op::Delay(v) => format!("DELAY, V{:01x}", v),
            Op::Sound(v) => format!("SOUND, V{:01x}", v),
            Op::AddI(v) => format!("I, V{:01x}", v),
            Op::RegDump(v) => format!("(I), V0-V{:01x}", v),
            Op::RegLoad(v) => format!("V0-V{:01x}, (I)", v),
            Op::SetI(address) => format!("I, #${:03x}", address),
            Op::JumpPlusV0(address) => format!("#${:03x}(V0)", address),
            Op::Draw { v, v2, lit } => format!("V{:01x}, V{:01x}, #${:01x}", v, v2, lit),
        };
        (format!("{:-10} ", instruction), args)
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
