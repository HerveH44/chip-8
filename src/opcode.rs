use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum OpCode {
    ClearScreen,
    Jump(u16),
    Unknown,
    SetRegister { register: usize, value: u8 },
    AddRegister { register: usize, value: u8 },
    SetIndex(u16),
    Display(usize, usize, u8),
    SkipIfEqual(u8, u8),
    SkipIfNotEqual(u8, u8),
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
            OpCode::SkipIfEqual(x, nn) => { Display::fmt(&format!("SkipIfEqual(x={x}, nn={nn})"), f)}
            OpCode::SkipIfNotEqual(x, nn) => { Display::fmt(&format!("SkipIfNotEqual(x={x}, nn={nn})"), f)}
        }
    }
}
