pub struct State {
    pub disassembler_open: bool
}

impl State {
    pub fn new() -> Self {
        Self {
            disassembler_open: false
        }
    }
}