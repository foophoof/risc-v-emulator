use instruction::{encoding, Instruction};
use cpu::CPU;

#[derive(Debug)]
pub struct Op {
    typ: OperationType,
    dest: u8,
    operand1: u8, // multiplicand/dividend
    operand2: u8, // multiplier/divisor
}

#[derive(Debug)]
pub enum OperationType {
    Mul,
    MulHighSigned,
    MulHighUnsigned,
    MulHighSignedUnsigned,
    Div,
    DivUnsigned,
    Remainder,
    RemainderUnsigned,
}

impl Op {
    pub fn parse(instruction: u32) -> Option<Op> {
        let decoded = encoding::R::parse(instruction);

        if decoded.opcode != 0x33 {
            // Not a OP opcode
            return None;
        }

        if decoded.funct7 != 1 {
            return None;
        }

        let typ = match decoded.funct3 {
            0b000 => OperationType::Mul,
            0b001 => OperationType::MulHighSigned,
            0b010 => OperationType::MulHighSignedUnsigned,
            0b011 => OperationType::MulHighUnsigned,
            0b100 => OperationType::Div,
            0b101 => OperationType::DivUnsigned,
            0b110 => OperationType::Remainder,
            0b111 => OperationType::RemainderUnsigned,
            _ => unreachable!(),
        };

        Some(Op {
            typ: typ,
            dest: decoded.rd,
            operand1: decoded.rs1,
            operand2: decoded.rs2,
        })
    }
}

impl Instruction for Op {
    fn execute(&self, cpu: &mut CPU) {
        let operand1 = cpu.get_register(self.operand1);
        let operand2 = cpu.get_register(self.operand2);

        let result = match self.typ {
            OperationType::Mul => ((operand1 as u64) * (operand2 as u64)) as u32,
            OperationType::MulHighSigned => (((operand1 as i32 as i64) * (operand2 as i32 as i64)) >> 32) as u32,
            OperationType::MulHighUnsigned => (((operand1 as u64) * (operand2 as u64)) >> 32) as u32,
            OperationType::MulHighSignedUnsigned => (((operand1 as i32 as i64) * (operand2 as i64)) >> 32) as u32,
            OperationType::Div => ((operand1 as i32) / (operand2 as i32)) as u32,
            OperationType::DivUnsigned => operand1 / operand2,
            OperationType::Remainder => ((operand1 as i32) % (operand2 as i32)) as u32,
            OperationType::RemainderUnsigned => operand1 % operand2,
        };

        cpu.set_register(self.dest, result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ram::RAM;
    use cpu::CPU;
    use instruction::Instruction;

    #[test]
    fn test_multiply() {
        let mut cpu = CPU::new(RAM::new(1024));

        let instr = Op::parse(0b0000001_00011_00010_000_00001_0110011).expect("couldn't parse MUL x0,x1,x2");

        macro_rules! test_mul {
            ($result:expr, $val1:expr, $val2:expr) => {
                cpu.set_register(2, $val1);
                cpu.set_register(3, $val2);
                instr.execute(&mut cpu);
                assert_eq!(cpu.get_register(1), $result);
            }
        }

        test_mul!(0x00001200, 0x00007e00, 0xb6db6db7);
        test_mul!(0x00001240, 0x00007fc0, 0xb6db6db7);

        test_mul!(0x00000000, 0x00000000, 0x00000000);
        test_mul!(0x00000001, 0x00000001, 0x00000001);
        test_mul!(0x00000015, 0x00000003, 0x00000007);

        test_mul!(0x00000000, 0x00000000, 0xffff8000);
        test_mul!(0x00000000, 0x80000000, 0x00000000);
        test_mul!(0x00000000, 0x80000000, 0xffff8000);

        test_mul!(0x0000ff7f, 0xaaaaaaab, 0x0002fe7d);
        test_mul!(0x0000ff7f, 0x0002fe7d, 0xaaaaaaab);

        test_mul!(0x00000000, 0xff000000, 0xff000000);

        test_mul!(0x00000001, 0xffffffff, 0xffffffff);
        test_mul!(0xffffffff, 0xffffffff, 0x00000001);
        test_mul!(0xffffffff, 0x00000001, 0xffffffff);
    }
}