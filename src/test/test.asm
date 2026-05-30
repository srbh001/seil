; IITB-RISC Pipelined Architecture — Assembler Test
;
; Demonstrates: ORG, labels, all 26 instruction types,
;               RAW hazard sites, memory access, branching.
;
; Register convention:
;   R0 = 0  (base address / zero)
;   R1 = first operand / scratch
;   R2 = second operand / scratch
;   R3 = result
;   R4-R7 = general purpose

ORG 0x0000

; ==========================================================
; INITIALIZATION
; ==========================================================
INIT:
    LLI  R0, 0           ; R0 = 0   (zero / memory base)
    LLI  R1, 12          ; R1 = 12
    LLI  R2, 7           ; R2 = 7

; ==========================================================
; 3-REGISTER ALU
; ADA/ADZ/ADC/AWC write RA = RB + RC (with flag conditions)
; ACA/ACZ/ACC/ACW write RA = RB + RC + carry
; NDU/NDZ/NDC / NCU/NCZ/NCC write RA = ~(RB & RC)
; ==========================================================
    ADA  R3, R1, R2      ; R3 = R1 + R2 = 19   (unconditional)
    ADZ  R4, R1, R2      ; R4 = R1 + R2 if Z=1
    ADC  R5, R1, R2      ; R5 = R1 + R2 if C=1
    AWC  R6, R1, R2      ; R6 = R1 + R2 + carry
    ACA  R7, R1, R2      ; R7 = R1 + R2 + C (unconditional)
    ACZ  R4, R1, R2      ; R4 = R1 + R2 + C if Z=1
    ACC  R5, R1, R2      ; R5 = R1 + R2 + C if C=1
    ACW  R6, R1, R2      ; R6 = R1 + R2 + C (write carry)
    NDU  R7, R1, R2      ; R7 = ~(R1 & R2) unconditional
    NDZ  R4, R1, R2      ; R4 = ~(R1 & R2) if Z=1
    NDC  R5, R1, R2      ; R5 = ~(R1 & R2) if C=1
    NCU  R6, R1, R2      ; R6 = ~(R1 & R2) + C unconditional
    NCZ  R7, R1, R2      ; R7 = ~(R1 & R2) + C if Z=1
    NCC  R4, R1, R2      ; R4 = ~(R1 & R2) + C if C=1

; ==========================================================
; 2-REGISTER (reg + immediate)
;
; RAW HAZARD SITE:
;   ADA wrote R3 at the top of this section. The two ADI
;   instructions below read R3 immediately — within the
;   2-cycle hazard window of the 6-stage pipeline.
;   The scheduler should fill these slots or insert NOPs.
; ==========================================================
    ADI  R5, R3, 8       ; R5 = R3 + 8 = 27   *** RAW on R3 ***
    ADI  R6, R3, 5       ; R6 = R3 + 5 = 24   *** RAW on R3 ***

; ==========================================================
; MEMORY OPERATIONS
; ==========================================================
    SW   R5, R0, 0       ; mem[0] = 27
    SW   R6, R0, 2       ; mem[2] = 24
    LW   R5, R0, 0       ; R5 = mem[0] = 27
    LW   R6, R0, 2       ; R6 = mem[2] = 24

; Load/store multiple
    LM   R0, 3           ; load R0 and R1 from memory (bitmask = 0b00000011)
    SM   R0, 3           ; store R0 and R1 to memory

; ==========================================================
; BRANCH (numeric offsets — scheduler-safe)
; offset is PC-relative: PC_new = (PC + 1) + offset
; ==========================================================
BRANCH_TEST:
    BEQ  R5, R6, 2       ; if R5 == R6, skip 2 instructions
    BLT  R5, R6, 1       ; if R5 <  R6, skip 1 instruction
    BLE  R5, R6, 0       ; if R5 <= R6, skip 0 (no-op branch)

; ==========================================================
; JUMP AND LINK
; ==========================================================
JUMP_TEST:
    JAL  R7, 0           ; jump to addr 0, save PC+1 in R7
    JLR  R7, 0           ; jump to address stored in R7
    JRI  R7, 0           ; return via R7 (halt / return)
