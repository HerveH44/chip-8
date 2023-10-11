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
    SkipIfRegisterEquals(u8, u8),
    SkipIfRegisterNotEquals(u8, u8),
    SkipIfBothRegistersEqual(u8, u8),
    SetRegisterToRegisterValue(u8, u8),
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
            OpCode::SkipIfRegisterEquals(x, nn) => { Display::fmt(&format!("SkipIfEqual(x={x}, nn={nn})"), f)}
            OpCode::SkipIfRegisterNotEquals(x, nn) => { Display::fmt(&format!("SkipIfNotEqual(x={x}, nn={nn})"), f)}
            OpCode::SkipIfBothRegistersEqual(x, y) => { Display::fmt(&format!("SkipIfBothRegistersEquals(x={x}, y={y})"), f)}
            OpCode::SetRegisterToRegisterValue(x, y) => { Display::fmt(&format!("SetRegisterToRegisterValue(x={x}, y={y})"), f)}
        }
    }
}
