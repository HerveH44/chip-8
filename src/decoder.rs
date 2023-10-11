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
        (0x1, _, _, _) => OpCode::Jump(nnn),
        (0x3, _, _, _) => OpCode::SkipIfRegisterEquals(x as u8, nn),
        (0x4, _, _, _) => OpCode::SkipIfRegisterNotEquals(x as u8, nn),
        (0x6, _, _, _) => OpCode::SetRegister {register: x, value: nn},
        (0x7, _, _, _) => OpCode::AddRegister {register: x, value: nn},
        (0xA, _, _, _) => OpCode::SetIndex(nnn),
        (0xD, _, _, _) => OpCode::Display(x, y, n),
        _ => OpCode::Unknown
    };

    info!("instruction={instruction}|kind={kind}|x={x}|y={y}|n={n}|nn={nn}|nnn={nnn}|opcode={opcode}");
    return opcode
}
