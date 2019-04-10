//SDL2 imports
extern crate sdl2; 
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

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
       SCREEN: &[[bool; 64]; 32], KEYS: &mut [bool;16]){
    let mut event_pump = sdl_context.event_pump().unwrap();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit {..} |
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                std::process::exit(0);
            },
            Event::KeyDown { keycode: Some(key), .. } => {
                KEYS[(key as usize)%16] = true;
            },
            Event::KeyUp { keycode: Some(key), .. } => {
                KEYS[(key as usize)%16] = false;
            },
            _ => {}
        }
    }
    // The rest of the game loop goes here...
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    for row in 0..32{
        for col in 0..64{
            if SCREEN[row][col]{
                let r = sdl2::rect::Rect::new(10*col as i32, 10*row as i32,10,10);
                canvas.fill_rect(r);
            }
        }
    }

    canvas.present();
    //::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
}

