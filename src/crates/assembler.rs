// This crate provides a simple way to rearrange instructions to mitigate the performance impact of pipeline data hazards.
// For more information, see [Instruction Scheduling](https://en.wikipedia.org/wiki/Instruction_scheduling).
//
// Data Hazards in the IITB RISC-V Processor:
//
// 1. RAW (Read After Write) Hazard:
//    - Example: ADD R1, R2, R3; LW R4, R1, R5
//    - The ADD instruction writes to R1, and the subsequent LW instruction reads from R1.
//
// 2. WAR (Write After Read) Hazard:
//    - Example: ADD R1, R2, R3; ADD R4, R1, R5
//    - The first ADD instruction reads from R1, and the second ADD instruction writes to R1 (not a hazard in this case).
//
// 3. WAW (Write After Write) Hazard:
//    - Example: ADD R2, R1, R3; LW R5, R1, 000001
//    - The ADD instruction writes to R2, and the LW instruction writes to R1.
//
// Method used to mitigate hazards: [List Scheduling](https://en.wikipedia.org/wiki/List_scheduling).

use crate::lexer;
use crate::parser::{Instruction, Parser};

pub fn dissasembler(parser: Parser) {
    let instructions = parser.instructions;
    let labels = parser.labels;
    let label_line_numbers = parser.label_line_numbers;
    let lexer_string: String = parser.lexer.input.iter().collect();
    let mut lines_traversed = 0;

    for instruction in instructions.iter() {
        if instruction.line_number > lines_traversed {
            let line = lexer_string
                .lines()
                .nth(instruction.line_number - 1)
                .unwrap();
            println!(
                "[INFO] Instruction: {:016b}, {}",
                instruction_to_binary(instruction.clone()),
                instruction.opcode
            );
            lines_traversed = instruction.line_number;
        }
    }
}

fn instruction_to_binary(instruction: Instruction) -> i16 {
    let instruction_bin: i16;

    let opcode_bin = String::from(opcode_to_binary(instruction.opcode.as_str()));
    let opcode = instruction.clone().opcode;
    let reg_a = register_to_binary(instruction.reg_a);

    if lexer::OPCODES_WITH_SINGLE_REGISTER_PIPELINED.contains(&opcode.as_str()) {
        if instruction.imm > 511 {
            panic!("[ERROR] Immediate value out of range");
        }
        let imm = format!("{:09b}", instruction.imm);

        let instruction_bin_str = format!("{}{}{}", opcode_bin, reg_a, imm);
        instruction_bin = i16::from_str_radix(&instruction_bin_str, 2).unwrap();
    } else if lexer::OPCODES_WITH_TWO_REGISTERS_PIPELINED.contains(&opcode.as_str()) {
        let reg_b = register_to_binary(instruction.reg_b.expect("[ERROR] Missing register B"));

        if instruction.imm > 63 {
            panic!("[ERROR] Immediate value out of range");
        }
        let imm = format!("{:06b}", instruction.imm);

        let instruction_bin_str = format!("{}{}{}{}", opcode_bin, reg_a, reg_b, imm);

        instruction_bin = i16::from_str_radix(&instruction_bin_str, 2).unwrap();
    } else if lexer::OPCODES_WITH_THREE_REGISTERS_PIPELINED.contains(&opcode.as_str()) {
        let reg_b = register_to_binary(instruction.reg_b.expect("[ERROR] Missing register B"));
        let reg_c = register_to_binary(instruction.reg_c.expect("[ERROR] Missing register C"));

        let instruction_bin_str = format!("{}{}{}{}", opcode_bin, reg_a, reg_b, reg_c);

        instruction_bin = i16::from_str_radix(&instruction_bin_str, 2).unwrap();
    } else {
        panic!("Invalid opcode: {}", instruction.opcode.as_str());
        // Technically, this should never be reached.
    }

    instruction_bin
}

fn register_to_binary(reg: i32) -> String {
    if reg > 7 {
        panic!("[ERROR] Register out of range");
    }
    format!("{:03b}", reg)
}

fn opcode_to_binary(opcode: &str) -> &str {
    match opcode {
        "ADA" => "0001", // RA RB RC 0 00
        "ADC" => "0001", // RA RB RC 0 10
        "ADZ" => "0001", // RA RB RC 0 01
        "AWC" => "0001", // RA RB RC 0 11
        "ACA" => "0001", // RA RB RC 1 00
        "ACC" => "0001", // RA RB RC 1 10
        "ACZ" => "0001", // RA RB RC 1 01
        "ACW" => "0001", // RA RB RC 1 11
        "ADI" => "0000", //RA RB IMM6
        "NDU" => "0010", // RA RB RC 0 00
        "NDC" => "0010", // RA RB RC 0 10
        "NDZ" => "0010", // RA RB RC 0 01
        "NCU" => "0010", // RA RB RC 1 00
        "NCC" => "0010", // RA RB RC 1 10
        "NCZ" => "0010", // RA RB RC 1 01
        "LLI" => "0011", // RA IMM9
        "LW" => "0100",  //0 RA RB IMM6
        "SW" => "0101",  //1 RA RB IMM6
        "LM" => "0110",  //0 RA 0 + 8 bits corresponding to R0 to R7
        "SM" => "0111",  //1 RA 0 + 8 bits corresponding to R0 to R7
        "BEQ" => "1000", // RA RB IMM6
        "BLT" => "1001", // RA RB IMM6
        "BLE" => "1010", // RA RB IMM6
        "JAL" => "1100", // RA IMM9
        "JLR" => "1101", // RA 0 0000
        "JRI" => "1111", //"; RA 0 000
        _ => panic!("Invalid opcode"),
    }
}

fn immediate6_to_binary(imm: i32) -> String {
    let imm_binary = format!("{:6b}", imm.to_owned());

    imm_binary
}

fn immediate9_to_binary(imm: i32) -> String {
    let imm_binary = format!("{:9b}", imm.to_owned());

    imm_binary
}
