extern crate byteorder;

use std::fs::File;
use std::io::Read;
use std::io::Cursor;
use std::io::stdin;

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

macro_rules! reg {
    (a, $e:expr) => ((($e >> 6) & 7) as usize);
    (b, $e:expr) => ((($e >> 3) & 7) as usize);
    (c, $e:expr) => (($e & 7) as usize);
}

fn main() {
    let mut f = File::open("sandmark.umz").unwrap();
    let mut f_data = Vec::new();
    f.read_to_end(&mut f_data).unwrap();

    let mut program = Vec::new();
    let mut cur = Cursor::new(f_data);

    loop {
        let inst = cur.read_u32::<BigEndian>();
        match inst {
            Ok(n) => { program.push(n); },
            Err(_) => { break; }
        }
    }

    let mut pc = 0;
    let mut reg = [0u32; 8];
    let mut memory: Vec<Vec<u32>> = vec![];
    memory.push(program);
    let mut outfile: Option<File> = None;

    let mut running = true;
    while running {
        let inst = memory[0][pc as usize];

        let op = inst >> 28;

        //println!("[{}] {} {:?}: {}, {}, {}", pc, op, reg, a, b, c);

        pc += 1;
        match op {
            0 => {
                if reg[reg!(c, inst)] != 0 {
                    reg[reg!(a, inst)] = reg[reg!(b, inst)];
                }
            },
            1 => {
                reg[reg!(a, inst)] = memory[reg[reg!(b, inst)] as usize][reg[reg!(c, inst)] as usize];
            },
            2 => {
                /*if (reg[a as usize] == 0) {
                    println!("Self-modifiying instruction {}", reg[b as usize]);
                }*/

                memory[reg[reg!(a, inst)] as usize][reg[reg!(b, inst)] as usize] = reg[reg!(c, inst)];
            },
            3 => {
                reg[reg!(a, inst)] = reg[reg!(b, inst)].wrapping_add(reg[reg!(c, inst)]);
            },
            4 => {
                reg[reg!(a, inst)] = reg[reg!(b, inst)].wrapping_mul(reg[reg!(c, inst)]);
            },
            5 => {
                reg[reg!(a, inst)] = reg[reg!(b, inst)].wrapping_div(reg[reg!(c, inst)]);
            },
            6 => {
                reg[reg!(a, inst)] = !(reg[reg!(b, inst)] & reg[reg!(c, inst)]);
            },
            7 => {
                running = false;
            },
            8 => {
                let vec_size = reg[reg!(c, inst)] as usize;
                reg[reg!(b, inst)] = memory.len() as u32;
                memory.push(vec![0u32; vec_size]);
            },
            9 => {
                memory[reg[reg!(c, inst)] as usize] = vec![];
            },
            10 => {
                let ch = reg[reg!(c, inst)] as u8;
                match outfile {
                    Some(ref mut file) => {
                        file.write_u8(ch).unwrap();
                    }
                    None => {
                        print!("{}", ch as char);
                    }
                }
            },
            11 => {
                match stdin().bytes().next() {
                    Some(x) => {
                        let ch = x.unwrap();

                        reg[reg!(c, inst)] = (ch as u32) & 0xFF;
                    }
                    None => { reg[reg!(c, inst)] = 0xFFFFFFFF; }
                }
            },
            12 => {
                if reg[reg!(b, inst)] != 0 {
                    memory[0] = memory[reg[reg!(b, inst)] as usize].clone();
                }

                pc = reg[reg!(c, inst)];
            },
            13 => {
                let x = ((inst >> 25) & 7) as usize;
                let num = inst & 0x01FFFFFF;
                reg[x] = num;
            },
            _ => { unimplemented!(); }
        }
    }
}
