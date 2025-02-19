use crate::decoder::decode_instruction;
use crate::opcode::OpCode;
use crate::renderer::{GRID_X_SIZE, GRID_Y_SIZE};
use log::warn;
use std::collections::HashSet;
use std::fs;
use std::ops::BitXor;
use std::time::Instant;

#[derive(Debug)]
pub struct Cpu {
    memory: [u16; 4096],
    pc: u16,     //program counter
    v: [u8; 16], // registers
    i: u16,      // index
    stack: Vec<u16>,

    pub screen: [[bool; GRID_X_SIZE]; GRID_Y_SIZE],
    pub should_render: bool,
    delay_timer: u8,
    sound_timer: u8,
    is_key_pressed: bool,
    key_pressed: Option<u16>,
    keys_pressed: HashSet<u8>,
    last_ticked_at: Instant,
}

pub fn new() -> Cpu {
    Cpu {
        memory: [0; 4096],
        pc: 0,
        v: [0; 16],
        i: 0,
        stack: Vec::new(),
        screen: [[false; GRID_X_SIZE]; GRID_Y_SIZE],
        should_render: false,
        delay_timer: 0,
        sound_timer: 0,
        is_key_pressed: false,
        key_pressed: None,
        last_ticked_at: Instant::now(),
        keys_pressed: HashSet::new(),
    }
}

impl Default for Cpu {
    fn default() -> Self {
        new()
    }
}

impl Cpu {
    pub fn load_rom(&mut self, path: &str) -> Result<(), &str> {
        let bytes = fs::read(path).map_err(|_| "can't read rom file")?;
        self.load_into_memory(bytes, 0x200);
        self.pc = 0x200;
        Ok(())
    }

    pub fn load_fonts(&mut self, path: &str) -> Result<(), &str> {
        let bytes = fs::read(path).map_err(|_| "can't read fonts file")?;
        self.load_into_memory(bytes, 0x50);
        Ok(())
    }

    fn load_into_memory(&mut self, bytes: Vec<u8>, offset: usize) {
        bytes.iter().enumerate().for_each(|(i, &x)| {
            self.memory[offset + i] = x.into();
        });
    }

    pub fn tick(&mut self) {
        let instruction = self.fetch_next_instruction();
        self.pc += 2;
        let op_code = decode_instruction(instruction);

        match op_code {
            OpCode::Jump(next_pc) => self.pc = next_pc,
            OpCode::JumpWithV0Offset(next_pc) => {
                self.pc = next_pc.saturating_add(self.v[0x0] as u16)
            }
            OpCode::RetFromSubroutine => {
                let return_address = self.stack.pop().unwrap();
                self.pc = return_address;
            }
            OpCode::CallSubroutine(next_pc) => {
                self.stack.push(self.pc);
                self.pc = next_pc;
            }
            OpCode::AddRegister { register, value } => self.add_to_register(register, value),
            OpCode::SetRegister { register, value } => self.set_to_register(register, value),
            OpCode::SetRegisterToRegisterValueUsingOR(x, y) => {
                self.v[x as usize] |= self.v[y as usize];
            }
            OpCode::SetRegisterToRegisterValueUsingAND(x, y) => {
                self.v[x as usize] &= self.v[y as usize];
            }
            OpCode::SetRegisterToRegisterValueUsingXOR(x, y) => {
                self.v[x as usize] ^= self.v[y as usize];
            }
            OpCode::SetRegisterWithRandom(x, nn) => {
                let random_number: u8 = rand::random();
                self.v[x] = random_number & nn;
            }
            OpCode::AddRegisterToRegister(x, y) => {
                let (new_register_value, is_overflow) =
                    self.v[x as usize].overflowing_add(self.v[y as usize]);
                self.v[x as usize] = new_register_value;
                self.v[0xF] = if is_overflow { 1 } else { 0 };
            }
            OpCode::SubRegisterToRegister(x, y) => {
                let (new_register_value, is_overflow) =
                    self.v[x as usize].overflowing_sub(self.v[y as usize]);
                self.v[x as usize] = new_register_value;
                self.v[0xF] = if is_overflow { 0 } else { 1 };
            }
            OpCode::ShiftRightRegisterFromRegister(x, _y) => {
                // Check the least significant bit
                let least_significant_bit = if self.v[x as usize] & 0x1 == 1 { 1 } else { 0 };
                self.v[x as usize] >>= 1;
                self.v[0xF] = least_significant_bit;
            }
            OpCode::ShiftLeftRegisterFromRegister(x, _y) => {
                // Check the least significant bit
                let least_significant_bit = if self.v[x as usize] & 0x80 == 128 {
                    1
                } else {
                    0
                };
                self.v[x as usize] <<= 1;
                self.v[0xF] = least_significant_bit;
            }
            OpCode::SubRegisterToRegisterReverse(x, y) => {
                let (new_register_value, is_overflow) =
                    self.v[y as usize].overflowing_sub(self.v[x as usize]);
                self.v[x as usize] = new_register_value;
                self.v[0xF] = if is_overflow { 0 } else { 1 };
            }
            OpCode::SetIndex(index) => self.set_index(index),
            OpCode::ClearScreen => self.clear_screen(),
            OpCode::Draw(vx, vy, nibble) => self.display(vx, vy, nibble),
            OpCode::SkipIfRegisterEquals(register, value) => {
                if self.v[register as usize] == value {
                    self.pc += 2;
                }
            }
            OpCode::SkipIfRegisterNotEquals(register, value) => {
                if self.v[register as usize] != value {
                    self.pc += 2;
                }
            }
            OpCode::SkipIfBothRegistersEqual(x, y) => {
                if self.v[x as usize] == self.v[y as usize] {
                    self.pc += 2;
                }
            }
            OpCode::SkipIfBothRegistersNotEqual(x, y) => {
                if self.v[x as usize] != self.v[y as usize] {
                    self.pc += 2;
                }
            }
            OpCode::SetRegisterToRegisterValue(x, y) => {
                self.v[x as usize] = self.v[y as usize];
            }
            OpCode::AddRegisterValueToIndex(x) => {
                self.i += self.v[x as usize] as u16;
            }
            OpCode::StoreBCDRepresentationOfRegister(x) => {
                let value = self.v[x as usize];
                let hundreds = value / 100;
                let tens = (value / 10) % 10;
                let ones = value % 10;
                self.memory[self.i as usize] = hundreds as u16;
                self.memory[(self.i + 1) as usize] = tens as u16;
                self.memory[(self.i + 2) as usize] = ones as u16;
            }
            OpCode::LoadFromRegistersToMemory(x) => {
                for i in 0..=x {
                    self.memory[(self.i + i as u16) as usize] = self.v[i as usize] as u16;
                }
            }
            OpCode::LoadFromMemoryToRegisters(x) => {
                for i in 0..=x {
                    self.v[i as usize] = self.memory[(self.i + i as u16) as usize] as u8;
                }
            }
            OpCode::SetRegisterFromDelayTimer(x) => {
                self.v[x as usize] = self.delay_timer;
            }
            OpCode::SetDelayTimerFromRegister(x) => {
                self.delay_timer = self.v[x as usize];
            }
            OpCode::SetSoundTimerFromRegister(x) => {
                self.sound_timer = self.v[x as usize];
            }
            OpCode::LoadSpriteRepresentationInMemory(x) => {
                self.i = self.v[x as usize] as u16 * 5;
            }
            OpCode::GetKey(x) => {
                if !self.keys_pressed.contains(&self.v[x]) {
                    self.pc -= 2;
                }
            }
            OpCode::SkipIfKey(x) => {
                if self.keys_pressed.contains(&self.v[x]) {
                    self.pc += 2;
                }
            }
            OpCode::SkipIfNotKey(x) => {
                if !self.keys_pressed.contains(&self.v[x]) {
                    self.pc += 2;
                }
            }
            _ => {
                warn!("Unknown opcode: {}", op_code);
            }
        }
        log::info!("pc={}|i={}|regs={:?}", self.pc, self.i, self.v);
    }

    pub fn tick_timers(&mut self) {
        let duration = self.last_ticked_at.elapsed();
        if duration.as_millis() > (1000 / 60) as u128 {
            self.sound_timer = self.sound_timer.saturating_sub(1);
            self.delay_timer = self.delay_timer.saturating_sub(1);
            self.last_ticked_at = Instant::now();
        }
    }

    fn set_index(&mut self, index: u16) {
        self.i = index
    }

    fn add_to_register(&mut self, register: usize, value: u8) {
        self.v[register] = self.v[register].wrapping_add(value);
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

    fn clear_screen(&mut self) {
        self.screen.iter_mut().for_each(|row| {
            row.iter_mut().for_each(|col| {
                *col = false;
            });
        });
        self.should_render = true;
    }

    fn display(&mut self, x: usize, y: usize, nibble: u8) {
        let mut vy = self.v[y] % 32;
        self.v[0xF] = 0;

        'lines: for sprite_row in 0..nibble {
            let mut vx = self.v[x] % 64;
            let sprite_index = (self.i + sprite_row as u16) as usize;
            let sprite_data = self.memory[sprite_index] as u8;

            'columns: for i in (0..8).rev() {
                // update the screen sprite
                let sprite_value_suggestion = (sprite_data >> i & 1) >= 1;
                let collision: bool =
                    self.draw_pixel(vx as usize, vy as usize, sprite_value_suggestion);
                if collision {
                    self.v[0xF] = 1;
                }
                vx += 1;
                if vx >= 64 {
                    break 'columns;
                }
            }
            vy += 1;
            if vy >= 32 {
                break 'lines;
            }
        }
        self.should_render = true;
    }

    fn draw_pixel(&mut self, x: usize, y: usize, value: bool) -> bool {
        let current_sprite_value = self.screen[y][x];
        let new_sprite_value = current_sprite_value.bitxor(value);

        self.screen[y][x] = new_sprite_value;

        if current_sprite_value && !new_sprite_value {
            return true;
        }
        false
    }

    pub fn set_key_pressed(&mut self, key: u16) {
        self.is_key_pressed = true;
        self.key_pressed = Some(key);
    }

    pub fn set_key_released(&mut self) {
        self.is_key_pressed = false;
    }

    pub fn set_keys_pressed(&mut self, keys: HashSet<u8>) {
        self.keys_pressed = keys;
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::new;
    use crate::decoder::decode_instruction;
    use crate::opcode::OpCode;

    #[test]
    fn can_load_rom_file() {
        let mut instance = new();
        let result = instance.load_rom("./roms/test.ch8");

        assert!(result.is_ok());
        assert_ne!(instance.memory[0x200], 0);
    }

    #[test]
    fn throws_when_path_is_invalid() {
        let mut instance = new();
        let result = instance.load_rom("./roms/not_existing_file.ch8");

        assert!(result.is_err());
    }

    #[test]
    fn can_log_the_instruction() {
        let mut instance = new();
        instance.load_rom("./roms/test.ch8").unwrap();

        let instruction = instance.fetch_next_instruction();
        let op_code = decode_instruction(instruction);
        assert_eq!(op_code, OpCode::Jump(520))
    }
}
