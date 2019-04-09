use std::fs::File;
use std::io::prelude::*;
use std::{thread, time};
use std::process::Command;
//use std::thread;

//Random vals
extern crate rand;

//Graphics
extern crate sdl2; 

mod graphics;
mod debuggui;
use debuggui::instruct;

fn main() {
    let (mut canvas, context) = graphics::init();
    disassemble(String::from("../roms/PONG"), &mut canvas, &context);
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

fn disassemble(path: String, canvas: &mut sdl2::render::WindowCanvas, sdl_context: &sdl2::Sdl)
    -> std::io::Result<()>{
    let mut cpu = instruct::CPU::new();
    //Old debugging function, new plan is to spawn another process
    let output = Command::new("./debug").expect("Failed to execute command");
    //thread::spawn(|| debuggui::run(&cpu));

    let mut input = File::open(path)?;
    let mut buffer = Vec::new();

    //read until end of file
    input.read_to_end(&mut buffer)?;

    //Setup instructions to memory for running program
    let mut i = 0;
    while i < buffer.len(){
        cpu.MEMORY[cpu.PC as usize+i] = buffer[i];
        cpu.MEMORY[cpu.PC as usize+i+1] = buffer[i+1];
        i+=2;
    }

    loop {
        let opcode = transform_opcode(cpu.PC as usize, &cpu.MEMORY);
        //print!("{1:00$X}: ", 4, PC);
        match opcode[0] {
            0x0 => match opcode[1] {
                0x0 => match opcode[2] {
                    0xE => match opcode[3] {
                        0x0 => instruct::clear_screen(&mut cpu, &opcode),
                        0xE => instruct::ret(&mut cpu, &opcode),
                        _ => print!("CALL {:x}{:x}{:x}", opcode[1], opcode[2], opcode[3]),
                    },
                    _ => print!("CALL {:x}{:x}{:x}", opcode[1], opcode[2], opcode[3]),
                },
                _ => print!("CALL {:x}{:x}{:x}", opcode[1], opcode[2], opcode[3]),
            },
            0x1 => instruct::goto(&mut cpu, &opcode),
            0x2 => instruct::func(&mut cpu, &opcode),
            0x3 => instruct::if_eq_i(&mut cpu, &opcode),
            0x4 => instruct::if_neq_i(&mut cpu, &opcode),
            0x5 => instruct::if_neq(&mut cpu, &opcode),
            0x6 => instruct::assign_i(&mut cpu, &opcode),
            0x7 => instruct::plus_eq_i(&mut cpu, &opcode),
            0x8 => match opcode[3] {
                0x0 => instruct::assign(&mut cpu, &opcode),
                0x1 => instruct::bit_or(&mut cpu, &opcode),
                0x2 => instruct::bit_and(&mut cpu, &opcode),
                0x3 => instruct::bit_xor(&mut cpu, &opcode),
                0x4 => instruct::plus_eq(&mut cpu, &opcode),
                0x5 => instruct::minus_eq(&mut cpu, &opcode),
                0x6 => instruct::shift_r(&mut cpu, &opcode),
                0x7 => instruct::minus_eq_b(&mut cpu, &opcode),
                0xe => instruct::shift_l(&mut cpu, &opcode),
                _ => debug_print(&opcode, 3),
            },
            0x9 => instruct::if_eq(&mut cpu, &opcode),
            0xA => instruct::assign_addr(&mut cpu, &opcode),
            0xB => instruct::jump_plus(&mut cpu, &opcode),
            0xC => instruct::rand(&mut cpu, &opcode),
            0xD => instruct::draw(&mut cpu, &opcode),
            0xE => match opcode[2] {
                0x9 => instruct::if_key_eq(&mut cpu, &opcode),
                0xA => instruct::if_key_neq(&mut cpu, &opcode),
                _ => debug_print(&opcode, 4),
            },
            0xF => match opcode[2]{
                0x0 => match opcode[3]{
                    0x7 => instruct::get_delay(&mut cpu, &opcode),
                    0xA => instruct::get_key(&mut cpu, &opcode),
                    _ => debug_print(&opcode, 5),
                },
                0x1 => match opcode[3]{
                    0x5 => instruct::delay_assign(&mut cpu, &opcode),
                    0x8 => instruct::sound_assign(&mut cpu, &opcode),
                    0xE => instruct::i_plus_eq(&mut cpu, &opcode),
                    _ => debug_print(&opcode, 6),
                },
                0x2 => instruct::i_sprite(&mut cpu, &opcode),
                0x3 => instruct::set_bcd(&mut cpu, &opcode),
                0x5 => instruct::reg_dump(&mut cpu, &opcode),
                0x6 => instruct::reg_load(&mut cpu, &opcode),
                _ => debug_print(&opcode, 7),
            },
            _ => debug_print(&opcode, 8),
        }

        let frametime = time::Duration::from_millis(16);
        thread::sleep(frametime);

        cpu.DELAY_TIMER = cpu.DELAY_TIMER+1;
        //REFRESH, this may need to be in the loop
        graphics::draw(canvas, sdl_context, &cpu.SCREEN);
        if cpu.DELAY_TIMER >= 60{
            cpu.DELAY_TIMER = 0;
        }
        cpu.PC = cpu.PC+2;
        //println!("");
    }
}
