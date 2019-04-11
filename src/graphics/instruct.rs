use sdl2::event::Event;
use sdl2::keyboard::Keycode;

#[allow(non_snake_case)]
pub struct CPU {
    pub PROGRAM: Vec<Vec<u8>>,
    pub PC: u16,
    pub SP: u16,
    pub REGS: [u8; 16],
    pub ADDR: u16,
    pub STACK: Vec<u16>,
    pub DELAY_TIMER: u8,
    pub SOUND_TIMER: u8,
    pub MEMORY: [u8; 4096],
    pub SCREEN: [[bool; 64]; 32],
    pub KEYS: [bool; 16],
}
impl CPU {
    pub fn new() -> CPU {
        return CPU{
            PROGRAM: Vec::new(),
            PC: 0x200,
            SP: 0xEA0,
            REGS: [0;16],
            ADDR: 0,
            STACK: Vec::new(),
            DELAY_TIMER: 0,
            SOUND_TIMER: 0,
            MEMORY: [0; 4096],
            SCREEN: [[false; 64]; 32],
            KEYS: [false; 16],
        }
    }
    
    pub fn init(&mut self){
        //0 Character
        self.MEMORY[0]=0xF0;self.MEMORY[1]=0x90;self.MEMORY[2]=0x90;
        self.MEMORY[3]=0x90;self.MEMORY[4]=0xF0;
        //1 Character
        self.MEMORY[5]=0x20;self.MEMORY[6]=0x60;self.MEMORY[7]=0x20;
        self.MEMORY[8]=0x20;self.MEMORY[9]=0x70;
        //2 Character
        self.MEMORY[10]=0xF0;self.MEMORY[11]=0x10;self.MEMORY[12]=0xF0;
        self.MEMORY[13]=0x80;self.MEMORY[14]=0xF0;
        //3 Character
        self.MEMORY[15]=0xF0;self.MEMORY[16]=0x10;self.MEMORY[17]=0xF0;
        self.MEMORY[18]=0x10;self.MEMORY[19]=0xF0;
        //4 Character
        self.MEMORY[20]=0x90;self.MEMORY[21]=0x90;self.MEMORY[22]=0xF0;
        self.MEMORY[23]=0x10;self.MEMORY[24]=0x10;
        //5 Character
        self.MEMORY[25]=0xF0;self.MEMORY[26]=0x80;self.MEMORY[27]=0xF0;
        self.MEMORY[28]=0x10;self.MEMORY[29]=0xF0;
        //6 Character
        self.MEMORY[30]=0xF0;self.MEMORY[31]=0x80;self.MEMORY[32]=0xF0;
        self.MEMORY[33]=0x90;self.MEMORY[34]=0xF0;
        //7 Character
        self.MEMORY[35]=0xF0;self.MEMORY[36]=0x10;self.MEMORY[37]=0x20;
        self.MEMORY[38]=0x40;self.MEMORY[39]=0x40;
        //8 Character
        self.MEMORY[40]=0xF0;self.MEMORY[41]=0x90;self.MEMORY[42]=0xF0;
        self.MEMORY[43]=0x90;self.MEMORY[44]=0xF0;
        //9 Character
        self.MEMORY[45]=0xF0;self.MEMORY[46]=0x90;self.MEMORY[47]=0xF0;
        self.MEMORY[48]=0x10;self.MEMORY[49]=0xF0;
        //A Character
        self.MEMORY[50]=0xF0;self.MEMORY[51]=0x90;self.MEMORY[52]=0xF0;
        self.MEMORY[53]=0x90;self.MEMORY[54]=0x90;
        //B Character
        self.MEMORY[55]=0xE0;self.MEMORY[56]=0x90;self.MEMORY[57]=0xE0;
        self.MEMORY[58]=0x90;self.MEMORY[59]=0xE0;
        //C Character
        self.MEMORY[60]=0xF0;self.MEMORY[61]=0x80;self.MEMORY[62]=0x80;
        self.MEMORY[63]=0x80;self.MEMORY[64]=0xF0;
        //D Character
        self.MEMORY[65]=0xE0;self.MEMORY[66]=0x90;self.MEMORY[67]=0x90;
        self.MEMORY[68]=0x90;self.MEMORY[69]=0xE0;
        //E Character
        self.MEMORY[70]=0xF0;self.MEMORY[71]=0x80;self.MEMORY[72]=0xF0;
        self.MEMORY[73]=0x80;self.MEMORY[74]=0xF0;
        //F Character
        self.MEMORY[75]=0xF0;self.MEMORY[76]=0x80;self.MEMORY[77]=0xF0;
        self.MEMORY[78]=0x80;self.MEMORY[79]=0x80;
    }
}

//OPCODE: 00E0
pub fn clear_screen(cpu: &mut CPU, opcode: &Vec<u8>){
    for row in 0..32{
        for col in 0..64{
            cpu.SCREEN[row][col] = false;
        }
    }
}
//OPCODE: 00EE
pub fn ret(cpu: &mut CPU, opcode: &Vec<u8>){
    cpu.PC = cpu.STACK[cpu.STACK.len()-1];
    cpu.STACK.pop();
}
//OPCODE: 1NNN
pub fn goto(cpu: &mut CPU, opcode: &Vec<u8>){
    cpu.PC = (opcode[1] as u16) << 8;
    cpu.PC |= (opcode[2] as u16) << 4;
    cpu.PC |= opcode[3] as u16;
    cpu.PC = cpu.PC - 2; //Account for adding 2 to PC at end of loop
}
//OPCODE: 2NNN
pub fn func(cpu: &mut CPU, opcode: &Vec<u8>){
    cpu.STACK.push(cpu.PC);
    cpu.PC = (opcode[1] as u16) << 8;
    cpu.PC |= (opcode[2] as u16) << 4;
    cpu.PC |= opcode[3] as u16;
    cpu.PC = cpu.PC - 2; //Account for adding 2 to PC at end of loop
}
//OPCODE: 3XNN
//NOTE:
pub fn skip_eq_i(cpu: &mut CPU, opcode: &Vec<u8>){
    if cpu.REGS[opcode[1] as usize] == ((opcode[2] << 4) | opcode[3]){
        cpu.PC = cpu.PC + 2;
    }
}
//OPCODE: 4XNN
pub fn skip_neq_i(cpu: &mut CPU, opcode: &Vec<u8>){
    if cpu.REGS[opcode[1] as usize] != ((opcode[2] << 4) | opcode[3]){
        cpu.PC = cpu.PC + 2;
    }
}
//OPCODE: 5XY0
pub fn skip_eq(cpu: &mut CPU, opcode: &Vec<u8>){
    if cpu.REGS[opcode[1] as usize] == cpu.REGS[opcode[2] as usize]{
        cpu.PC = cpu.PC + 2;
    }
}
//OPCODE: 6XNN
pub fn assign_i(cpu: &mut CPU, opcode: &Vec<u8>){
    cpu.REGS[opcode[1] as usize] = ((opcode[2] << 4) | opcode[3]);
}
//OPCODE: 7XNN
pub fn plus_eq_i(cpu: &mut CPU, opcode: &Vec<u8>){
    cpu.REGS[opcode[1] as usize] =
        cpu.REGS[opcode[1] as usize] + ((opcode[2] << 4) | opcode[3]);
}
//OPCODE: 8XY0
pub fn assign(cpu: &mut CPU, opcode: &Vec<u8>){
    cpu.REGS[opcode[1] as usize] = cpu.REGS[opcode[2] as usize];
}
//OPCODE: 8XY1
pub fn bit_or(cpu: &mut CPU, opcode: &Vec<u8>){
    cpu.REGS[opcode[1] as usize] |= cpu.REGS[opcode[2] as usize];
}
//OPCODE: 8XY2
pub fn bit_and(cpu: &mut CPU, opcode: &Vec<u8>){
    cpu.REGS[opcode[1] as usize] &= cpu.REGS[opcode[2] as usize];
}
//OPCODE: 8XY3
pub fn bit_xor(cpu: &mut CPU, opcode: &Vec<u8>){
    cpu.REGS[opcode[1] as usize] ^= cpu.REGS[opcode[2] as usize];
}
//OPCODE: 8XY4
pub fn plus_eq(cpu: &mut CPU, opcode: &Vec<u8>){
    let res: u16 = cpu.REGS[opcode[1] as usize] as u16 + cpu.REGS[opcode[2] as usize] as u16;
    cpu.REGS[0xF] = if res > 255 {1} else {0};
    cpu.REGS[opcode[1] as usize] = (res & 0x00FF) as u8;
}
//OPCODE: 8XY5
pub fn minus_eq(cpu: &mut CPU, opcode: &Vec<u8>){
    cpu.REGS[0xF] = if cpu.REGS[opcode[1] as usize] > cpu.REGS[opcode[2] as usize] {1} else {0};
    cpu.REGS[opcode[1] as usize] = cpu.REGS[opcode[1] as usize] - cpu.REGS[opcode[2] as usize];
}
//OPCODE: 8XY6
pub fn shift_r(cpu: &mut CPU, opcode: &Vec<u8>){
    cpu.REGS[0xF] = cpu.REGS[opcode[1] as usize] & 1;
    cpu.REGS[opcode[1] as usize] >>= 1;
}
//OPCODE: 8XY7
pub fn minus_eq_b(cpu: &mut CPU, opcode: &Vec<u8>){
    cpu.REGS[0xF] = if cpu.REGS[opcode[2] as usize] > cpu.REGS[opcode[1] as usize] {1} else {0};
    cpu.REGS[opcode[1] as usize] = cpu.REGS[opcode[2] as usize] - cpu.REGS[opcode[1] as usize];
}
//OPCODE: 8XYE
pub fn shift_l(cpu: &mut CPU, opcode: &Vec<u8>){
    cpu.REGS[0xF] = cpu.REGS[opcode[1] as usize] & (1 >> 7);
    cpu.REGS[opcode[1] as usize] <<= 1;
}
//OPCODE: 9XY0
pub fn skip_neq(cpu: &mut CPU, opcode: &Vec<u8>){
    if cpu.REGS[opcode[1] as usize] != cpu.REGS[opcode[2] as usize]{
        cpu.PC = cpu.PC + 2;
    }
}
//OPCODE: 
pub fn assign_addr(cpu: &mut CPU, opcode: &Vec<u8>){
    cpu.ADDR = (opcode[1] as u16) << 8;
    cpu.ADDR |= (opcode[2] as u16) << 4;
    cpu.ADDR |= opcode[3] as u16;
}
//OPCODE: BNNN
pub fn jump_plus(cpu: &mut CPU, opcode: &Vec<u8>){
    cpu.PC = (opcode[1] as u16) << 8;
    cpu.PC |= (opcode[2] as u16) << 4;
    cpu.PC |= opcode[3] as u16;
    cpu.PC = cpu.PC + cpu.REGS[0] as u16;
    cpu.PC = cpu.PC - 2; //Account for adding 2 to PC at end of loop
}
//OPCODE: CXNN
pub fn rand(cpu: &mut CPU, opcode: &Vec<u8>){
    cpu.REGS[opcode[1] as usize] = 
        rand::random::<u8>()&((opcode[2] << 4) | opcode[3]);
    //println!("RAND: {}", cpu.REGS[opcode[1] as usize]);
}
//OPCODE:
pub fn draw(cpu: &mut CPU, opcode: &Vec<u8>){
    //Docs and experimenting shows that first opcode is col num second is row num
    let mut coli = cpu.REGS[opcode[1] as usize] as usize;
    let mut rowi = cpu.REGS[opcode[2] as usize] as usize;
    let mut addr = cpu.ADDR as usize;
    let mut erased = false;
    //println!("row: {:x} col: {:x}", rowi, coli);
    for _ in 0..opcode[3] as usize{
        let mut row = cpu.MEMORY[addr];
        for _ in 0..8{
            let prev = cpu.SCREEN[rowi%32][coli%64];
            cpu.SCREEN[rowi%32][coli%64] ^= (row & (1 << 7)) != 0;
            //println!("SET row: {:x} col: {:x}", rowi%32, coli%64);
            if cpu.SCREEN[rowi%32][coli%64] != prev && prev == true{
                cpu.REGS[0xF] = 1;
                erased = true;
            }
            row <<= 1;
            coli = coli+1;
        }
        rowi = rowi+1;
        coli = coli-8;
        addr = addr+1;
    }
    if erased == false{
        cpu.REGS[0xF] = 0;
    }
}
//OPCODE: EX9E
pub fn skip_key_eq(cpu: &mut CPU, opcode: &Vec<u8>){
    if cpu.KEYS[cpu.REGS[(opcode[1]) as usize] as usize] == true {
        cpu.PC = cpu.PC + 2;
    }
}
//OPCODE: EX9E
pub fn skip_key_neq(cpu: &mut CPU, opcode: &Vec<u8>){
    if cpu.KEYS[cpu.REGS[(opcode[1]) as usize] as usize] == false {
        cpu.PC = cpu.PC + 2;
    }
}
//OPCODE: FX07
pub fn get_delay(cpu: &mut CPU, opcode: &Vec<u8>){
    cpu.REGS[opcode[1] as usize] = cpu.DELAY_TIMER;
}
//OPCODE: FX0A
//NOTE: likely needs fixing
pub fn get_key(cpu: &mut CPU, opcode: &Vec<u8>, sdl_context: &sdl2::Sdl){
    let mut event_pump = sdl_context.event_pump().unwrap();
    //println!("TESTTTTTTTT");
    for event in event_pump.poll_iter() {
        let mut keypress = true;
        match event {
            Event::KeyDown {keycode:Some(Num1),..}=> cpu.REGS[opcode[1] as usize] = 0,
            Event::KeyDown {keycode:Some(Num2),..}=> cpu.REGS[opcode[1] as usize] = 1,
            Event::KeyDown {keycode:Some(Num3),..}=> cpu.REGS[opcode[1] as usize] = 2,
            Event::KeyDown {keycode:Some(Num4),..}=> cpu.REGS[opcode[1] as usize] = 3,
            Event::KeyDown {keycode:Some(Q),..}=> cpu.REGS[opcode[1] as usize] = 4,
            Event::KeyDown {keycode:Some(W),..}=> cpu.REGS[opcode[1] as usize] = 5,
            Event::KeyDown {keycode:Some(E),..}=> cpu.REGS[opcode[1] as usize] = 6,
            Event::KeyDown {keycode:Some(R),..}=> cpu.REGS[opcode[1] as usize] = 7,
            Event::KeyDown {keycode:Some(A),..}=> cpu.REGS[opcode[1] as usize] = 8,
            Event::KeyDown {keycode:Some(S),..}=> cpu.REGS[opcode[1] as usize] = 9,
            Event::KeyDown {keycode:Some(D),..}=> cpu.REGS[opcode[1] as usize] = 10,
            Event::KeyDown {keycode:Some(F),..}=> cpu.REGS[opcode[1] as usize] = 11,
            Event::KeyDown {keycode:Some(Z),..}=> cpu.REGS[opcode[1] as usize] = 12,
            Event::KeyDown {keycode:Some(X),..}=> cpu.REGS[opcode[1] as usize] = 13,
            Event::KeyDown {keycode:Some(C),..}=> cpu.REGS[opcode[1] as usize] = 14,
            Event::KeyDown {keycode:Some(V),..}=> cpu.REGS[opcode[1] as usize] = 15,
            _ => {keypress = false; break;},
        }
        if keypress == true {
            break;
        }
    }
    //print!("V{:x} = get_key()", opcode[1]);
}
//OPCODE: FX15
pub fn delay_assign(cpu: &mut CPU, opcode: &Vec<u8>){
    cpu.DELAY_TIMER = cpu.REGS[opcode[1] as usize];
}
//OPCODE: FX18
pub fn sound_assign(cpu: &mut CPU, opcode: &Vec<u8>){
    cpu.SOUND_TIMER = cpu.REGS[opcode[1] as usize];
}
//OPCODE: FX1E
pub fn i_plus_eq(cpu: &mut CPU, opcode: &Vec<u8>){
    cpu.ADDR = cpu.ADDR + cpu.REGS[opcode[1] as usize] as u16;
}
//OPCODE: FX29
pub fn i_sprite(cpu: &mut CPU, opcode: &Vec<u8>){
    match cpu.REGS[opcode[1] as usize]{
        0x0 => cpu.ADDR = 0,
        0x1 => cpu.ADDR = 5,
        0x2 => cpu.ADDR = 10,
        0x3 => cpu.ADDR = 15,
        0x4 => cpu.ADDR = 20,
        0x5 => cpu.ADDR = 25,
        0x6 => cpu.ADDR = 30,
        0x7 => cpu.ADDR = 35,
        0x8 => cpu.ADDR = 40,
        0x9 => cpu.ADDR = 45,
        0xA => cpu.ADDR = 50,
        0xB => cpu.ADDR = 55,
        0xC => cpu.ADDR = 60,
        0xD => cpu.ADDR = 65,
        0xE => cpu.ADDR = 70,
        0xF => cpu.ADDR = 75,
        _ => println!("Bad sprite"),
    }
}
//OPCODE: FX33
pub fn set_bcd(cpu: &mut CPU, opcode: &Vec<u8>){
    let val = cpu.REGS[opcode[1] as usize];
    cpu.MEMORY[cpu.ADDR as usize] = (val-val%100)/100;
    cpu.MEMORY[cpu.ADDR as usize+1] = val/10%10;
    cpu.MEMORY[cpu.ADDR as usize+2] = val%10;
}
//OPCODE: FX55
pub fn reg_dump(cpu: &mut CPU, opcode: &Vec<u8>){
    for i in 0..1+opcode[1] as usize{
        cpu.MEMORY[cpu.ADDR as usize +i] = cpu.REGS[i];
    }
}
//OPCODE: FX65
pub fn reg_load(cpu: &mut CPU, opcode: &Vec<u8>){
    for i in 0..1+opcode[1] as usize{
        cpu.REGS[i] = cpu.MEMORY[cpu.ADDR as usize +i];
    }
}
