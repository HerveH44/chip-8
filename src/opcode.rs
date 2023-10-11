use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum OpCode {
    ClearScreen, // CLS
    RetFromSubroutine, // RET
    Jump(u16), // JP addr
    CallSubroutine(u16), // CALL addr
    SetRegister { register: usize, value: u8 }, // LD Vx, byte
    AddRegister { register: usize, value: u8 }, // ADD Vx, byte
    SetIndex(u16), // LD I, addr
    Display(usize, usize, u8), // DRW Vx, Vy, nibble
    SkipIfRegisterEquals(u8, u8), // SE Vx, byte
    SkipIfRegisterNotEquals(u8, u8), // SNE Vx, byte
    SkipIfBothRegistersEqual(u8, u8), // SE Vx, Vy
    SetRegisterToRegisterValue(u8, u8), // LD Vx, Vy
    SetRegisterToRegisterValueUsingOR(u8, u8), // OR Vx, Vy
    Unknown,
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
            OpCode::CallSubroutine(nnn) => { Display::fmt(&format!("CallSubroutine(nnn={nnn})"), f)}
            OpCode::RetFromSubroutine => { Display::fmt(&format!("RetFromSubroutine"), f)}
            OpCode::SetRegisterToRegisterValueUsingOR(x, y) => { Display::fmt(&format!("SetRegisterToRegisterValueUsingOR(x={x}, y={y}"), f)}
        }
    }
}
