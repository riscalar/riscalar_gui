use super::{FunctionalCore, Mode};
use crate::cpu::{exception::Exception, instr::*, register::RegisterFile};

impl FunctionalCore {
    pub fn execute(&mut self, instr: u64) -> Result<(), Exception> {
        self.pc += 4;

        let opcode = instr & 0x0000007F;
        let rd = (instr & 0x00000F80) >> 7;
        let rs1 = (instr & 0x000F8000) >> 15;
        let rs2 = (instr & 0x01F00000) >> 20;
        let funct3 = (instr & 0x00007000) >> 12;
        let funct7 = (instr & 0xFE000000) >> 25;
        let imm20 = (instr & 0xFFFFF000) as i32 as i64 as u64;
        let imm12 = (instr as i32 as i64) >> 20;

        match opcode {
            OPCODE_LUI => self.execute_lui(rd, imm20),
            OPCODE_AUIPC => self.execute_auipc(rd, imm20),
            OPCODE_JAL => self.execute_jal(rd, instr),
            OPCODE_JALR => self.execute_jalr(rd, rs1, imm12),
            OPCODE_BRANCH => self.execute_branch(funct3, rs1, rs2, instr),
            OPCODE_LOAD => self.execute_load(rd, funct3, rs1, instr),
            OPCODE_STORE => self.execute_store(funct3, rs1, rs2, instr),
            OPCODE_OP_IMM => self.execute_op_imm(rd, funct3, rs1, imm12 as u64, funct7, instr),
            OPCODE_OP_IMM32 => self.execute_op_imm32(rd, funct3, rs1, imm12 as u64, funct7, instr),
            OPCODE_OP => self.execute_op(rd, funct3, rs1, rs2, funct7, instr),
            OPCODE_OP32 => self.execute_op32(rd, funct3, rs1, rs2, funct7, instr),
            OPCODE_MISC_MEM => self.execute_misc_mem(instr),
            OPCODE_SYSTEM => self.execute_system(rd, funct3, rs1, instr),
            _ => Err(Exception::IllegalInstruction(instr)),
        }
    }

    #[inline]
    fn execute_load(
        &mut self,
        rd: u64,
        funct3: u64,
        rs1: u64,
        instr: u64,
    ) -> Result<(), Exception> {
        self.stats.instr.instr_load += 1;
        // self.("memory_instructions");
        // self.("load_instructions");

        let imm = ((instr as i32 as i64) >> 20) as u64;
        let addr = self.xreg.read(rs1).wrapping_add(imm);

        match funct3 {
            0b000 => {
                // lb
                let value = self.read(addr, 8)?;
                self.xreg.write(rd, value as i8 as i64 as u64);
            }
            0b001 => {
                // lh
                let value = self.read(addr, 16)?;
                self.xreg.write(rd, value as i16 as i64 as u64);
            }
            0b010 => {
                // lw
                let value = self.read(addr, 32)?;
                self.xreg.write(rd, value as i32 as i64 as u64);
            }
            0b011 => {
                // ld
                let value = self.read(addr, 64)?;
                self.xreg.write(rd, value as i64 as u64);
            }
            0b100 => {
                // lbu
                let value = self.read(addr, 8)?;
                self.xreg.write(rd, value);
            }
            0b101 => {
                // lhu
                let value = self.read(addr, 16)?;
                self.xreg.write(rd, value);
            }
            0b110 => {
                // lwu
                let value = self.read(addr, 32)?;
                self.xreg.write(rd, value);
            }
            _ => return Err(Exception::IllegalInstruction(instr)),
        }
        Ok(())
    }

    #[inline]
    fn execute_store(
        &mut self,
        funct3: u64,
        rs1: u64,
        rs2: u64,
        instr: u64,
    ) -> Result<(), Exception> {
        self.stats.instr.instr_store += 1;

        // self.("memory_instructions");
        // self.("store_instructions");

        let imm = (((instr & 0xfe000000) as i32 as i64 >> 20) as u64) | ((instr >> 7) & 0x1f);
        let rs1 = self.xreg.read(rs1);
        let addr = rs1.wrapping_add(imm);

        let value = self.xreg.read(rs2);

        match funct3 {
            0b000 => {
                // sb
                self.write(addr, value, 8)?
            }
            0b001 => {
                // sh
                self.write(addr, value, 16)?
            }
            0b010 => {
                // sw
                self.write(addr, value, 32)?
            }
            0b011 => {
                // sd
                self.write(addr, value, 64)?
            }
            _ => return Err(Exception::IllegalInstruction(instr)),
        }
        Ok(())
    }

    #[inline]
    fn execute_op_imm(
        &mut self,
        rd: u64,
        funct3: u64,
        rs1: u64,
        imm12: u64,
        funct7: u64,
        instr: u64,
    ) -> Result<(), Exception> {
        self.stats.instr.instr_arith += 1;
        // self.("alu_instructions");

        let a = self.xreg.read(rs1);
        let funct6 = funct7 >> 1;
        match funct3 {
            0b000 => {
                // addi
                self.xreg.write(rd, a.wrapping_add(imm12))
            }
            0b001 => {
                // slli
                let shamt = (instr >> 20) & 0x3f;
                self.xreg.write(rd, a << shamt);
            }
            0b010 => {
                //slti
                self.xreg
                    .write(rd, if (a as i64) < (imm12 as i64) { 1 } else { 0 })
            }
            0b011 => {
                //sltiu
                self.xreg.write(rd, if a < imm12 { 1 } else { 0 })
            }
            0b100 => {
                //xori
                self.xreg.write(rd, a ^ imm12);
            }
            0b101 => {
                let shamt = (instr >> 20) & 0x3f;

                match funct6 {
                    0b00_0000 => {
                        // srli
                        self.xreg.write(rd, a >> shamt)
                    }
                    0b01_0000 => {
                        // srai
                        self.xreg.write(rd, ((a as i64) >> shamt) as u64)
                    }
                    _ => return Err(Exception::IllegalInstruction(instr)),
                }
            }
            0b110 => {
                // ori
                self.xreg.write(rd, a | imm12);
            }
            0b111 => {
                // andi
                self.xreg.write(rd, a & imm12)
            }
            _ => return Err(Exception::IllegalInstruction(instr)),
        }
        Ok(())
    }

    #[inline]
    fn execute_op_imm32(
        &mut self,
        rd: u64,
        funct3: u64,
        rs1: u64,
        imm12: u64,
        funct7: u64,
        instr: u64,
    ) -> Result<(), Exception> {
        self.stats.instr.instr_arith += 1;
        // self.("alu_instructions");

        let a = self.xreg.read(rs1);
        let funct6 = funct7 >> 1;
        let shamt = (imm12 & 0x1F) as u32;
        match funct3 {
            0b000 => {
                // addiw
                self.xreg
                    .write(rd, a.wrapping_add(imm12) as i32 as i64 as u64)
            }
            0b001 => {
                // slliw
                self.xreg.write(rd, (a << shamt) as i32 as i64 as u64);
            }
            0b101 => {
                match funct6 {
                    0b00_0000 => {
                        // srliw
                        self.xreg
                            .write(rd, ((a as u32) >> shamt) as i32 as i64 as u64)
                    }
                    0b01_0000 => {
                        // sraiw
                        self.xreg.write(rd, ((a as i32) >> shamt) as i64 as u64)
                    }
                    _ => return Err(Exception::IllegalInstruction(instr)),
                }
            }
            _ => return Err(Exception::IllegalInstruction(instr)),
        }
        Ok(())
    }

    #[inline]
    fn execute_op(
        &mut self,
        rd: u64,
        funct3: u64,
        rs1: u64,
        rs2: u64,
        funct7: u64,
        instr: u64,
    ) -> Result<(), Exception> {
        self.stats.instr.instr_arith += 1;
        // self.("alu_instructions");

        let a = self.xreg.read(rs1);
        let b = self.xreg.read(rs2);
        match (funct3, funct7) {
            (0x0, 0x00) => {
                //add
                self.xreg.write(rd, a.wrapping_add(b))
            }
            (0x0, 0x01) => {
                //mul
                self.xreg
                    .write(rd, (a as i64).wrapping_mul(b as i64) as u64)
            }
            (0x0, 0x20) => {
                //sub
                self.xreg.write(rd, a.wrapping_sub(b))
            }
            (0x1, 0x00) => {
                // sll
                let shamt = b & 0x3f;
                self.xreg.write(rd, a << shamt);
            }
            (0x1, 0x01) => {
                // mulh
                // signed × signed
                self.xreg.write(
                    rd,
                    ((a as i64 as i128).wrapping_mul(b as i64 as i128) >> 64) as u64,
                );
            }
            (0x2, 0x00) => {
                // slt
                self.xreg
                    .write(rd, if (a as i64) < (b as i64) { 1 } else { 0 });
            }
            (0x2, 0x01) => {
                // mulhsu
                // signed × unsigned
                self.xreg.write(
                    rd,
                    ((a as i64 as i128 as u128).wrapping_mul(b as u128) >> 64) as u64,
                );
            }
            (0x3, 0x00) => {
                // sltu
                self.xreg.write(rd, if a < b { 1 } else { 0 });
            }
            (0x3, 0x01) => {
                // mulhu
                // unsigned × unsigned
                self.xreg
                    .write(rd, ((a as u128).wrapping_mul(b as u128) >> 64) as u64);
            }
            (0x4, 0x00) => {
                // xor
                self.xreg.write(rd, a ^ b);
            }
            (0x4, 0x01) => {
                // div
                self.xreg.write(
                    rd,
                    if b as i64 == 0 {
                        // Division by zero
                        // Set DZ (Divide by Zero) flag to 1.
                        // self.state.write_bit(FCSR, 3, 1);
                        // "The quotient of division by zero has all bits set"
                        u64::MAX
                    } else if a as i64 == i64::MIN && b as i64 == -1 {
                        // Overflow
                        // "The quotient of a signed division with overflow is equal to the
                        // dividend"
                        a as i64 as u64
                    } else {
                        // "division of rs1 by rs2, rounding towards zero"
                        (a as i64).wrapping_div(b as i64) as u64
                    },
                );
            }
            (0x5, 0x00) => {
                // srl
                let shamt = b & 0x3f;
                self.xreg.write(rd, a >> shamt);
            }
            (0x5, 0x01) => {
                // divu
                self.xreg.write(
                    rd,
                    if b == 0 {
                        // Division by zero
                        // Set DZ (Divide by Zero) flag to 1.
                        // self.state.write_bit(FCSR, 3, 1);
                        // "The quotient of division by zero has all bits set"
                        u64::MAX
                    } else {
                        // "division of rs1 by rs2, rounding towards zero"
                        a.wrapping_div(b)
                    },
                );
            }
            (0x5, 0x20) => {
                // sra
                let shamt = b & 0x3f;
                self.xreg.write(rd, ((a as i64) >> shamt) as u64);
            }
            (0x6, 0x00) => {
                // or
                self.xreg.write(rd, a | b);
            }
            (0x6, 0x01) => {
                // rem
                self.xreg.write(
                    rd,
                    if b as i64 == 0 {
                        // Division by zero
                        // "the remainder of division by zero equals the dividend"
                        a as i64 as u64
                    } else if a as i64 == i64::MIN && b as i64 == -1 {
                        // Overflow
                        // "the remainder is zero"
                        0
                    } else {
                        // "provide the remainder of the corresponding division
                        // operation"
                        (a as i64).wrapping_rem(b as i64) as u64
                    },
                );
            }
            (0x7, 0x00) => {
                // and
                self.xreg.write(rd, a & b);
            }
            (0x7, 0x01) => {
                // remu

                self.xreg.write(
                    rd,
                    if b == 0 {
                        // Division by zero
                        // "the remainder of division by zero equals the dividend"
                        a
                    } else {
                        // "provide the remainder of the corresponding division
                        // operation"
                        a.wrapping_rem(b)
                    },
                );
            }

            (_, _) => return Err(Exception::IllegalInstruction(instr)),
        }
        Ok(())
    }

    #[inline]
    fn execute_op32(
        &mut self,
        rd: u64,
        funct3: u64,
        rs1: u64,
        rs2: u64,
        funct7: u64,
        instr: u64,
    ) -> Result<(), Exception> {
        self.stats.instr.instr_arith += 1;
        // self.("alu_instructions");

        let a = self.xreg.read(rs1);
        let b = self.xreg.read(rs2);
        let shamt = b & 0x1f;

        match (funct3, funct7) {
            (0x0, 0x00) => {
                //addw
                self.xreg.write(rd, a.wrapping_add(b) as i32 as i64 as u64)
            }
            (0x0, 0x01) => {
                //mulw
                self.xreg
                    .write(rd, (a as i32).wrapping_mul(b as i32) as i64 as u64)
            }
            (0x0, 0x20) => {
                //subw
                self.xreg.write(rd, a.wrapping_sub(b) as i32 as u64)
            }
            (0x1, 0x00) => {
                // sllw
                self.xreg.write(rd, (a << shamt) as i32 as i64 as u64);
            }
            (0x4, 0x01) => {
                // divw
                self.xreg.write(
                    rd,
                    if b as i32 == 0 {
                        // Division by zero
                        // Set DZ (Divide by Zero) flag to 1.
                        // self.state.write_bit(FCSR, 3, 1);
                        // "The quotient of division by zero has all bits set"
                        u64::MAX
                    } else if a as i32 == i32::MIN && b as i32 == -1 {
                        // Overflow
                        // "The quotient of a signed division with overflow is equal to the
                        // dividend"
                        a as i32 as i64 as u64
                    } else {
                        // "division of rs1 by rs2, rounding towards zero"
                        (a as i32).wrapping_div(b as i32) as i64 as u64
                    },
                );
            }
            (0x5, 0x00) => {
                // srlw
                self.xreg.write(rd, (a >> shamt) as i32 as i64 as u64);
            }
            (0x5, 0x01) => {
                // divuw
                self.xreg.write(
                    rd,
                    if b as u32 == 0 {
                        // Division by zero
                        // Set DZ (Divide by Zero) flag to 1.
                        // self.state.write_bit(FCSR, 3, 1);
                        // "The quotient of division by zero has all bits set"
                        u64::MAX
                    } else {
                        // "division of rs1 by rs2, rounding towards zero"
                        (a as u32).wrapping_div(b as u32) as i32 as i64 as u64
                    },
                );
            }
            (0x5, 0x20) => {
                // sraw
                self.xreg.write(rd, ((a as i32) >> shamt) as i64 as u64);
            }
            (0x6, 0x01) => {
                // remw
                self.xreg.write(
                    rd,
                    if b as i32 == 0 {
                        // Division by zero
                        // "the remainder of division by zero equals the dividend"
                        a as i32 as i64 as u64
                    } else if a as i32 == i32::MIN && b as i32 == -1 {
                        // Overflow
                        // "the remainder is zero"
                        0
                    } else {
                        // "provide the remainder of the corresponding division
                        // operation"
                        (a as i32).wrapping_rem(b as i32) as i64 as u64
                    },
                );
            }
            (0x7, 0x01) => {
                // remuw
                self.xreg.write(
                    rd,
                    if b as u32 == 0 {
                        // Division by zero
                        // "the remainder of division by zero equals the dividend"
                        a as u32 as i32 as i64 as u64
                    } else {
                        // "provide the remainder of the corresponding division
                        // operation"
                        (a as u32).wrapping_rem(b as u32) as i32 as i64 as u64
                    },
                );
            }

            (_, _) => return Err(Exception::IllegalInstruction(instr)),
        }
        Ok(())
    }

    #[inline]
    fn execute_jal(&mut self, rd: u64, instr: u64) -> Result<(), Exception> {
        self.stats.instr.instr_jump += 1;
        // self.("jump_instructions");

        self.xreg.write(rd, self.pc.wrapping_add(4));

        // imm[20|10:1|11|19:12] = inst[31|30:21|20|19:12]
        let offset = (((instr & 0x80000000) as i32 as i64 >> 11) as u64) // imm[20]
		 | (instr & 0xff000) // imm[19:12]
		 | ((instr >> 9) & 0x800) // imm[11]
		 | ((instr >> 20) & 0x7fe); // imm[10:1]

        self.pc = self.pc.wrapping_add(offset).wrapping_sub(4);

        Ok(())
    }

    #[inline]
    fn execute_jalr(&mut self, rd: u64, rs1: u64, imm12: i64) -> Result<(), Exception> {
        self.stats.instr.instr_jump += 1;
        // self.("jump_instructions");

        let next = self.pc.wrapping_add(4);

        let target = ((self.xreg.read(rs1) as i64).wrapping_add(imm12)) & !1;

        self.pc = (target as u64).wrapping_sub(4);
        self.xreg.write(rd, next);

        Ok(())
    }

    #[inline]
    fn execute_branch(
        &mut self,
        funct3: u64,
        rs1: u64,
        rs2: u64,
        instr: u64,
    ) -> Result<(), Exception> {
        self.stats.instr.instr_branch += 1;
        // self.("branch_instructions");

        // imm[12|10:5|4:1|11] = inst[31|30:25|11:8|7]
        let imm = (((instr & 0x80000000) as i32 as i64 >> 19) as u64)
        | ((instr & 0x80) << 4) // imm[11]
        | ((instr >> 20) & 0x7e0) // imm[10:5]
        | ((instr >> 7) & 0x1e); // imm[4:1]

        let a = self.xreg.read(rs1);
        let b = self.xreg.read(rs2);
        let new_pc = self.pc.wrapping_add(imm).wrapping_sub(4);

        match funct3 {
            0b000 => {
                // beq
                if a == b {
                    self.pc = new_pc
                }
            }
            0b001 => {
                // bne
                if a != b {
                    self.pc = new_pc
                }
            }
            0b100 => {
                // blt
                if (a as i64) < (b as i64) {
                    self.pc = new_pc
                }
            }
            0b101 => {
                // bge
                if (a as i64) >= (b as i64) {
                    self.pc = new_pc
                }
            }
            0b110 => {
                // bltu
                if a < b {
                    self.pc = new_pc
                }
            }
            0b111 => {
                // bgeu
                if a >= b {
                    self.pc = new_pc
                }
            }
            _ => return Err(Exception::IllegalInstruction(instr)),
        }
        Ok(())
    }

    #[inline]
    fn execute_misc_mem(&mut self, instr: u64) -> Result<(), Exception> {
        self.stats.instr.instr_fence += 1;
        // self.("fence_instructions");

        let funct3 = (instr & 0x00007000) >> 12;
        match funct3 {
            0x0 => Ok(()),
            0x1 => Ok(()),
            _ => Err(Exception::IllegalInstruction(instr)),
        }
    }

    #[inline]
    fn execute_lui(&mut self, rd: u64, imm20: u64) -> Result<(), Exception> {
        self.stats.instr.instr_load += 1;
        self.xreg.write(rd, imm20);
        Ok(())
    }

    #[inline]
    fn execute_auipc(&mut self, rd: u64, imm20: u64) -> Result<(), Exception> {
        self.stats.instr.instr_arith += 1;
        self.xreg.write(rd, self.pc.wrapping_add(imm20));
        Ok(())
    }

    #[inline]
    fn execute_system(
        &mut self,
        rd: u64,
        funct3: u64,
        rs1: u64,
        instr: u64,
    ) -> Result<(), Exception> {
        let funct12 = (instr & 0xFFF00000) >> 20;
        match (rd, funct3, rs1) {
            (0, 0, 0) => {
                match funct12 {
                    0x000 => {
                        // ecall
                        match self.mode {
                            Mode::Machine => Err(Exception::EnvironmentCallFromMMode),
                        }
                    }
                    0x001 => {
                        // ebreak
                        Err(Exception::Breakpoint)
                    }
                    _ => Err(Exception::IllegalInstruction(instr)),
                }
            }
            _ => Err(Exception::IllegalInstruction(instr)),
        }
    }
}
