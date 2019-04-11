use std::fs::File;
use std::io::prelude::*;
use std::{thread, time};
//use std::process::{Command, Stdio};
//use std::io::Write;
use std::env;

//Random vals
extern crate rand;

//Graphics
extern crate sdl2; 

mod graphics;
mod instruct;
mod disas;

fn main() {
    let (mut canvas, context) = graphics::init();
    let args: Vec<String> = env::args().collect();
    disassemble(&args[1], &mut canvas, &context);
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

fn disassemble(path: &String, canvas: &mut sdl2::render::WindowCanvas, sdl_context: &sdl2::Sdl)
    -> std::io::Result<()>{
    let mut cpu = instruct::CPU::new();
    let mut cycle_num: i64 = 0;
    cpu.init();

    //Old debugging function, new plan is to spawn another process
    //LEAVING DEBUG ALONE FOR NOW, MAYBE WORK ON IT LATER
    //thread::spawn(|| debuggui::run(&cpu));
    //let mut debug = Command::new("debug/target/debug/debug").stdin(Stdio::piped())
        //.spawn().expect("Failed to execute command");
    //let mut stdin = debug.stdin.as_mut().expect("Failed to open stdin");

    let mut input = File::open(path)?;
    let mut buffer = Vec::new();

    //read until end of file
    input.read_to_end(&mut buffer)?;

    //Setup instructions to memory for running program
    let mut i = 0;
    while i < buffer.len()-1{
        cpu.MEMORY[cpu.PC as usize+i] = buffer[i];
        cpu.MEMORY[cpu.PC as usize+i+1] = buffer[i+1];
        i+=2;
    }

    loop {
        let opcode = transform_opcode(cpu.PC as usize, &cpu.MEMORY);
        //print!("{1:00$X}: ", 4, cpu.PC);
        //print!(" V0:{:x} V1:{:x} V2:{:x} I:{:x} ", cpu.REGS[0], cpu.REGS[1], cpu.REGS[2], cpu.ADDR);
        //println!("{}", disas::opcode_to_str(&opcode));
        //println!("{:?}", cpu.KEYS);
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
            0x3 => instruct::skip_eq_i(&mut cpu, &opcode),
            0x4 => instruct::skip_neq_i(&mut cpu, &opcode),
            0x5 => instruct::skip_eq(&mut cpu, &opcode),
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
            0x9 => instruct::skip_neq(&mut cpu, &opcode),
            0xA => instruct::assign_addr(&mut cpu, &opcode),
            0xB => instruct::jump_plus(&mut cpu, &opcode),
            0xC => instruct::rand(&mut cpu, &opcode),
            0xD => instruct::draw(&mut cpu, &opcode),
            0xE => match opcode[2] {
                0x9 => instruct::skip_key_eq(&mut cpu, &opcode),
                0xA => instruct::skip_key_neq(&mut cpu, &opcode),
                _ => debug_print(&opcode, 4),
            },
            0xF => match opcode[2]{
                0x0 => match opcode[3]{
                    0x7 => instruct::get_delay(&mut cpu, &opcode),
                    0xA => instruct::get_key(&mut cpu, &opcode, sdl_context),
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

        //Count down delay and sound timers 60 times per second
        if cpu.DELAY_TIMER > 0 && cycle_num%16 == 0{
            cpu.DELAY_TIMER = cpu.DELAY_TIMER-1;
        }
        if cpu.SOUND_TIMER > 0 && cycle_num%16 == 0{
            cpu.SOUND_TIMER = cpu.SOUND_TIMER-1;
        }

        cpu.PC = cpu.PC+2;
        cycle_num = cycle_num+1;

        //Sleep for 1 ms so it doesnt go too fast, cycle time is not documented so
        //this will need to be good enough.
        let frametime = time::Duration::from_millis(1);
        thread::sleep(frametime);
        //Redraw the graphics buffer
        graphics::draw(canvas, &sdl_context, &cpu.SCREEN, &mut cpu.KEYS);

        //Try write to debugger
        //stdin.write(cpu.PC.as_bytes()).expect("Failed to write to stdin");
        //stdin.flush();
        //println!("");
    }
}
