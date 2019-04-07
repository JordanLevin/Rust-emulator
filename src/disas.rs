use std::fs::File;
use std::io::prelude::*;
use std::io;
use std::fmt;

fn main() {
    disassemble(String::from("../roms/15PUZZLE"));
}

fn disassemble(path: String) -> std::io::Result<()>{
    let mut input = File::open(path)?;
    let mut buffer = Vec::new();
    let mut i= 0;
    let mut opcodes: Vec<Vec<u8>> = Vec::new();

    //read until end of file
    input.read_to_end(&mut buffer)?;

    while i < buffer.len() {
        let mut opcode: Vec<u8> = vec![0,0,0,0];
        opcode[0] = buffer[i] >> 4;
        opcode[1] = (buffer[i] << 4) >> 4;
        opcode[2] = buffer[i+1] >> 4;
        opcode[3] = (buffer[i+1] << 4) >> 4;
        //print!("{:x}", opcode[0]);
        //print!("{:x}", opcode[1]);
        //print!("{:x}", opcode[2]);
        //println!("{:x}", opcode[3]);
        opcodes.push(opcode);
        i+=2;
    }

    for opcode in opcodes {
        match opcode[0] {
            0x0 => match opcode[1] {
                0x0 => match opcode[2] {
                    0xE => match opcode[3] {
                        0x0 => println!("DISPLAY CLEAR"),
                        0xE => println!("RETURN"),
                        _ => {
                            print!("{:x}", opcode[0]);
                            print!("{:x}", opcode[1]);
                            print!("{:x}", opcode[2]);
                            println!("{:x}", opcode[3]);
                        },
                    },
                    _ => {
                        print!("{:x}", opcode[0]);
                        print!("{:x}", opcode[1]);
                        print!("{:x}", opcode[2]);
                        println!("{:x}", opcode[3]);
                    },
                },
                _ => println!("CALL {:x}{:x}{:x}", opcode[1], opcode[2], opcode[3]),
            },
            0x1 => println!("GOTO"),
            0x2 => println!("CALL AT ADDR"),
            0x3 => println!("IF EQ"),
            0x4 => println!("IF NOT EQ"),
            0x5 => println!("IF NOT EQ VAR"),
            0x6 => println!("SET VAR"),
            0x7 => println!("PLUS EQUALS"),
            _ => {
                print!("{:x}", opcode[0]);
                print!("{:x}", opcode[1]);
                print!("{:x}", opcode[2]);
                println!("{:x}", opcode[3]);
            },
        }
    }


    //Return ok so the compiler doesnt complain about the result
    Ok(())
}
