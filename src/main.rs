use std::fs::File;
use std::io::prelude::*;
use std::{thread, time};
use std::time::Duration;

//Random vals
extern crate rand;
use rand::Rng;

//SDL2 imports
extern crate sdl2; 
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect;
 

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
 
    let window = video_subsystem.window("CHIP8 Emulator", 640, 320)
        .position_centered()
        .build()
        .unwrap();
 
    let mut canvas = window.into_canvas().build().unwrap();
 
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    disassemble(String::from("../roms/PONG"), &mut canvas, &sdl_context);
}

fn draw(canvas: &mut sdl2::render::WindowCanvas, sdl_context: &sdl2::Sdl,
       SCREEN: &[[bool; 64]; 32]){
    let mut event_pump = sdl_context.event_pump().unwrap();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit {..} |
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                std::process::exit(0);
            },
            _ => {}
        }
    }
    // The rest of the game loop goes here...
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    for row in 0..32{
        for col in 0..64{
            if SCREEN[row][col]{
                let r = sdl2::rect::Rect::new(10*row as i32, 10*col as i32,10,10);
                canvas.fill_rect(r);
            }
        }
    }

    canvas.present();
    ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
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
fn disassemble(path: String, canvas: &mut sdl2::render::WindowCanvas, sdl_context: &sdl2::Sdl)
    -> std::io::Result<()>{
    let mut PROGRAM: Vec<Vec<u8>> = Vec::new();
    let mut PC: u16 = 0x200;
    let mut SP: u16 = 0xEA0;
    let mut REGS: [u8; 16] = [0;16];
    let mut ADDR: u16 = 0;
    let mut STACK: Vec<u16> = Vec::new();
    let mut DELAY_TIMER: u8 = 0;
    let mut SOUND_TIMER: u8 = 0;
    let mut MEMORY: [u8; 4096] = [0; 4096];
    let mut SCREEN: [[bool; 64]; 32] = [[false; 64]; 32];
    //let mut DEBUG: u16 = 0;


    let mut input = File::open(path)?;
    let mut buffer = Vec::new();

    //read until end of file
    input.read_to_end(&mut buffer)?;

    //Setup instructions to memory for running program
    let mut i = 0;
    while i < buffer.len(){
        MEMORY[PC as usize+i] = buffer[i];
        MEMORY[PC as usize+i+1] = buffer[i+1];
        i+=2;
    }

    loop {
        let opcode = transform_opcode(PC as usize, &MEMORY);
        print!("{1:00$X}: ", 4, PC);
        match opcode[0] {
            0x0 => match opcode[1] {
                0x0 => match opcode[2] {
                    0xE => match opcode[3] {
                        0x0 => {
                            for row in 0..32{
                                for col in 0..64{
                                    SCREEN[row][col] = false;
                                }
                            }
                        },
                        0xE => {
                            PC = STACK[STACK.len()-1];
                            STACK.pop();
                        },
                        //_ => debug_print(&opcode, 1),
                        _ => print!("CALL {:x}{:x}{:x}", opcode[1], opcode[2], opcode[3]),
                    },
                    //_ => debug_print(&opcode, 2),
                    _ => print!("CALL {:x}{:x}{:x}", opcode[1], opcode[2], opcode[3]),
                },
                _ => print!("CALL {:x}{:x}{:x}", opcode[1], opcode[2], opcode[3]),
            },
            0x1 => {
                PC = (opcode[1] as u16) << 8;
                PC |= ((opcode[2] as u16) << 4);
                PC |= (opcode[3] as u16);
                PC = PC - 2; //Account for adding 2 to PC at end of loop
            },
                
            0x2 => {
                STACK.push(PC);
                PC = (opcode[1] as u16) << 8;
                PC |= ((opcode[2] as u16) << 4);
                PC |= (opcode[3] as u16);
                PC = PC - 2; //Account for adding 2 to PC at end of loop
            },
            0x3 => if REGS[opcode[1] as usize] == ((opcode[2] << 4) | opcode[3]){
                    PC = PC + 2;
                },
            0x4 => if REGS[opcode[1] as usize] != ((opcode[2] << 4) | opcode[3]){
                    PC = PC + 2;
                },
            0x5 => if REGS[opcode[1] as usize] != REGS[opcode[2] as usize]{
                    PC = PC + 2;
                },
            0x6 => {
                REGS[opcode[1] as usize] = ((opcode[2] << 4) | opcode[3]);
                },
            0x7 => {
                REGS[opcode[1] as usize] =
                REGS[opcode[1] as usize] + ((opcode[2] << 4) | opcode[3]);
                },
            0x8 => match opcode[3] {
                0x0 => REGS[opcode[1] as usize] = REGS[opcode[2] as usize],
                0x1 => REGS[opcode[1] as usize] |= REGS[opcode[2] as usize],
                0x2 => REGS[opcode[1] as usize] &= REGS[opcode[2] as usize],
                0x3 => REGS[opcode[1] as usize] ^= REGS[opcode[2] as usize],
                0x4 => REGS[opcode[1] as usize] = REGS[opcode[1] as usize] + REGS[opcode[2] as usize], //TODO carry
                0x5 => REGS[opcode[1] as usize] = REGS[opcode[1] as usize] - REGS[opcode[2] as usize], //TODO carry
                0x6 => {
                    REGS[0xF] = (REGS[opcode[1] as usize] & 1);
                    REGS[opcode[1] as usize] >>= 1;
                },
                0x7 => REGS[opcode[1] as usize] = REGS[opcode[2] as usize] - REGS[opcode[1] as usize], //TODO borrow bit
                0xe => {
                    REGS[0xF] = (REGS[opcode[1] as usize] & 1);
                    REGS[opcode[1] as usize] <<= 1;
                },
                _ => debug_print(&opcode, 3),
            },
            0x9 => if REGS[opcode[1] as usize] == REGS[opcode[2] as usize]{
                    PC = PC + 2;
                },
            0xA => {
                ADDR = (opcode[1] as u16) << 8;
                ADDR |= ((opcode[2] as u16) << 4);
                ADDR |= (opcode[3] as u16);
            },
            0xB => {
                PC = (opcode[1] as u16) << 8;
                PC |= ((opcode[2] as u16) << 4);
                PC |= (opcode[3] as u16);
                PC = PC + REGS[0] as u16;
                PC = PC - 2; //Account for adding 2 to PC at end of loop
            },
            0xC => REGS[opcode[1] as usize] = 
                rand::random::<u8>()&((opcode[1] << 4) | opcode[2]),
            0xD => {
                //println!("draw(V{:x},V{:x},{:x})", opcode[1], opcode[2], opcode[3]);
                let mut x = REGS[opcode[1] as usize] as usize;
                let mut y = REGS[opcode[2] as usize] as usize;
                let mut addr = ADDR as usize;
                for i in 0..opcode[3] as usize{
                    let mut row = MEMORY[addr];
                    for _ in 0..8{
                        if y >= SCREEN.len() || x >= SCREEN[0].len(){
                            println!("{0} {1}", x, y);
                            continue;
                        }
                        let prev = SCREEN[y][x];
                        SCREEN[y][x] = ((row & 1) != 0);
                        if SCREEN[y][x] != prev{
                            REGS[0xF] = 1;
                        }
                        row >>= 1;
                        x = x+1;
                    }
                    y = y+1;
                    x = x-8;
                    addr = addr+1;
                }
            },//TODO, thats a big boy, verify if it works
            0xE => match opcode[2]{
                0x9 => print!("IF key() == V{:x}", opcode[1]),
                0xA => print!("IF key() != V{:x}", opcode[1]),
                _ => debug_print(&opcode, 4),
            },
            0xF => match opcode[2]{
                0x0 => match opcode[3]{
                    0x7 => REGS[opcode[1] as usize] = DELAY_TIMER,
                    0xA => print!("V{:x} = get_key()", opcode[1]),
                    _ => debug_print(&opcode, 5),
                },
                0x1 => match opcode[3]{
                    0x5 => DELAY_TIMER = REGS[opcode[1] as usize],
                    0x8 => SOUND_TIMER = REGS[opcode[1] as usize],
                    0xE => ADDR = ADDR + REGS[opcode[1] as usize] as u16,
                    _ => debug_print(&opcode, 6),
                },
                0x2 => print!("I=spride_addr[V{:x}]", opcode[1]),
                0x3 => print!("set_BCD(V{:x})", opcode[1]),
                0x5 => print!("reg_dump(V{:x}, &I)", opcode[1]),
                0x6 => print!("reg_load(V{:x}, &I)", opcode[1]),
                _ => debug_print(&opcode, 7),
            },
            _ => debug_print(&opcode, 8),
        }

        let frametime = time::Duration::from_millis(16);
        thread::sleep(frametime);

        DELAY_TIMER = DELAY_TIMER+1;
        if DELAY_TIMER >= 60{
            draw(canvas, sdl_context, &SCREEN);
            //REFRESH
            DELAY_TIMER = 0;
        }
        PC = PC+2;


        println!("");
    }

    //Return ok so the compiler doesnt complain about the result
    Ok(())
}
