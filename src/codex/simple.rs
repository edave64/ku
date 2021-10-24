use std::vec::IntoIter;
use std::iter::Peekable;

#[allow(dead_code)]
pub fn encode(string: &str) -> Vec<u8> {
    let nums: Vec<u8> = string.chars().map( |x| x.to_digit(10).unwrap() as u8).collect();
    let mut iter = nums.into_iter().peekable();

    let mut ret: Vec<u8> = vec![];

    loop {
        let mut outnum;
        let num1 = encode_single(&mut iter);
        if num1 == None {
            break;
        } else {
            outnum = num1.unwrap() << 4;
        }

        let num2 = encode_single(&mut iter);
        if let Some(num2) = num2 {
            outnum |= num2;
        }
        ret.push(outnum);
    }
    ret
}

fn encode_single(iter: &mut Peekable<IntoIter<u8>>) -> Option<u8> {
    let num = iter.next();
    match num {
        None => None,
        Some(x) => {
            if x == 0 {
                let mut zerocount = 1;
                loop {
                    let peek = iter.peek();
                    if let Some(y) = peek {
                        if y.to_be() == 0 {
                            zerocount += 1;
                            iter.next();
                            if zerocount == 7 {
                                break;
                            }
                            continue;
                        }
                    }
                    break;
                }
                match zerocount {
                    1 => Some(0),
                    x => Some(9 + x - 1)
                }
            } else {
                Some(x)
            }
        }
    }
}

#[allow(dead_code)]
pub fn trivial_encode(string: &str) -> Vec<u8> {
    let nums: Vec<u8> = string.chars().map( |x| x.to_digit(10).unwrap() as u8).collect();
    nums.chunks(2).map( { |pair|
        match pair {
            [x, y] => (x << 4 | y),
            [x] => (x << 4),
            _ => panic!("Toast"),
        }
    } ).collect()
}

pub fn decode(coded: Vec<u8>) -> String {
    let mut ret = String::with_capacity(81);

    fn push_nibble (ret: &mut String, nibble: u8) {
        match nibble {
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 => ret.push_str(&*format!("{}", nibble)),
            x => ret.push_str(&*format!("{}", "0".repeat((x - 8) as usize)))
        }
    }

    for byte in coded {
        let first = (byte & 0xF0) >> 4;
        let second = byte & 0x0F;

        push_nibble(&mut ret, first);
        if ret.len() >= 81 { break }
        push_nibble(&mut ret,second);
    }

    ret
}
