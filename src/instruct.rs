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
    pub SCREEN: [[bool; 64]; 32]
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
        }
    }
}

//OPCODE: 00E0
pub fn clear_screen(cpu: &mut CPU, opcode: &Vec<u8>){
    println!("CLEAR SCREEN");
    for row in 0..32{
        for col in 0..64{
            cpu.SCREEN[row][col] = false;
        }
    }
}
//OPCODE: 
pub fn ret(cpu: &mut CPU, opcode: &Vec<u8>){
    cpu.PC = cpu.STACK[cpu.STACK.len()-1];
    cpu.STACK.pop();
}
//OPCODE: 
pub fn goto(cpu: &mut CPU, opcode: &Vec<u8>){
    cpu.PC = (opcode[1] as u16) << 8;
    cpu.PC |= (opcode[2] as u16) << 4;
    cpu.PC |= opcode[3] as u16;
    cpu.PC = cpu.PC - 2; //Account for adding 2 to PC at end of loop
}
//OPCODE: 
pub fn func(cpu: &mut CPU, opcode: &Vec<u8>){
    cpu.STACK.push(cpu.PC);
    cpu.PC = (opcode[1] as u16) << 8;
    cpu.PC |= (opcode[2] as u16) << 4;
    cpu.PC |= opcode[3] as u16;
    cpu.PC = cpu.PC - 2; //Account for adding 2 to PC at end of loop
}
//OPCODE: 
pub fn if_eq_i(cpu: &mut CPU, opcode: &Vec<u8>){
    if cpu.REGS[opcode[1] as usize] == ((opcode[2] << 4) | opcode[3]){
        cpu.PC = cpu.PC + 2;
    }
}
//OPCODE: 
pub fn if_neq_i(cpu: &mut CPU, opcode: &Vec<u8>){
    if cpu.REGS[opcode[1] as usize] != ((opcode[2] << 4) | opcode[3]){
        cpu.PC = cpu.PC + 2;
    }
}
//OPCODE: 
pub fn if_neq(cpu: &mut CPU, opcode: &Vec<u8>){
    if cpu.REGS[opcode[1] as usize] != cpu.REGS[opcode[2] as usize]{
        cpu.PC = cpu.PC + 2;
    }
}
//OPCODE: 7XNN
pub fn assign_i(cpu: &mut CPU, opcode: &Vec<u8>){
    cpu.REGS[opcode[1] as usize] = (opcode[2] << 4) | opcode[3];
}
//OPCODE: 
pub fn plus_eq_i(cpu: &mut CPU, opcode: &Vec<u8>){
    cpu.REGS[opcode[1] as usize] =
        cpu.REGS[opcode[1] as usize] + (opcode[2] << 4) | opcode[3];
}
//OPCODE: 
pub fn assign(cpu: &mut CPU, opcode: &Vec<u8>){
    cpu.REGS[opcode[1] as usize] = cpu.REGS[opcode[2] as usize];
}
//OPCODE: 
pub fn bit_or(cpu: &mut CPU, opcode: &Vec<u8>){
    cpu.REGS[opcode[1] as usize] |= cpu.REGS[opcode[2] as usize];
}
//OPCODE: 
pub fn bit_and(cpu: &mut CPU, opcode: &Vec<u8>){
    cpu.REGS[opcode[1] as usize] &= cpu.REGS[opcode[2] as usize];
}
//OPCODE: 
pub fn bit_xor(cpu: &mut CPU, opcode: &Vec<u8>){
    cpu.REGS[opcode[1] as usize] ^= cpu.REGS[opcode[2] as usize];
}
//OPCODE: 
pub fn plus_eq(cpu: &mut CPU, opcode: &Vec<u8>){
    cpu.REGS[opcode[1] as usize] = cpu.REGS[opcode[1] as usize] + cpu.REGS[opcode[2] as usize]; //TODO carr
}
//OPCODE: 
pub fn minus_eq(cpu: &mut CPU, opcode: &Vec<u8>){
    cpu.REGS[opcode[1] as usize] = cpu.REGS[opcode[1] as usize] - cpu.REGS[opcode[2] as usize]; //TODO carr
}
//OPCODE: 
pub fn shift_r(cpu: &mut CPU, opcode: &Vec<u8>){
    cpu.REGS[0xF] = cpu.REGS[opcode[1] as usize] & 1;
    cpu.REGS[opcode[1] as usize] >>= 1;
}
//OPCODE: 
pub fn minus_eq_b(cpu: &mut CPU, opcode: &Vec<u8>){
    cpu.REGS[opcode[1] as usize] = cpu.REGS[opcode[2] as usize] - cpu.REGS[opcode[1] as usize]; //TODO borrow bit
}
//OPCODE: 
pub fn shift_l(cpu: &mut CPU, opcode: &Vec<u8>){
    cpu.REGS[0xF] = cpu.REGS[opcode[1] as usize] & 1;
    cpu.REGS[opcode[1] as usize] <<= 1;
}
//OPCODE: 
pub fn if_eq(cpu: &mut CPU, opcode: &Vec<u8>){
    if cpu.REGS[opcode[1] as usize] == cpu.REGS[opcode[2] as usize]{
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
//OPCODE: 
pub fn rand(cpu: &mut CPU, opcode: &Vec<u8>){
    cpu.REGS[opcode[1] as usize] = 
        rand::random::<u8>()&((opcode[1] << 4) | opcode[2]);
}
//OPCODE:
pub fn draw(cpu: &mut CPU, opcode: &Vec<u8>){
    //Docs and experimenting shows that first opcode is col num second is row num
    let mut coli = cpu.REGS[opcode[1] as usize] as usize;
    let mut rowi = cpu.REGS[opcode[2] as usize] as usize;
    let mut addr = cpu.ADDR as usize;
    let mut erased = false;
    println!("row: {:x} col: {:x}", rowi, coli);
    for _ in 0..opcode[3] as usize{
        let mut row = cpu.MEMORY[addr];
        for _ in 0..8{
            if rowi < 0 {
                rowi = 32 + rowi;
            }
            if coli < 0 {
                coli = 64 + coli;
            }
            let prev = cpu.SCREEN[rowi%32][coli%64];
            cpu.SCREEN[rowi%32][coli%64] ^= (row & 1) != 0;
            println!("SET row: {:x} col: {:x}", rowi%32, coli%64);
            if cpu.SCREEN[rowi%32][coli%64] != prev && prev == true{
                cpu.REGS[0xF] = 1;
                erased = true;
            }
            row >>= 1;
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
//OPCODE: 
pub fn if_key_eq(cpu: &mut CPU, opcode: &Vec<u8>){
    //print!("IF key() == V{:x}", opcode[1]);
}
//OPCODE: 
pub fn if_key_neq(cpu: &mut CPU, opcode: &Vec<u8>){
    //print!("IF key() != V{:x}", opcode[1]);
}
//OPCODE: 
pub fn get_delay(cpu: &mut CPU, opcode: &Vec<u8>){
    cpu.REGS[opcode[1] as usize] = cpu.DELAY_TIMER;
}
//OPCODE: 
pub fn get_key(cpu: &mut CPU, opcode: &Vec<u8>){
    //print!("V{:x} = get_key()", opcode[1]);
}
//OPCODE: 
pub fn delay_assign(cpu: &mut CPU, opcode: &Vec<u8>){
    cpu.DELAY_TIMER = cpu.REGS[opcode[1] as usize];
}
//OPCODE: 
pub fn sound_assign(cpu: &mut CPU, opcode: &Vec<u8>){
    cpu.SOUND_TIMER = cpu.REGS[opcode[1] as usize];
}
//OPCODE: 
pub fn i_plus_eq(cpu: &mut CPU, opcode: &Vec<u8>){
    cpu.ADDR = cpu.ADDR + cpu.REGS[opcode[1] as usize] as u16;
}
//OPCODE: 
pub fn i_sprite(cpu: &mut CPU, opcode: &Vec<u8>){
    //print!("I=spride_addr[V{:x}]", opcode[1]);
}
//OPCODE: 
pub fn set_bcd(cpu: &mut CPU, opcode: &Vec<u8>){
    //print!("set_BCD(V{:x})", opcode[1]);
}
//OPCODE: 
pub fn reg_dump(cpu: &mut CPU, opcode: &Vec<u8>){
    for i in 0..opcode[1] as usize{
        cpu.MEMORY[cpu.ADDR as usize +i] = cpu.REGS[i];
    }
}
//OPCODE: 
pub fn reg_load(cpu: &mut CPU, opcode: &Vec<u8>){
    for i in 0..opcode[1] as usize{
        cpu.REGS[i] = cpu.MEMORY[cpu.ADDR as usize +i];
    }
}
