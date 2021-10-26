pub struct BitWriter {
    bits: Vec<u8>,
    buffer: u16,
    pos: u8,
}

impl BitWriter {
    pub fn new() -> BitWriter {
        BitWriter {
            bits: Vec::new(),
            pos: 0,
            buffer: 0,
        }
    }

    pub fn write (&mut self, value: u8, length: u8) {
        self.buffer |= (value as u16) << (16 - length - self.pos);
        self.pos += length;
        if self.pos >= 8 {
            self.bits.push(((self.buffer & 0xFF00) >> 8) as u8);
            self.buffer <<= 8;
            self.pos -= 8;
        }
    }

    pub fn write_u16 (&mut self, value: u16, length: u8) {
        if length <= 8 {
            self.write((value & 0x00FF) as u8, length);
        } else {
            self.write(((value & 0xFF00) >> 8) as u8, 8);
            self.write((value & 0x00FF) as u8, length - 8);
        }
    }

    pub fn disolve (mut self) -> Vec<u8> {
        if self.pos > 0 {
            self.bits.push(((self.buffer & 0xFF00) >> 8) as u8);
            self.pos = 0;
        }
        self.bits
    }
}