pub struct Font(pub [[u8; 5]; 16]);

// chip8 fonts are 4x5 pixels and go from 0 to f in hexadecimal
impl Font {
    pub fn new() -> Self {
        Font([
            // each byte corresponds to 1 row of each character,
            // with 4 bits of padding to the right
            [0xf0, 0x90, 0x90, 0x90, 0xf0], // 0
            [0x20, 0x60, 0x20, 0x20, 0x70], // 1
            [0xf0, 0x10, 0xf0, 0x80, 0xf0], // 2
            [0xf0, 0x10, 0xf0, 0x10, 0xf0], // 3
            [0x90, 0x90, 0xf0, 0x10, 0x10], // 4
            [0xf0, 0x80, 0xf0, 0x10, 0xf0], // 5
            [0xf0, 0x80, 0xf0, 0x90, 0xf0], // 6
            [0xf0, 0x10, 0x20, 0x40, 0x40], // 7
            [0xF0, 0x90, 0xF0, 0x90, 0xF0], // 8
            [0xf0, 0x90, 0xf0, 0x10, 0xf0], // 9
            [0xf0, 0x90, 0xf0, 0x90, 0x90], // A
            [0xe0, 0x90, 0xe0, 0x90, 0xe0], // B
            [0xf0, 0x80, 0x80, 0x80, 0xf0], // C
            [0xe0, 0x90, 0x90, 0x90, 0xe0], // D
            [0xf0, 0x80, 0xf0, 0x80, 0xf0], // E
            [0xf0, 0x80, 0xf0, 0x80, 0x80], // F
        ])
    }
}
