extern crate byteorder;

use std::fs::File;
use std::io::Read;
use std::io::Cursor;
use std::collections::HashMap;

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
    let mut memory = HashMap::<u32, Vec<u32>>::new();
    memory.insert(0, program);
    let mut next_mem = 1;
    let mut pc = 0;

    let mut running = true;
    while running {
        let inst = memory.get(&0).unwrap()[pc as usize];
        pc += 1;

        let op = inst >> 28;
        if op == 13 {
            let a = (inst >> 25) & 7;
            let num = inst & 0x01FFFFFF;
            reg[a as usize] = num;
        } else {
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
                    let mem_slice = memory.get(&(reg[b as usize])).unwrap();
                    reg[a as usize] = mem_slice[reg[c as usize] as usize];
                },
                2 => {
                    memory.get_mut(&(reg[a as usize])).unwrap()[reg[b as usize] as usize] = reg[c as usize];
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
                    memory.insert(next_mem, vec![0; reg[c as usize] as usize]);
                    reg[b as usize] = next_mem;
                    next_mem += 1;
                },
                9 => {
                    memory.remove(&(reg[c as usize]));
                },
                10 => { print!("{}", reg[c as usize] as u8 as char); },
                11 => { unimplemented!(); },
                12 => {
                    pc = reg[c as usize];
                    if reg[b as usize] != 0 {
                        *memory.get_mut(&0).unwrap() = memory.get(&(reg[b as usize])).unwrap().clone();
                    }
                },
                _ => { unimplemented!(); }
            }
        }
    }
}
