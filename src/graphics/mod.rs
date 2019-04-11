//SDL2 imports
extern crate sdl2; 
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub mod instruct;

pub fn init() -> (sdl2::render::WindowCanvas, sdl2::Sdl){
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

    return (canvas, sdl_context);
}

pub fn draw(canvas: &mut sdl2::render::WindowCanvas, sdl_context: &sdl2::Sdl,
       cpu: &mut instruct::CPU){
    let mut event_pump = sdl_context.event_pump().unwrap();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit {..} |
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                std::process::exit(0);
            },
            Event::KeyDown {keycode: Some(Keycode::Num1), ..} => cpu.KEYS[1] = true,
            Event::KeyDown {keycode:Some(Keycode::Num2),..}=> cpu.KEYS[2] = true,
            Event::KeyDown {keycode:Some(Keycode::Num3),..}=> cpu.KEYS[3] = true,
            Event::KeyDown {keycode:Some(Keycode::Num4),..}=> cpu.KEYS[12] = true,
            Event::KeyDown {keycode:Some(Keycode::Q),..}=> cpu.KEYS[4] = true,
            Event::KeyDown {keycode:Some(Keycode::W),..}=> cpu.KEYS[5] = true,
            Event::KeyDown {keycode:Some(Keycode::E),..}=> cpu.KEYS[6] = true,
            Event::KeyDown {keycode:Some(Keycode::R),..}=> cpu.KEYS[13] = true,
            Event::KeyDown {keycode:Some(Keycode::A),..}=> cpu.KEYS[7] = true,
            Event::KeyDown {keycode:Some(Keycode::S),..}=> cpu.KEYS[8] = true,
            Event::KeyDown {keycode:Some(Keycode::D),..}=> cpu.KEYS[9] = true,
            Event::KeyDown {keycode:Some(Keycode::F),..}=> cpu.KEYS[14] = true,
            Event::KeyDown {keycode:Some(Keycode::Z),..}=> cpu.KEYS[10] = true,
            Event::KeyDown {keycode:Some(Keycode::X),..}=> cpu.KEYS[0] = true,
            Event::KeyDown {keycode:Some(Keycode::C),..}=> cpu.KEYS[11] = true,
            Event::KeyDown {keycode:Some(Keycode::V),..}=> cpu.KEYS[15] = true,
            Event::KeyUp {keycode: Some(Keycode::Num1), ..} => cpu.KEYS[1] = false,
            Event::KeyUp {keycode:Some(Keycode::Num2),..}=> cpu.KEYS[2] = false,
            Event::KeyUp {keycode:Some(Keycode::Num3),..}=> cpu.KEYS[3] = false,
            Event::KeyUp {keycode:Some(Keycode::Num4),..}=> cpu.KEYS[12] = false,
            Event::KeyUp {keycode:Some(Keycode::Q),..}=> cpu.KEYS[4] = false,
            Event::KeyUp {keycode:Some(Keycode::W),..}=> cpu.KEYS[5] = false,
            Event::KeyUp {keycode:Some(Keycode::E),..}=> cpu.KEYS[6] = false,
            Event::KeyUp {keycode:Some(Keycode::R),..}=> cpu.KEYS[13] = false,
            Event::KeyUp {keycode:Some(Keycode::A),..}=> cpu.KEYS[7] = false,
            Event::KeyUp {keycode:Some(Keycode::S),..}=> cpu.KEYS[8] = false,
            Event::KeyUp {keycode:Some(Keycode::D),..}=> cpu.KEYS[9] = false,
            Event::KeyUp {keycode:Some(Keycode::F),..}=> cpu.KEYS[14] = false,
            Event::KeyUp {keycode:Some(Keycode::Z),..}=> cpu.KEYS[10] = false,
            Event::KeyUp {keycode:Some(Keycode::X),..}=> cpu.KEYS[0] = false,
            Event::KeyUp {keycode:Some(Keycode::C),..}=> cpu.KEYS[11] = false,
            Event::KeyUp {keycode:Some(Keycode::V),..}=> cpu.KEYS[15] = false,
            _ => {}
        }
    }
    // The rest of the game loop goes here...
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    for row in 0..32{
        for col in 0..64{
            if cpu.SCREEN[row][col]{
                let r = sdl2::rect::Rect::new(10*col as i32, 10*row as i32,10,10);
                canvas.fill_rect(r);
            }
        }
    }

    canvas.present();
    //::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
}

