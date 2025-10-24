use crate::cpu::{CPUError, MyCPU};
use std::io::{Write, stdin, stdout};

impl MyCPU {
    pub fn halt(&mut self, _: &[u32]) {
        self.program_counter = self.eof;
    }

    pub fn input(&mut self, operands: &[u32]) -> Result<(), CPUError> {
        let register = operands[0];
        let mut input = String::new();
        print!("Enter value for register {register}: ");
        stdout().flush()?;
        stdin().read_line(&mut input)?;
        let input = input.trim().parse()?;
        self.register.set(register, input)?;
        Ok(())
    }

    pub fn output(&mut self, operands: &[u32]) -> Result<(), CPUError> {
        let register = operands[0];
        let value = self.register.get(register)?;
        println!("Output from register {register}: {value}");
        stdout().flush()?;
        Ok(())
    }

    // move value from memory to register
    pub fn mover(&mut self, operands: &[u32], immediate: bool) -> Result<(), CPUError> {
        let register = operands[0];
        let value = if immediate {
            operands[1] as i8
        } else {
            self.data_memory.get(operands[1])?
        };
        self.register.set(register, value)?;
        Ok(())
    }

    // move from register to memory
    pub fn movem(&mut self, operands: &[u32]) -> Result<(), CPUError> {
        let register = operands[0];
        let memory = operands[1];
        let value = self.register.get(register)?;
        self.data_memory.set(memory, value)?;
        Ok(())
    }

    // move immediate value to memory
    pub fn movemi(&mut self, operands: &[u32]) -> Result<(), CPUError> {
        let memory = operands[0];
        let value = operands[1];
        self.data_memory.set(memory, value as i8)?;
        Ok(())
    }

    pub fn add(&mut self, operands: &[u32], immediate: bool) -> Result<(), CPUError> {
        let dest = operands[0];
        let num1 = self.register.get(operands[1])?;
        let num2 = if immediate {
            operands[2] as i8
        } else {
            self.data_memory.get(operands[2])?
        };
        let (sum, carry) = num1.overflowing_add(num2);
        self.flags.sign = (sum & (1 << 7)) != 0;
        self.flags.zero = sum == 0;
        self.flags.carry = carry;
        self.flags.overflow = (num1 ^ sum) & (num2 ^ sum) & (1 << 7) != 0;
        self.register.set(dest, sum)?;
        Ok(())
    }

    pub fn sub(&mut self, operands: &[u32], immediate: bool) -> Result<(), CPUError> {
        let dest = operands[0];
        let num1 = self.register.get(operands[1])?;
        let num2 = if immediate {
            operands[2] as i8
        } else {
            self.data_memory.get(operands[2])?
        };
        let (diff, carry) = num1.overflowing_sub(num2);
        self.flags.sign = (diff & (1 << 7)) != 0;
        self.flags.zero = diff == 0;
        self.flags.carry = carry;
        self.flags.overflow = ((num1 ^ num2) & (num1 ^ diff)) & (1 << 7) != 0;
        self.register.set(dest, diff)?;
        Ok(())
    }

    pub fn mult(&mut self, operands: &[u32], immediate: bool) -> Result<(), CPUError> {
        let dest = operands[0];
        let num1 = self.register.get(operands[1])?;
        let num2 = if immediate {
            operands[2] as i8
        } else {
            self.data_memory.get(operands[2])?
        };
        let (product, carry) = num1.overflowing_mul(num2);
        self.flags.sign = (product & (1 << 7)) != 0;
        self.flags.zero = product == 0;
        self.flags.carry = carry;
        self.flags.overflow = carry;
        self.register.set(dest, product)?;
        Ok(())
    }

    pub fn cmp(&mut self, operands: &[u32], immediate: bool) -> Result<(), CPUError> {
        let num1 = self.register.get(operands[0])?;
        let num2 = if immediate {
            operands[2] as i8
        } else {
            self.data_memory.get(operands[1])?
        };
        let (diff, carry) = num1.overflowing_sub(num2);
        self.flags.sign = (diff & (1 << 7)) != 0;
        self.flags.zero = diff == 0;
        self.flags.carry = carry;
        self.flags.overflow = ((num1 ^ num2) & (num1 ^ diff)) & (1 << 7) != 0;
        Ok(())
    }

    pub fn jmp(&mut self, operands: &[u32]) {
        let address = operands[0];
        self.program_counter = address;
    }

    pub fn jz(&mut self, operands: &[u32]) {
        let address = operands[0];
        if self.flags.zero {
            self.program_counter = address;
        }
    }

    pub fn jnz(&mut self, operands: &[u32]) {
        let address = operands[0];
        if !self.flags.zero {
            self.program_counter = address;
        }
    }

    pub fn and(&mut self, operands: &[u32]) -> Result<(), CPUError> {
        let dest = operands[0];
        let num1 = self.register.get(operands[1])?;
        let num2 = self.data_memory.get(operands[2])?;
        let product = num1 & num2;
        self.flags.zero = product == 0;
        self.flags.sign = (product & (1 << 7)) != 0;
        self.flags.overflow = false;
        self.flags.carry = false;
        self.register.set(dest, product)?;
        Ok(())
    }

    pub fn or(&mut self, operands: &[u32]) -> Result<(), CPUError> {
        let dest = operands[0];
        let num1 = self.register.get(operands[1])?;
        let num2 = self.data_memory.get(operands[2])?;
        let product = num1 | num2;
        self.flags.zero = product == 0;
        self.flags.sign = (product & (1 << 7)) != 0;
        self.flags.overflow = false;
        self.flags.carry = false;
        self.register.set(dest, product)?;
        Ok(())
    }

    pub fn xor(&mut self, operands: &[u32]) -> Result<(), CPUError> {
        let dest = operands[0];
        let num1 = self.register.get(operands[1])?;
        let num2 = self.data_memory.get(operands[2])?;
        let product = num1 ^ num2;
        self.flags.zero = product == 0;
        self.flags.sign = (product & (1 << 7)) != 0;
        self.flags.overflow = false;
        self.flags.carry = false;
        self.register.set(dest, product)?;
        Ok(())
    }

    pub fn not(&mut self, operands: &[u32]) -> Result<(), CPUError> {
        let dest = operands[0];
        let num1 = self.register.get(operands[1])?;
        let product = !num1;
        self.flags.zero = product == 0;
        self.flags.sign = (product & (1 << 7)) != 0;
        self.flags.overflow = false;
        self.flags.carry = false;
        self.register.set(dest, product)?;
        Ok(())
    }

    pub fn shl(&mut self, operands: &[u32]) -> Result<(), CPUError> {
        let reg = operands[0];
        let value = self.register.get(reg)?;
        self.flags.carry = (value & (1 << 7)) != 0;
        let value = value << 1;
        self.flags.zero = value == 0;
        self.flags.sign = value < 0;
        self.flags.overflow = false;
        self.register.set(operands[0], value)?;
        Ok(())
    }

    pub fn shr(&mut self, operands: &[u32]) -> Result<(), CPUError> {
        let reg = operands[0];
        let value = self.register.get(reg)?;
        self.flags.carry = (value & 1) != 0;
        let value = value >> 1;
        self.flags.zero = value == 0;
        self.flags.sign = (value & (1 << 7)) != 0;
        self.flags.overflow = false;
        self.register.set(operands[0], value)?;
        Ok(())
    }
}
