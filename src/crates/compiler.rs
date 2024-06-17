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
