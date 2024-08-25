pub struct IF_ID {
    IR_in: u16,
    PC_in: u16,

    IR_out: u16,
    PC_out: u16,
    Taken_branch: bool,
    Enable_IF_ID: bool,
    reg_file_wr_out: bool,
    mem_wr_out: bool,
    reg_file_wr_in: bool,
    mem_wr_in: bool,
    clk: bool,
}

impl IF_ID {
    pub fn new() -> IF_ID {
        IF_ID {
            IR_in: 0,
            PC_in: 0,

            IR_out: 0,
            PC_out: 0,

            Taken_branch: false,
            Enable_IF_ID: false,
            reg_file_wr_out: false,
            mem_wr_out: false,
            reg_file_wr_in: false,
            mem_wr_in: false,
            clk: false,
        }
    }

    pub fn IF_ID_fetch(&mut self) {
        if self.clk && self.Enable_IF_ID {
            self.IR_out = self.IR_in;
            self.PC_out = self.PC_in;

            if self.Taken_branch {
                self.reg_file_wr_out = false;
                self.mem_wr_out = false;
            } else {
                self.reg_file_wr_out = self.reg_file_wr_in;
                self.mem_wr_out = self.mem_wr_in;
            }
        }
    }
}

pub struct RegDecodeOperandrd {
    opcode_in: [u8; 4],
    zcbit_in: [u8; 2],
    reg_a_in: [u8; 3],
    reg_b_in: [u8; 3],
    reg_c_in: [u8; 3],
    alu_cntrl_in: [u8; 3],
    pc_in: [u8; 16],
    imm_16_in: [u8; 16],
    pc_2in: [u8; 16],
    reg_file_wr_in: bool,
    mem_wr_in: bool,
    clk: bool,
    carry_write_in: bool,
    zero_write_in: bool,
    taken_branch: bool,
    enable_id_rr: bool,

    zcbit_out: [u8; 2],
    opcode_out: [u8; 4],
    reg_a_out: [u8; 3],
    reg_b_out: [u8; 3],
    reg_c_out: [u8; 3],
    alu_cntrl_out: [u8; 3],
    imm_16_out: [u8; 16],
    pc_out: [u8; 16],
    pc_2out: [u8; 16],
    reg_file_wr_out: bool,
    mem_wr_out: bool,
    carry_write_out: bool,
    zero_write_out: bool,

    // Internal temporary signals
    reg_a_temp: [u8; 3],
    reg_b_temp: [u8; 3],
    reg_c_temp: [u8; 3],
    pc_temp: [u8; 16],
    pc_2temp: [u8; 16],
    imm_16_temp: [u8; 16],
    alu_cntrl_temp: [u8; 3],
    reg_file_wr_temp: bool,
    mem_wr_temp: bool,
    carry_write_temp: bool,
    zero_write_temp: bool,
    opcode_temp: [u8; 4],
    zcbit: [u8; 2],
}

impl RegDecodeOperandrd {
    pub fn new() -> RegDecodeOperandrd {
        RegDecodeOperandrd {
            opcode_in: [0; 4],
            zcbit_in: [0; 2],
            reg_a_in: [0; 3],
            reg_b_in: [0; 3],
            reg_c_in: [0; 3],
            alu_cntrl_in: [0; 3],
            pc_in: [0; 16],
            imm_16_in: [0; 16],
            pc_2in: [0; 16],
            reg_file_wr_in: false,
            mem_wr_in: false,
            clk: false,
            carry_write_in: false,
            zero_write_in: false,
            taken_branch: false,
            enable_id_rr: false,

            zcbit_out: [0; 2],
            opcode_out: [0; 4],
            reg_a_out: [0; 3],
            reg_b_out: [0; 3],
            reg_c_out: [0; 3],
            alu_cntrl_out: [0; 3],
            imm_16_out: [0; 16],
            pc_out: [0; 16],
            pc_2out: [0; 16],
            reg_file_wr_out: false,
            mem_wr_out: false,
            carry_write_out: false,
            zero_write_out: false,

            // Initialize internal temporary signals
            reg_a_temp: [0; 3],
            reg_b_temp: [0; 3],
            reg_c_temp: [0; 3],
            pc_temp: [0; 16],
            pc_2temp: [0; 16],
            imm_16_temp: [0; 16],
            alu_cntrl_temp: [0; 3],
            reg_file_wr_temp: true,
            mem_wr_temp: true,
            carry_write_temp: false,
            zero_write_temp: false,
            opcode_temp: [0; 4],
            zcbit: [0; 2],
        }
    }

    pub fn reg_decode_operandrd(&mut self) {
        if self.clk && self.enable_id_rr {
            self.reg_a_temp = self.reg_a_in;
            self.reg_b_temp = self.reg_b_in;
            self.reg_c_temp = self.reg_c_in;
            self.pc_2temp = self.pc_2in;

            self.pc_temp = self.pc_in;
            if self.taken_branch {
                self.reg_file_wr_temp = false;
                self.mem_wr_temp = false;
            } else {
                self.reg_file_wr_temp = self.reg_file_wr_in;
                self.mem_wr_temp = self.mem_wr_in;
            }

            self.imm_16_temp = self.imm_16_in;
            self.alu_cntrl_temp = self.alu_cntrl_in;
            self.carry_write_temp = self.carry_write_in;
            self.zero_write_temp = self.zero_write_in;
            self.opcode_temp = self.opcode_in;
            self.zcbit = self.zcbit_in;
        }

        self.imm_16_out = self.imm_16_temp;
        self.reg_a_out = self.reg_a_temp;
        self.reg_b_out = self.reg_b_temp;
        self.reg_c_out = self.reg_c_temp;
        self.alu_cntrl_out = self.alu_cntrl_temp;
        self.carry_write_out = self.carry_write_temp;
        self.zero_write_out = self.zero_write_temp;
        self.pc_out = self.pc_temp;
        self.reg_file_wr_out = self.reg_file_wr_temp;
        self.mem_wr_out = self.mem_wr_temp;
        self.pc_2out = self.pc_2temp;
        self.opcode_out = self.opcode_temp;
        self.zcbit_out = self.zcbit;
    }
}
