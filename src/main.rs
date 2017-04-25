extern crate byteorder;

use std::fs::File;
use std::io::Read;
use std::io::Cursor;

use byteorder::{BigEndian, ReadBytesExt};

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

    let mut reg = [0u32; 8];
    let mut memory: Vec<Vec<u32>> = vec![];
    memory.push(program);
    let mut pc = 0;

    let mut running = true;

    while running {
        let inst = memory[0][pc as usize];
        pc += 1;

        let op = inst >> 28;
        let a = (inst >> 6) & 7;
        let b = (inst >> 3) & 7;
        let c = (inst >> 0) & 7;

        match op {
            0 => {
                if reg[c as usize] != 0 {
                    reg[a as usize] = reg[b as usize];
                }
            },
            1 => {
                reg[a as usize] = memory[reg[b as usize] as usize][reg[c as usize] as usize];
            },
            2 => {
                memory[reg[a as usize] as usize][reg[b as usize] as usize] = reg[c as usize];
            },
            3 => {
                reg[a as usize] = reg[b as usize].wrapping_add(reg[c as usize]);
            },
            4 => {
                reg[a as usize] = reg[b as usize].wrapping_mul(reg[c as usize]);
            },
            5 => {
                reg[a as usize] = reg[b as usize] / reg[c as usize];
            },
            6 => {
                reg[a as usize] = !(reg[b as usize] & reg[c as usize]);
            },
            7 => {
                running = false;
            },
            8 => {
                let vec_size = reg[c as usize] as usize;
                reg[b as usize] = memory.len() as u32;
                memory.push(vec![0u32; vec_size]);
            },
            9 => {
                memory[reg[c as usize] as usize] = vec![];
            },
            10 => { print!("{}", reg[c as usize] as u8 as char); },
            11 => { unimplemented!(); },
            12 => {
                if reg[b as usize] != 0 {
                    memory[0] = memory[reg[b as usize] as usize].clone();
                }
                pc = reg[c as usize];
            },
            13 => {
                let x = (inst >> 25) & 7;
                let num = inst & 0x01FFFFFF;
                reg[x as usize] = num;
            },
            _ => { unimplemented!(); }
        }
    }
}
