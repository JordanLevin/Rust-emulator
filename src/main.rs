use std::fs::File;
use std::io::prelude::*;

fn main() {

    disassemble(String::from("../roms/PONG"));
}

fn debug_print(opcode : &Vec<u8>, src: i32){
    print!("INVALID OPCODE AT {}: ", src);
    print!("{:x}", opcode[0]);
    print!("{:x}", opcode[1]);
    print!("{:x}", opcode[2]);
    println!("{:x}", opcode[3]);
}

fn transform_opcode(pc: usize, mem: &[u8; 4096]) -> Vec<u8> {
    let mut opcode: Vec<u8> = vec![0,0,0,0];
    opcode[0] = mem[pc] >> 4;
    opcode[1] = (mem[pc] << 4) >> 4;
    opcode[2] = mem[pc+1] >> 4;
    opcode[3] = (mem[pc+1] << 4) >> 4;
    return opcode;
}

#[allow(non_snake_case)]
fn disassemble(path: String) -> std::io::Result<()>{
    let mut PROGRAM: Vec<Vec<u8>> = Vec::new();
    let mut PC: u16 = 0x200;
    let mut SP: u16 = 0xEA0;
    let mut REGS: [u8; 16] = [0;16];
    let mut ADDR: u16 = 0;
    let mut STACK: Vec<[u8; 48]> = Vec::new();
    let mut DELAY_TIMER: u8 = 0;
    let mut SOUND_TIMER: u8 = 0;
    let mut MEMORY: [u8; 4096] = [0; 4096];
    let mut SCREAN: [[bool; 64]; 32] = [[false; 64]; 32];
    let mut DEBUG: u16 = 0;


    let mut input = File::open(path)?;
    let mut buffer = Vec::new();

    //read until end of file
    input.read_to_end(&mut buffer)?;

    //Setup instructions for running program
    let mut i = 0;
    while i < buffer.len(){
        //let mut opcode: Vec<u8>  vec![0,0,0,0];
        //opcode[0] = buffer[i] >> 4;
        //opcode[1] = (buffer[i] << 4) >> 4;
        //opcode[2] = buffer[i+1] >> 4;
        //opcode[3] = (buffer[i+1] << 4) >> 4;
        //PROGRAM.push(opcode);
        MEMORY[PC as usize+i] = buffer[i];
        MEMORY[PC as usize+i+1] = buffer[i+1];
        i+=2;
    }
    //println!("{:?}", MEMORY.to_vec());

    let mut addr = 0;
    loop {
        DEBUG = DEBUG+1;
        if DEBUG > 500{
            break;
        }
        //let opcode = &PROGRAM[(PC/4) as usize];
        let opcode = transform_opcode(PC as usize, &MEMORY);
        print!("{1:00$X}: ", 4, PC);
        addr += 2;
        match opcode[0] {
            0x0 => match opcode[1] {
                0x0 => match opcode[2] {
                    0xE => match opcode[3] {
                        0x0 => println!("DISPLAY CLEAR"),
                        0xE => println!("RETURN"),
                        //_ => debug_print(&opcode, 1),
                        _ => println!("CALL {:x}{:x}{:x}", opcode[1], opcode[2], opcode[3]),
                    },
                    //_ => debug_print(&opcode, 2),
                    _ => println!("CALL {:x}{:x}{:x}", opcode[1], opcode[2], opcode[3]),
                },
                _ => println!("CALL {:x}{:x}{:x}", opcode[1], opcode[2], opcode[3]),
            },
            0x1 => {
                println!("GOTO {:x}{:x}{:x}", opcode[1], opcode[2], opcode[3]);
                PC = (opcode[1] as u16) << 8;
                PC |= ((opcode[2] as u16) << 4);
                PC |= (opcode[3] as u16);
            },
                
            0x2 => println!("CALL AT ADDR {:x}{:x}{:x}", opcode[1], opcode[2], opcode[3]),
            0x3 => if REGS[opcode[1] as usize] == ((opcode[2] << 4) | opcode[3]){
                    PC = PC + 4;
                },
            0x4 => if REGS[opcode[1] as usize] != ((opcode[2] << 4) | opcode[3]){
                    PC = PC + 4;
                },
            0x5 => if REGS[opcode[1] as usize] != REGS[opcode[2] as usize]{
                    PC = PC + 4;
                },
            0x6 => REGS[opcode[1] as usize] = ((opcode[2] << 4) | opcode[3]),
            0x7 => REGS[opcode[1] as usize] =
                REGS[opcode[1] as usize] + ((opcode[2] << 4) | opcode[3]),
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
                    _ => debug_print(&opcode, 3),
                    },
            0x9 => println!("IF V{:x} == V{:x}", opcode[1], opcode[2]),
            0xA => println!("I = {:x}{:x}{:x}", opcode[1], opcode[2], opcode[3]),
            0xB => println!("PC = V0 + {:x}{:x}{:x}", opcode[1], opcode[2], opcode[3]),
            0xC => println!("V{:x} = rand()&{:x}{:x}", opcode[1], opcode[2], opcode[3]),
            0xD => println!("draw(V{:x},V{:x},{:x})", opcode[1], opcode[2], opcode[3]),
            0xE => match opcode[2]{
                0x9 => println!("IF key() == V{:x}", opcode[1]),
                0xA => println!("IF key() != V{:x}", opcode[1]),
                _ => debug_print(&opcode, 4),
            },
            0xF => match opcode[2]{
                0x0 => match opcode[3]{
                    0x7 => println!("V{:x} = get_delay()", opcode[1]),
                    0xA => println!("V{:x} = get_key()", opcode[1]),
                    _ => debug_print(&opcode, 5),
                },
                0x1 => match opcode[3]{
                    0x5 => println!("delay_timer(V{:x})", opcode[1]),
                    0x8 => println!("sound_timer(V{:x})", opcode[1]),
                    0xE => println!("I += V{:x}", opcode[1]),
                    _ => debug_print(&opcode, 6),
                },
                0x2 => println!("I=spride_addr[V{:x}]", opcode[1]),
                0x3 => println!("set_BCD(V{:x})", opcode[1]),
                0x5 => println!("reg_dump(V{:x}, &I)", opcode[1]),
                0x6 => println!("reg_load(V{:x}, &I)", opcode[1]),
                _ => debug_print(&opcode, 7),
            },
            _ => debug_print(&opcode, 8),
        }
 
        PC = PC+2;
    }

    //Return ok so the compiler doesnt complain about the result
    Ok(())
}
