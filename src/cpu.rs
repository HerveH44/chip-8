use std::fmt::{Debug, Display, Formatter};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::time::SystemTime;
use crate::screen::Screen;

pub struct Cpu<'a> {
    memory: [u16; 4096],
    pc: u16, //program counter
    v: [u8; 16], // registers
    i: u16, // index
    screen: &'a mut Screen,
    should_render: bool,
    pub log_file: File,
}

pub fn new(screen: &mut Screen) -> Cpu {
    Cpu {
        memory: [0; 4096],
        pc: 0,
        v: [0; 16],
        i: 0,
        screen,
        should_render: false,
        log_file: File::create(format!("log_instruction-{}.txt", SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs()))
            .unwrap()
    }
}

impl<'a> Cpu<'a> {

    pub fn load_rom(&mut self, path: &str) -> Result<(), &str> {
        let bytes = fs::read(path).map_err(|_|"can't read rom file")?;
        self.load_into_memory(bytes, 0x200);
        self.pc = 0x200;
        Ok(())
    }

    pub fn load_fonts(&mut self, path: &str) -> Result<(), &str> {
        let bytes = fs::read(path).map_err(|_|"can't read fonts file")?;
       self.load_into_memory(bytes, 0x50);
        Ok(())
    }

    fn load_into_memory(&mut self, bytes: Vec<u8>, offset: usize) {
        bytes.iter().enumerate().for_each(| (i, &x )  | {
            self.memory[offset + i] = x.into();
        });
    }

    pub fn tick(&mut self) {
        let instruction = self.fetch_next_instruction();
        self.pc+=2;
        let op_code = self.decode_instruction(instruction);

        match op_code {
            OpCode::Jump(next_pc) => self.pc = next_pc,
            OpCode::AddRegister {register, value} => self.add_to_register(register, value),
            OpCode::SetRegister {register, value} => self.set_to_register(register, value),
            OpCode::SetIndex(index) => self.set_index(index),
            OpCode::ClearScreen => self.clear_screen(),
            OpCode::Display(vx, vy, nibble) => self.display(vx, vy, nibble),
            _ => ()
        }

        if self.should_render {
            self.screen.render();
            self.should_render = false;
        }
    }

    fn set_index(&mut self, index: u16) {
        self.i = index
    }

    fn add_to_register(&mut self, register: usize, value: u8) {
        let (new_register_value, is_overflow)= self.v[register].overflowing_add(value);
        self.v[0xF] = if is_overflow { 1 } else { 0 };
        self.v[register] = new_register_value;
    }

    fn set_to_register(&mut self, register: usize, value: u8) {
        self.v[register] = value
    }


    fn fetch_next_instruction(&self) -> u16 {
        let first_part = self.memory[self.pc as usize];
        let second_part = self.memory[(self.pc + 1) as usize];

        // combine first and second part as u16
        (first_part << 8) | second_part
    }

    fn decode_instruction(&mut self, instruction: u16) -> OpCode {
        let kind = (instruction & 0xF000) >> 12;
        let x = ((instruction & 0x0F00) >> 8) as usize;
        let y = ((instruction & 0x00F0) >> 4) as usize;
        let n = (instruction & 0x000F) as u8;
        let nn = (instruction & 0x00FF) as u8;
        let nnn = instruction & 0x0FFF;


        let opcode = match (kind, x, y, n) {
            (0x0, 0x0, 0xE, 0x0) => OpCode::ClearScreen,
            (0x1, _, _, _) => OpCode::Jump(nnn),
            (0x6, _, _, _) => OpCode::SetRegister {register: x, value: nn},
            (0x7, _, _, _) => OpCode::AddRegister {register: x, value: nn},
            (0xA, _, _, _) => OpCode::SetIndex(nnn),
            (0xD, _, _, _) => OpCode::Display(x, y, n),
            _ => OpCode::Unknown
        };

        self.log_file.write(format!("instruction={instruction}|kind={kind}|x={x}|y={y}|n={n}|nn={nn}|nnn={nnn}|opcode={opcode}\n").as_bytes()).unwrap();
        return opcode
    }

    fn clear_screen(&mut self) {
        self.screen.clear();
    }

    fn display(&mut self, x: usize, y: usize, nibble: u8) {
        let mut vy = self.v[y] % 32;
        self.v[0xF] = 0;

        'lines: for sprite_row in 0 .. nibble {
            let mut vx = self.v[x] % 64;
            let sprite_index = (self.i + sprite_row as u16) as usize;
            let sprite_data = self.memory[sprite_index] as u8;

            'columns: for i in (0..8).rev() {
                // update the screen sprite
                let sprite_value_suggestion = if (sprite_data >> i & 1) >= 1 { true } else { false };
                let collision: bool = self.screen.draw_pixel(vx as usize, vy as usize, sprite_value_suggestion);
                if collision {
                    self.v[0xF] = 1;
                }
                vx+=1;
                if vx >= 64 { break 'columns }
            }
            vy+=1;
            if vy >= 32 { break 'lines }
        }
        self.should_render = true;
    }
}


#[derive(Copy, Clone, Debug, PartialEq)]
enum OpCode {
    ClearScreen,
    Jump(u16),
    Unknown,
    SetRegister { register: usize, value: u8 },
    AddRegister { register: usize, value: u8 },
    SetIndex(u16),
    Display(usize, usize, u8),
}

impl Display for OpCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OpCode::ClearScreen => { Display::fmt("ClearScreen", f) }
            OpCode::Jump(jump) => { Display::fmt(&format!("Jump({jump})"), f)}
            OpCode::Unknown => { Display::fmt("Unknown", f)}
            OpCode::SetRegister { register, value } => { Display::fmt(&format!("SetRegister(register={register}, value={value})"), f)}
            OpCode::AddRegister { register, value } => { Display::fmt(&format!("AddRegister(register={register}, value={value})"), f)}
            OpCode::SetIndex(index) => { Display::fmt(&format!("SetIndex({index})", ), f)}
            OpCode::Display(x, y, n) => { Display::fmt(&format!("Display(x={x}, y={y}, n={n})"), f) }
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::cpu::{new, OpCode};
    use crate::screen;

    fn get_screen() -> screen::Screen {
        let sdl_context = sdl2::init().unwrap();
        screen::new(&sdl_context)
    }

    #[test]
    fn can_load_rom_file() {
        let mut my_screen = get_screen();
        let mut instance = new(&mut my_screen);
        let result = instance.load_rom("./roms/test.ch8");

        assert!(result.is_ok());
        assert_ne!(instance.memory[0x200], 0);
    }

    #[test]
    fn throws_when_path_is_invalid() {
        let mut my_screen = get_screen();
        let mut instance = new(&mut my_screen);
        let result = instance.load_rom("./roms/not_existing_file.ch8");

        assert!(result.is_err());
    }

    #[test]
    fn can_log_the_instruction() {
        let mut my_screen = get_screen();
        let mut instance = new(&mut my_screen);
        instance.load_rom("./roms/test.ch8").unwrap();

        let instruction = instance.fetch_next_instruction();
        let op_code = instance.decode_instruction(instruction);
        assert_eq!(op_code, OpCode::Jump(520))
    }

}
