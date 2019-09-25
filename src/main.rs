extern crate sdl2;

fn main() {
    let cpu = &mut Cpu::new();
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

    fn load_program(&mut self, mut program: Vec<u8>) {
        let mut data = vec![0; 0x200];
        data.append(&mut program);

        for (index, &byte) in data.iter().enumerate() {
            self.memory[index] = byte;
        }
    }
}