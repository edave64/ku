use crate::errors::UnsolvableError;
use crate::solver::board::Board;
use crate::solver::solve::solve;
use crate::tools::{BitReader, BitWriter};
use std::error::Error;

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

        fn mask(val: u8) -> u16 {
            if val == 0 {
                return 0;
            };
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

        (1..10)
            .into_iter()
            .filter(|&x| (out_mask & mask(x)) == 0)
            .collect()
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

const HOLE_ENCODE_BITS: u8 = 3;
const HOLE_ENCODE_MAX: u8 = 2u32.pow((HOLE_ENCODE_BITS - 1) as u32) as u8;
const CHAIN_ENCODE_BITS: u8 = 1;
const CHAIN_ENCODE_MAX: u8 = 1_u8;

pub fn encode(string: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let nums: Vec<u8> = string
        .chars()
        .map(|x| x.to_digit(10).unwrap() as u8)
        .collect();
    let holes: Vec<bool> = nums.iter().map(|&num| num == 0).collect();
    let solved = solve(Board::from_puzzle(nums)?, false)?;
    if let Some(solved) = solved {
        let solved_nums = solved.to_nums();

        let mut board = PredictorBoard { board: [0u8; 81] };

        let mut writer = BitWriter::new();

        for i in 0..81 {
            let real_i = pattern_linear(i);
            let num = solved_nums[real_i as usize];
            let probabilies = board.possibilities(real_i);
            let idx = probabilies
                .iter()
                .position(|&x| x == num)
                .expect("Impossible puzzle to encode") as u8;

            match probabilies.len() {
                9 => writer.write(idx, 4),
                5..=8 => writer.write(idx, 3),
                3 | 4 => writer.write(idx, 2),
                2 => writer.write(idx, 1),
                1 => {}
                _ => panic!("Encoding of broken puzzle!"),
            }

            board.set(real_i, num);
        }

        let mut hole_length = 0;
        let mut chain_length = 0;

        for hole in holes {
            if hole {
                //writer.write(1, 1);
                if chain_length > 0 {
                    writer.write(0, CHAIN_ENCODE_BITS);
                    chain_length = 0;
                }
                hole_length += 1;
                if hole_length >= HOLE_ENCODE_MAX {
                    writer.write(
                        (hole_length - 1) | 1 << (HOLE_ENCODE_BITS - 1),
                        HOLE_ENCODE_BITS,
                    );
                    hole_length = 0;
                }
            } else {
                //writer.write(0, 1);
                if hole_length > 0 {
                    writer.write(
                        (hole_length - 1) | 1 << (HOLE_ENCODE_BITS - 1),
                        HOLE_ENCODE_BITS,
                    );
                    if hole_length < HOLE_ENCODE_MAX {
                        hole_length = 0;
                        // The hole wasn't as large as it could have been, so a the next field can't be a hole
                        continue;
                    }
                    hole_length = 0;
                }
                chain_length += 1;
                if chain_length >= CHAIN_ENCODE_MAX {
                    writer.write(0, CHAIN_ENCODE_BITS);
                    chain_length = 0;
                }
            }
        }
        if hole_length > 0 {
            writer.write(
                (hole_length - 1) | 1 << (HOLE_ENCODE_BITS - 1),
                HOLE_ENCODE_BITS,
            );
            hole_length = 0;
        }
        Ok(writer.disolve_drop_zeros())
    } else {
        Err(Box::new(UnsolvableError {}))
    }
}

pub fn decode(coded: Vec<u8>) -> String {
    let mut reader = BitReader::new(coded);

    let mut board = PredictorBoard { board: [0u8; 81] };

    for i in 0..81 {
        let real_i = pattern_linear(i);
        let possiblities = board.possibilities(real_i);
        let decoded = match possiblities.len() {
            9 => reader.read(4),
            5..=8 => reader.read(3),
            3 | 4 => reader.read(2),
            2 => reader.read(1),
            1 => 0,
            _ => panic!("Encoding of broken puzzle!"),
        } as usize;

        let num = possiblities[decoded];

        board.set(real_i, num);
    }

    let mut pos = 0;

    loop {
        let is_hole = reader.read(1) == 1;
        if is_hole {
            let hole_size = reader.read(HOLE_ENCODE_BITS - 1) + 1;
            for _ in 0..hole_size {
                board.set(pos, 0);
                pos += 1;
            }
            // The hole wasn't as large as it could have been, so a the next field can't be a hole
            if hole_size < HOLE_ENCODE_MAX {
                pos += 1;
            }
        } else {
            pos += 1;
        }
        if pos >= 81 {
            break;
        }
    }
    /*
    for pos in 0..81 {
        let is_hole = reader.read(1) == 1;
        if is_hole {
            board.set(pos, 0);
        }
    }
     */

    board.board.map(|x| format!("{}", x)).join("")
}
