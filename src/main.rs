use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn main() {

    disassemble(String::from("../roms/15PUZZLE"));
}

#[allow(non_snake_case)]
fn disassemble(path: String) -> std::io::Result<()>{
    let mut PROGRAM: Vec<Vec<u8>> = Vec::new();
    let mut PC = 0;
    let mut REGS: Vec<u8> = vec!([0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]);
    let mut ADDR: u16 = 0;
    let mut STACK: Vec<[u8; 48]> = Vec::new();
    let mut DELAY_TIMER: u8 = 0;
    let mut SOUND_TIMER: u8 = 0;
    let mut MEMORY: [u8: 4096] = [0; 4096];

    let mut input = File::open(path)?;
    let mut buffer = Vec::new();

    //read until end of file
    input.read_to_end(&mut buffer)?;

    //Setup instructions for running program
    let mut i = 0;
    while i < buffer.len() {
        let mut opcode: Vec<u8> = vec![0,0,0,0];
        opcode[0] = buffer[i] >> 4;
        opcode[1] = (buffer[i] << 4) >> 4;
        opcode[2] = buffer[i+1] >> 4;
        opcode[3] = (buffer[i+1] << 4) >> 4;
        PROGRAM.push(opcode);
        i+=2;
    }

    let mut addr = 0;
    loop {
        let opcode = &PROGRAM[PC/4];
        print!("{1:00$X}: ", 4, addr);
        addr += 2;
        match opcode[0] {
            0x0 => match opcode[1] {
                0x0 => match opcode[2] {
                    0xE => match opcode[3] {
                        0x0 => println!("DISPLAY CLEAR"),
                        0xE => println!("RETURN"),
                        _ => println!("INVALID OPCODE"),
                    },
                    _ => println!("INVALID OPCODE"),
                },
                _ => println!("CALL {:x}{:x}{:x}", opcode[1], opcode[2], opcode[3]),
            },
            0x1 => println!("GOTO {:x}{:x}{:x}", opcode[1], opcode[2], opcode[3]),
            0x2 => println!("CALL AT ADDR {:x}{:x}{:x}", opcode[1], opcode[2], opcode[3]),
            0x3 => println!("IF V{:x} == {:x}{:x}", opcode[1], opcode[2], opcode[3]),
            0x4 => println!("IF V{:x} != {:x}{:x}", opcode[1], opcode[2], opcode[3]),
            0x5 => println!("IF V{:x} != V{:x}", opcode[1], opcode[2]),
            0x6 => println!("V{:x} = {:x}{:x}", opcode[1], opcode[2], opcode[3]),
            0x7 => println!("V{:x} += {:x}{:x}", opcode[1], opcode[2], opcode[3]),
            0x8 => match opcode[3] {
                    0x0 => println!("V{:x} = V{:x}", opcode[1], opcode[2]),
                    0x1 => println!("V{:x} = V{:x}|V{:x}", opcode[1], opcode[1], opcode[2]),
                    0x2 => println!("V{:x} = V{:x}&V{:x}", opcode[1], opcode[1], opcode[2]),
                    0x3 => println!("V{:x} = V{:x}^V{:x}", opcode[1], opcode[1], opcode[2]),
                    0x4 => println!("V{:x} += V{:x}", opcode[1], opcode[2]),
                    0x5 => println!("V{:x} -= V{:x}", opcode[1], opcode[2]),
                    0x6 => println!("V{:x}>>=1", opcode[1]),
                    0x7 => println!("V{:x} = V{:x}-V{:x}", opcode[1], opcode[2], opcode[1]),
                    0xe => println!("V{:x}<<=1", opcode[1]),
                    _ => println!("INVALID OPCODE"),
                    },
            0x9 => println!("IF V{:x} == V{:x}", opcode[1], opcode[2]),
            0xA => println!("I = {:x}{:x}{:x}", opcode[1], opcode[2], opcode[3]),
            0xB => println!("PC = V0 + {:x}{:x}{:x}", opcode[1], opcode[2], opcode[3]),
            0xC => println!("V{:x} = rand()&{:x}{:x}", opcode[1], opcode[2], opcode[3]),
            0xD => println!("draw(V{:x},V{:x},{:x})", opcode[1], opcode[2], opcode[3]),
            0xE => match opcode[2]{
                0x9 => println!("IF key() == V{:x}", opcode[1]),
                0xA => println!("IF key() != V{:x}", opcode[1]),
                _ => println!("INVALID OPCODE"),
            },
            0xF => match opcode[2]{
                0x0 => match opcode[3]{
                    0x7 => println!("V{:x} = get_delay()", opcode[1]),
                    0xA => println!("V{:x} = get_key()", opcode[1]),
                    _ => println!("INVALID OPCODE"),
                },
                0x1 => match opcode[3]{
                    0x5 => println!("delay_timer(V{:x})", opcode[1]),
                    0x8 => println!("sound_timer(V{:x})", opcode[1]),
                    0xE => println!("I += V{:x}", opcode[1]),
                    _ => println!("INVALID OPCODE"),
                },
                0x2 => println!("set_BCD(V{:x})", opcode[1]),
                0x5 => println!("reg_dump(V{:x}, &I)", opcode[1]),
                0x6 => println!("reg_load(V{:x}, &I)", opcode[1]),
                _ => println!("INVALID OPCODE"),
            },
            _ => {
                print!("{:x}", opcode[0]);
                print!("{:x}", opcode[1]);
                print!("{:x}", opcode[2]);
                println!("{:x}", opcode[3]);
            },
        }
        PC = PC+1;
    }

    //Return ok so the compiler doesnt complain about the result
    Ok(())
}
