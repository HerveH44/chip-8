use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Debug, PartialEq )]
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
    SetRegisterToRegisterValueUsingAND(u8, u8), // AND Vx, Vy
    SetRegisterToRegisterValueUsingXOR(u8, u8), // XOR Vx, Vy
    AddRegisterToRegister(u8, u8), // ADD Vx, Vy
    SubRegisterToRegister(u8, u8), // SUB Vx, Vy
    ShiftRightRegisterFromRegister(u8, u8), // SHR Vx, Vy
    ShiftLeftRegisterFromRegister(u8, u8), // SHL Vx, Vy
    SubRegisterToRegisterReverse(u8, u8) , // SUBN Vx, Vy
    LoadFromMemoryToRegisters(u8), // LD Vx, [I]
    LoadFromRegistersToMemory(u8), // LD [I], Vx
    StoreBCDRepresentationOfRegister(u8), // LD B, Vx
    AddRegisterValueToIndex(u8), // ADD I, Vx
    SetRegisterFromDelayTimer(u8), // LD Vx, DT
    SetSoundTimerFromRegister(u8), // LD ST, Vx
    SetDelayTimerFromRegister(u8), // LD DT, Vx
    SkipIfBothRegistersNotEqual(u8, u8), // SNE Vx, Vy
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
            OpCode::SetRegisterToRegisterValueUsingAND(x, y) => { Display::fmt(&format!("SetRegisterToRegisterValueUsingAND(x={x}, y={y}"), f)}
            OpCode::SetRegisterToRegisterValueUsingXOR(x, y) => { Display::fmt(&format!("SetRegisterToRegisterValueUsingXOR(x={x}, y={y}"), f)}
            OpCode::AddRegisterToRegister(x, y) => { Display::fmt(&format!("AddRegisterToRegister(x={x}, y={y}"), f)}
            OpCode::SubRegisterToRegister(x, y) => { Display::fmt(&format!("SubRegisterToRegister(x={x}, y={y}"), f)}
            OpCode::ShiftRightRegisterFromRegister(x, y) => { Display::fmt(&format!("ShiftRightRegisterFromRegister(x={x}, y={y}"), f)}
            OpCode::ShiftLeftRegisterFromRegister(x, y) => { Display::fmt(&format!("ShiftLeftRegisterFromRegister(x={x}, y={y}"), f)}
            OpCode::SubRegisterToRegisterReverse(x, y) => { Display::fmt(&format!("SubRegisterToRegisterReverse(x={x}, y={y}"), f)}
            OpCode::LoadFromMemoryToRegisters(x) => { Display::fmt(&format!("LoadFromMemoryToRegisters(x={x}"), f)}
            OpCode::LoadFromRegistersToMemory(x) => { Display::fmt(&format!("LoadFromRegistersToMemory(x={x}"), f)}
            OpCode::StoreBCDRepresentationOfRegister(x) => { Display::fmt(&format!("StoreBCDRepresentationOfRegister(x={x}"), f)}
            OpCode::AddRegisterValueToIndex(x) => { Display::fmt(&format!("AddRegisterValueToIndex(x={x}"), f)}
            OpCode::SetRegisterFromDelayTimer(x) => { Display::fmt(&format!("SetDelayTimerValueToRegister(x={x}"), f)}
            OpCode::SetDelayTimerFromRegister(x) => { Display::fmt(&format!("SetDelayTimerFromRegister(x={x}"), f)}
            OpCode::SetSoundTimerFromRegister(x) => { Display::fmt(&format!("SetSoundTimerFromRegister(x={x}"), f)}
            OpCode::SkipIfBothRegistersNotEqual(x, y) => { Display::fmt(&format!("SkipIfBothRegistersNotEqual(x={x}, y={y}"), f)}
        }
    }
}
