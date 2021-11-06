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

    pub fn disolve_drop_zeros (mut self) -> Vec<u8> {
        if self.pos > 0 && self.buffer > 0 {
            self.bits.push(((self.buffer & 0xFF00) >> 8) as u8);
            self.pos = 0;
        }
        while let Some(&x) = self.bits.last() {
            if x != 0 { break; }
            self.bits.pop();
        }
        self.bits
    }
}

pub struct BitReader {
    bits: Vec<u8>,
    buffer: u16,
    pos: u8,
}

impl BitReader {
    pub fn new(mut vec: Vec<u8>) -> BitReader {
        vec.reverse();
        let buffer = (vec.pop().unwrap_or(0) as u16) << 8 | vec.pop().unwrap_or(0) as u16;
        BitReader {
            bits: vec,
            pos: 0,
            buffer,
        }
    }

    pub fn read (&mut self, length: u8) -> u8 {
        assert!(length <= 8);
        let mask = 2u16.pow(length as u32) - 1;
        let displacement = 16 - self.pos - length;
        let ret = ((self.buffer & (mask << displacement)) >> displacement) as u8;
        self.pos += length;
        if self.pos > 8 {
            self.buffer = (self.buffer << 8) | (self.bits.pop().unwrap_or(0) as u16);
            self.pos -= 8;
        }
        ret
    }
}