extern crate sdl2;

fn main() {

    let sprites: [u8; 80] = [
        0xF0, 0x90, 0x90, 0x90, 0xF0,
        0x20, 0x60, 0x20, 0x20, 0x70,
        0xF0, 0x10, 0xF0, 0x80, 0xF0,
        0xF0, 0x10, 0xF0, 0x10, 0xF0,
        0x90, 0x90, 0xF0, 0x10, 0x10,
        0xF0, 0x80, 0xF0, 0x10, 0xF0,
        0xF0, 0x80, 0xF0, 0x90, 0xF0,
        0xF0, 0x10, 0x20, 0x40, 0x40,
        0xF0, 0x90, 0xF0, 0x90, 0xF0,
        0xF0, 0x90, 0xF0, 0x10, 0xF0,
        0xF0, 0x90, 0xF0, 0x90, 0x90,
        0xE0, 0x90, 0xE0, 0x90, 0xE0,
        0xF0, 0x80, 0x80, 0x80, 0xF0,
        0xE0, 0x90, 0x90, 0x90, 0xE0,
        0xF0, 0x80, 0xF0, 0x80, 0xF0,
        0xF0, 0x80, 0xF0, 0x80, 0x80,
    ];

    let cpu = &mut Cpu::new();
    cpu.load_sprites(sprites);
    cpu.load_program(vec![0x13, 0xC5]);
}

struct Cpu {
    opcode: u16,
    registers_v: [u8; 16],
    register_i: u16,
    sound_timer: u8,
    delay_timer: u8,
    program_counter: u16,
    stack_pointer: u8,
    memory: [u8; 4096]
}

impl Cpu {
    fn new() -> Cpu {
        Cpu {
            opcode: 0,
            registers_v: [0; 16],
            register_i: 0x200,
            sound_timer: 0,
            delay_timer: 0,
            program_counter: 0x200,
            stack_pointer: 0,
            memory: [0; 4096]
        }
    }

    fn load_sprites(&mut self, mut sprites: [u8; 80]){
        for (index, &byte) in sprites.iter().enumerate() {
            println!("pre: {}", self.memory[index]);
            self.memory[index] = byte;
            println!("post: {}", self.memory[index]);
        }
    }
    
    fn load_program(&mut self, mut program: Vec<u8>) {
        for (index, &byte) in program.iter().enumerate() {
            let i = index + 0x200;
            println!("{}", i);
            self.memory[i] = byte;
        }

        println!("{}", self.memory[0]);
    }
}