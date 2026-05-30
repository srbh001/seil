// Compiler backend optimization passes for the IITB-RISC pipelined architecture.
//
// Currently implements: List Scheduling
//   Detects RAW (read-after-write) hazards caused by the 6-stage pipeline
//   and resolves them by reordering independent instructions or inserting
//   NOPs (ADI R0, R0, 0) where no safe reordering exists.
//
// Hazard window = 2: in a 6-stage pipeline (IF→ID→RR→EX→MEM→WB), a result
// written in WB (stage 6) is needed in RR (stage 3). Instructions issued
// 1 or 2 cycles after the writer reach RR before WB completes — those two
// slots are the hazard window.

use crate::lexer::Processor;
use crate::parser::Instruction;

const HAZARD_WINDOW: usize = 2;

fn writes_to(instr: &Instruction) -> Option<i32> {
    match instr.opcode.as_str() {
        "SW" | "BEQ" | "BLT" | "BLE" | "JLR" | "JRI" | "SM" => None,
        _ => Some(instr.reg_a),
    }
}

fn reads_from(instr: &Instruction) -> Vec<i32> {
    let mut reads = Vec::new();
    match instr.opcode.as_str() {
        "ADA"|"ADC"|"ADZ"|"AWC"|"ACA"|"ACC"|"ACZ"|"ACW"|
        "NDU"|"NDC"|"NDZ"|"NCU"|"NCC"|"NCZ" => {
            if let Some(b) = instr.reg_b { reads.push(b); }
            if let Some(c) = instr.reg_c { reads.push(c); }
        }
        "ADI" | "LW" => {
            if let Some(b) = instr.reg_b { reads.push(b); }
        }
        "SW" | "BEQ" | "BLT" | "BLE" => {
            reads.push(instr.reg_a);
            if let Some(b) = instr.reg_b { reads.push(b); }
        }
        "JLR" | "JRI" | "SM" => { reads.push(instr.reg_a); }
        _ => {}
    }
    reads
}

fn has_raw_hazard(recent_output: &[Instruction], next: &Instruction) -> bool {
    let next_reads = reads_from(next);
    recent_output.iter().any(|prev| {
        writes_to(prev).map_or(false, |w| next_reads.contains(&w))
    })
}

fn has_any_dependency(a: &Instruction, b: &Instruction) -> bool {
    let a_writes = writes_to(a);
    let b_writes = writes_to(b);
    let a_reads = reads_from(a);
    let b_reads = reads_from(b);
    if a_writes.map_or(false, |w| b_reads.contains(&w)) { return true; }
    if b_writes.map_or(false, |w| a_reads.contains(&w)) { return true; }
    if let (Some(aw), Some(bw)) = (a_writes, b_writes) {
        if aw == bw { return true; }
    }
    false
}

fn make_nop() -> Instruction {
    Instruction::new("ADI".to_string(), 0, Some(0), None, 0, 0, 0, Processor::Pipelined)
}

/// Reorders instructions to eliminate RAW pipeline hazards.
/// Independent instructions are moved into hazard slots.
/// NOPs are inserted only when no safe reordering is possible.
pub fn schedule(instructions: Vec<Instruction>) -> Vec<Instruction> {
    let mut output: Vec<Instruction> = Vec::new();
    let mut remaining = instructions;

    while !remaining.is_empty() {
        let window_start = output.len().saturating_sub(HAZARD_WINDOW);
        let recent = &output[window_start..];

        if has_raw_hazard(recent, &remaining[0]) {
            let mut found = false;
            for j in 1..remaining.len() {
                if !has_raw_hazard(recent, &remaining[j])
                    && !has_any_dependency(&remaining[0], &remaining[j])
                {
                    let filler = remaining.remove(j);
                    output.push(filler);
                    found = true;
                    break;
                }
            }
            if !found {
                output.push(make_nop());
            }
        } else {
            let instr = remaining.remove(0);
            output.push(instr);
        }
    }
    output
}
