use crate::tools::BitWriter;

struct PredictorBoard {
    board: [u8; 81],
}

impl PredictorBoard {
    fn possibilities(&self, i: u8) -> Vec<u8> {
        if self.board[i as usize] != 0 {
            return vec![self.board[i as usize]];
        }

        let row = (i / 9) * 9;
        let col = i % 9;
        let block = (row / 27) * 27 + col / 3 * 3;

        fn mask (val: u8) -> u16 {
            if val == 0 { return 0 };
            1 << (val - 1)
        }

        let mut out_mask: u16 = 0;

        for j in 0..9 {
            let col_val = self.board[(col + j * 9) as usize];
            let row_val = self.board[(row + j) as usize];

            let block_row = j / 3;
            let block_col = i % 3;
            let block_val = self.board[(block + block_col + block_row * 9) as usize];

            out_mask |= mask(col_val) | mask(row_val) | mask(block_val);
        }

        (1..10).into_iter().filter(|&x| (out_mask & mask(x)) == 0).collect()
    }

    fn set(&mut self, i: u8, val: u8) {
        self.board[i as usize] = val;
    }
}

pub fn pattern_linear(i: u8) -> u8 {
    i
}

pub fn pattern_c2(i: u8) -> u8 {
    i * 2 % 81
}

pub fn pattern_c4(i: u8) -> u8 {
    i * 4 % 81
}

pub fn pattern_c8(i: u8) -> u8 {
    ((i as u32 * 8) % 81) as u8
}

pub fn pattern_c16(i: u8) -> u8 {
    ((i as u32 * 16) % 81) as u8
}

pub fn encode(string: &str) -> Vec<u8> {
    let nums: Vec<u8> = string.chars().map( |x| x.to_digit(10).unwrap() as u8).collect();

    let mut board = PredictorBoard {
        board: [0u8; 81]
    };

    let mut writer = BitWriter::new();

    for i in 0..81 {
        let real_i = pattern_linear(i);
        let num = nums[real_i as usize];
        let probabilies = board.possibilities(real_i);
        let idx = probabilies.iter().position(|&x| x == num).expect("Impossible puzzle to encode") as u8;

        match probabilies.len() {
            9 => writer.write(4, idx),
            5..=8 => writer.write(3, idx),
            3 | 4 => writer.write(2, idx),
            2 => writer.write(1, idx),
            1 => {},
            _ => panic!("Encoding of broken puzzle!")
        }

        board.set(real_i, num);
    }

    writer.disolve()
}

pub fn decode(coded: Vec<u8>) -> String {
    "a".to_string()
}
