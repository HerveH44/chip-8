use log::info;

use crate::opcode::OpCode;

pub fn decode_instruction(instruction: u16) -> OpCode {
    let kind = (instruction & 0xF000) >> 12;
    let x = ((instruction & 0x0F00) >> 8) as usize;
    let y = ((instruction & 0x00F0) >> 4) as usize;
    let n = (instruction & 0x000F) as u8;
    let nn = (instruction & 0x00FF) as u8;
    let nnn = instruction & 0x0FFF;

    let opcode = match (kind, x, y, n) {
        (0x0, 0x0, 0xE, 0x0) => OpCode::ClearScreen,
        (0x0, 0x0, 0xE, 0xE) => OpCode::RetFromSubroutine,
        (0x1, _, _, _) => OpCode::Jump(nnn),
        (0x2, _, _, _) => OpCode::CallSubroutine(nnn),
        (0x3, _, _, _) => OpCode::SkipIfRegisterEquals(x as u8, nn),
        (0x4, _, _, _) => OpCode::SkipIfRegisterNotEquals(x as u8, nn),
        (0x5, _, _, 0x0) => OpCode::SkipIfBothRegistersEqual(x as u8, y as u8),
        (0x6, _, _, _) => OpCode::SetRegister {
            register: x,
            value: nn,
        },
        (0x7, _, _, _) => OpCode::AddRegister {
            register: x,
            value: nn,
        },
        (0x8, _, _, 0x0) => OpCode::SetRegisterToRegisterValue(x as u8, y as u8),
        (0x8, _, _, 0x1) => OpCode::SetRegisterToRegisterValueUsingOR(x as u8, y as u8),
        (0x8, _, _, 0x2) => OpCode::SetRegisterToRegisterValueUsingAND(x as u8, y as u8),
        (0x8, _, _, 0x3) => OpCode::SetRegisterToRegisterValueUsingXOR(x as u8, y as u8),
        (0x8, _, _, 0x4) => OpCode::AddRegisterToRegister(x as u8, y as u8),
        (0x8, _, _, 0x5) => OpCode::SubRegisterToRegister(x as u8, y as u8),
        (0x8, _, _, 0x6) => OpCode::ShiftRightRegisterFromRegister(x as u8, y as u8),
        (0x8, _, _, 0x7) => OpCode::SubRegisterToRegisterReverse(x as u8, y as u8),
        (0x8, _, _, 0xE) => OpCode::ShiftLeftRegisterFromRegister(x as u8, y as u8),
        (0x9, _, _, 0x0) => OpCode::SkipIfBothRegistersNotEqual(x as u8, y as u8),
        (0xA, _, _, _) => OpCode::SetIndex(nnn),
        (0xB, _, _, _) => OpCode::JumpWithV0Offset(nnn),
        (0xC, _, _, _) => OpCode::SetRegisterWithRandom(x, nn),
        (0xD, _, _, _) => OpCode::Draw(x, y, n),
        (0xE, _, 0x9, 0xE) => OpCode::SkipIfKey(x),
        (0xE, _, 0xA, 0x1) => OpCode::SkipIfNotKey(x),
        (0xF, _, 0x0, 0x7) => OpCode::SetRegisterFromDelayTimer(x as u8),
        (0xF, _, 0x0, 0xA) => OpCode::GetKey(x),
        (0xF, _, 0x1, 0x5) => OpCode::SetDelayTimerFromRegister(x as u8),
        (0xF, _, 0x1, 0x8) => OpCode::SetSoundTimerFromRegister(x as u8),
        (0xF, _, 0x1, 0xE) => OpCode::AddRegisterValueToIndex(x as u8),
        (0xF, _, 0x2, 0x9) => OpCode::LoadSpriteRepresentationInMemory(x as u8),
        (0xF, _, 0x3, 0x3) => OpCode::StoreBCDRepresentationOfRegister(x as u8),
        (0xF, _, 0x5, 0x5) => OpCode::LoadFromRegistersToMemory(x as u8),
        (0xF, _, 0x6, 0x5) => OpCode::LoadFromMemoryToRegisters(x as u8),
        _ => OpCode::Unknown,
    };

    info!("instruction={instruction}|opcode={opcode}");
    opcode
}
