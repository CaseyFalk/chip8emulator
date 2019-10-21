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
    cpu.cycle();
}

struct Cpu {
    opcode: u16,
    registers_v: [u8; 16],
    register_i: u16,
    sound_timer: u8,
    delay_timer: u8,
    stack: [u16; 16],
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
            stack: [0; 16],
            program_counter: 0x200,
            stack_pointer: 0,
            memory: [0; 4096]
        }
    }

    fn load_sprites(&mut self, sprites: [u8; 80]){
        for (index, &byte) in sprites.iter().enumerate() {
            self.memory[index] = byte;
        }
    }

    fn load_program(&mut self, program: Vec<u8>) {
        for (index, &byte) in program.iter().enumerate() {
            let i = index + 0x200;
            self.memory[i] = byte;
        }
    }

    fn cycle(&mut self) {
        self.opcode = (self.memory[self.program_counter as usize] as u16) << 8 | (self.memory[(self.program_counter + 1) as usize] as u16);

        let nnn = self.opcode & 0x0FFF;
        let n = (self.opcode & 0x000F) as u8;
        let x = ((self.opcode & 0x0F00) >> 8) as u8;
        let y = ((self.opcode & 0x00F0) >> 4) as u8;
        let kk = (self.opcode & 0x00FF) as u8;

        println!("opcode: {:x}, n: {:x}, nnn: {:x}, x: {:x}, y: {:x}", self.opcode, n, nnn, x, y);

        match (self.opcode & 0xF000) >> 12 {

            0x0 => match n {

                //00E0: CLS, clear the display
                //0x0 =>

                //00EE: RET, return from a subroutine
                0xE => {
                    self.stack_pointer -= 1;
                    self.program_counter = self.stack[self.stack_pointer as usize];
                    self.program_counter += 2;
                },

                //Invalid opcode
                _  => {
                    println!("Invalid opcode {}, aborting.", self.opcode);
                },

            },

            //1nnn: JP addr, jump to location nnn
            0x1 => self.program_counter = self.opcode & 0x0FFF,

            //2nnn: CALL addr, call subroutine at nnn
            0x2 => {
                self.stack[self.stack_pointer as usize] = self.program_counter;
                self.stack_pointer += 1;
                self.program_counter = nnn;
            }

            //3xkk: SE Vx, byte, skips the next instruction if Vx = kk
            0x3000 => {
                if self.registers_v[x as usize] == kk{
                    self.program_counter += 4;
                }
                else {
                    self.program_counter += 2;
                }
            }

            //4xkk: SNE Vx, byte, skips the next instruction if Vx != kk
            0x4000 => {
                if self.registers_v[x as usize] != kk{
                    self.program_counter += 4;
                }
                else {
                    self.program_counter += 2;
                }
            }

            //5xy0: SE Vx, Vy, skips the next instruction if Vx = Vy
            0x4000 => {
                if self.registers_v[x as usize] == self.registers_v[y as usize]{
                    self.program_counter += 4;
                }
                else {
                    self.program_counter += 2;
                }
            }

            //Invalid opcode
            _  => {
                println!("Invalid opcode {}, aborting.", self.opcode);
            },
        }
    }
}