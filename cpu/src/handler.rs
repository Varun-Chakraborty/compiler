use crate::{CPUError, MemoryAccess, MyCPU};
use std::io::{Write, stdin, stdout};

pub struct Delta {
    pub registers: Vec<String>,
    pub flags: Vec<String>,
    pub memory_access: Option<MemoryAccess>,
}

impl MyCPU {
    pub fn halt(&mut self, _: &[u32]) -> Result<Delta, CPUError> {
        self.program_counter = self.eof;
        Ok(Delta {
            registers: vec![],
            flags: vec![],
            memory_access: None,
        })
    }

    pub fn input(&mut self, operands: &[u32]) -> Result<Delta, CPUError> {
        let register = operands[0];
        let mut input = String::new();
        print!("Enter value for register {register}: ");
        stdout().flush()?;
        stdin().read_line(&mut input)?;
        let input = input.trim().parse::<i8>()? as u8;
        self.register.set(register, input)?;
        Ok(Delta {
            registers: vec![format!("R{register}")],
            flags: vec![],
            memory_access: None,
        })
    }

    pub fn output(&mut self, operands: &[u32]) -> Result<Delta, CPUError> {
        let register = operands[0];
        let value = self.register.get(register)? as i8;
        println!("Output from register {register}: {value}");
        stdout().flush()?;
        Ok(Delta {
            registers: vec![],
            flags: vec![],
            memory_access: None,
        })
    }

    pub fn output_16(&mut self, _: &[u32]) -> Result<Delta, CPUError> {
        let high_byte = self.register.get(1)? as u16;
        let low_byte = self.register.get(0)? as u16;
        let value = ((high_byte << 8) | low_byte) as i16;
        println!("Combined output from registers 0 and 1: {value}");
        stdout().flush()?;
        Ok(Delta {
            registers: vec![],
            flags: vec![],
            memory_access: None,
        })
    }

    pub fn output_char(&mut self, operands: &[u32]) -> Result<Delta, CPUError> {
        let register = operands[0];
        let value = self.register.get(register)? as i8;
        print!("{}", value as u8 as char);
        stdout().flush()?;
        Ok(Delta {
            registers: vec![],
            flags: vec![],
            memory_access: None,
        })
    }

    pub fn mover(&mut self, operands: &[u32], immediate: bool) -> Result<Delta, CPUError> {
        let register = operands[0];
        let value = if immediate {
            operands[1] as u8
        } else {
            self.data_memory.get(operands[1])?
        };
        self.register.set(register, value)?;
        Ok(Delta {
            registers: vec![format!("R{register}")],
            flags: vec![],
            memory_access: if immediate {
                None
            } else {
                Some(MemoryAccess {
                    address: operands[1],
                    value: value,
                    type_: crate::Type::Read,
                })
            },
        })
    }

    pub fn movem(&mut self, operands: &[u32]) -> Result<Delta, CPUError> {
        let register = operands[0];
        let memory = operands[1];
        let value = self.register.get(register)?;
        self.data_memory.set(memory, value)?;
        Ok(Delta {
            registers: vec![],
            flags: vec![],
            memory_access: Some(MemoryAccess {
                address: memory,
                value: value,
                type_: crate::Type::Write,
            }),
        })
    }

    pub fn add(&mut self, operands: &[u32], immediate: bool) -> Result<Delta, CPUError> {
        let dest = operands[0];
        let num1 = self.register.get(operands[1])?;
        let num2 = if immediate {
            operands[2] as u8
        } else {
            self.register.get(operands[2])?
        };
        let sum_16 = num1 as u16 + num2 as u16;
        let sum_8 = sum_16 as i8;
        self.flags.zero = sum_8 == 0;
        self.flags.sign = sum_8 < 0;
        self.flags.carry = sum_16 > 255;
        self.flags.overflow = ((num1 ^ sum_8 as u8) & (num2 ^ sum_8 as u8)) & (1 << 7) != 0;
        self.register.set(dest, sum_8 as u8)?;
        Ok(Delta {
            registers: vec![],
            flags: vec![],
            memory_access: None,
        })
    }

    pub fn adc(&mut self, operands: &[u32], immediate: bool) -> Result<Delta, CPUError> {
        let dest = operands[0];
        let num1 = self.register.get(operands[1])?;
        let num2 = if immediate {
            operands[2] as u8
        } else {
            self.register.get(operands[2])?
        };
        let sum_16 = num1 as u16 + num2 as u16 + self.flags.carry as u16;
        let sum_8 = sum_16 as i8;
        self.flags.zero = sum_8 == 0;
        self.flags.sign = sum_8 < 0;
        self.flags.carry = sum_16 > 255;
        self.flags.overflow = ((num1 ^ sum_8 as u8) & (num2 ^ sum_8 as u8)) & (1 << 7) != 0;
        self.register.set(dest, sum_8 as u8)?;
        Ok(Delta {
            registers: vec![],
            flags: vec![],
            memory_access: None,
        })
    }

    pub fn sub(&mut self, operands: &[u32], immediate: bool) -> Result<Delta, CPUError> {
        let dest = operands[0];
        let num1 = self.register.get(operands[1])?;
        let num2 = if immediate {
            operands[2] as u8
        } else {
            self.register.get(operands[2])?
        };
        let diff_16 = num1 as u16 + 256 - num2 as u16;
        let diff_8 = diff_16 as i8;
        self.flags.zero = diff_8 == 0;
        self.flags.sign = diff_8 < 0;
        self.flags.carry = num1 < num2;
        self.flags.overflow = ((num1 ^ num2) & (num1 ^ diff_8 as u8)) & (1 << 7) != 0;
        self.register.set(dest, diff_8 as u8)?;
        Ok(Delta {
            registers: vec![],
            flags: vec![],
            memory_access: None,
        })
    }

    pub fn sbc(&mut self, operands: &[u32], immediate: bool) -> Result<Delta, CPUError> {
        let dest = operands[0];
        let num1 = self.register.get(operands[1])?;
        let num2 = if immediate {
            operands[2] as u8
        } else {
            self.register.get(operands[2])?
        };
        let diff_16 = num1 as u16 + 256 - num2 as u16 - self.flags.carry as u16;
        let diff_8 = diff_16 as i8;
        self.flags.zero = diff_8 == 0;
        self.flags.sign = diff_8 < 0;
        self.flags.carry = num1 < (num2 + self.flags.carry as u8);
        self.flags.overflow = ((num1 ^ num2) & (num1 ^ diff_8 as u8)) & (1 << 7) != 0;
        self.register.set(dest, diff_8 as u8)?;
        Ok(Delta {
            registers: vec![],
            flags: vec![],
            memory_access: None,
        })
    }

    pub fn mult(&mut self, operands: &[u32], immediate: bool) -> Result<Delta, CPUError> {
        let dest = operands[0];
        let num1 = self.register.get(operands[1])? as i8 as i16;
        let num2 = if immediate {
            operands[2] as i8 as i16
        } else {
            self.register.get(operands[2])? as i8 as i16
        };
        let product = num1 * num2;

        let lowbyte = product as u8;
        let highbyte = (product >> 8) as u8;

        self.register.set(dest, lowbyte)?;
        self.register.set(dest + 1, highbyte)?;

        self.flags.zero = product == 0;
        self.flags.sign = product < 0;
        self.flags.overflow = highbyte != 0;
        self.flags.carry = highbyte != 0;
        Ok(Delta {
            registers: vec![],
            flags: vec![],
            memory_access: None,
        })
    }

    pub fn mult_16(&mut self, operands: &[u32], immediate: bool) -> Result<Delta, CPUError> {
        let num1 = if immediate {
            operands[0] as i8 as i16
        } else {
            self.register.get(operands[0])? as i8 as i16
        };
        let num2 =
            (((self.register.get(1)? as i8 as u16) << 8) | self.register.get(0)? as u16) as i16;
        let product = num1 * num2;
        let highbyte = (product >> 8) as u8;
        let lowbyte = product as u8;

        self.register.set(0, lowbyte)?;
        self.register.set(1, highbyte)?;

        self.flags.zero = product == 0;
        self.flags.sign = product < 0;
        self.flags.overflow = highbyte != 0;
        self.flags.carry = highbyte != 0;
        Ok(Delta {
            registers: vec![],
            flags: vec![],
            memory_access: None,
        })
    }

    pub fn cmp(&mut self, operands: &[u32], immediate: bool) -> Result<Delta, CPUError> {
        let num1 = self.register.get(operands[0])? as i8;
        let num2 = if immediate {
            operands[1] as i8
        } else {
            self.register.get(operands[1])? as i8
        };
        let (diff, carry) = num1.overflowing_sub(num2);
        self.flags.sign = diff < 0;
        self.flags.zero = diff == 0;
        self.flags.carry = carry;
        self.flags.overflow = ((num1 ^ num2) & (num1 ^ diff)) & (1 << 7) != 0;
        Ok(Delta {
            registers: vec![],
            flags: vec![],
            memory_access: None,
        })
    }

    pub fn and(&mut self, operands: &[u32]) -> Result<Delta, CPUError> {
        let dest = operands[0];
        let num1 = self.register.get(operands[1])?;
        let num2 = self.register.get(operands[2])?;
        let product = num1 & num2;
        self.flags.zero = product == 0;
        self.flags.sign = (product & (1 << 7)) != 0;
        self.flags.overflow = false;
        self.flags.carry = false;
        self.register.set(dest, product)?;
        Ok(Delta {
            registers: vec![],
            flags: vec![],
            memory_access: None,
        })
    }

    pub fn or(&mut self, operands: &[u32]) -> Result<Delta, CPUError> {
        let dest = operands[0];
        let num1 = self.register.get(operands[1])?;
        let num2 = self.register.get(operands[2])?;
        let product = num1 | num2;
        self.flags.zero = product == 0;
        self.flags.sign = (product & (1 << 7)) != 0;
        self.flags.overflow = false;
        self.flags.carry = false;
        self.register.set(dest, product)?;
        Ok(Delta {
            registers: vec![],
            flags: vec![],
            memory_access: None,
        })
    }

    pub fn xor(&mut self, operands: &[u32]) -> Result<Delta, CPUError> {
        let dest = operands[0];
        let num1 = self.register.get(operands[1])?;
        let num2 = self.register.get(operands[2])?;
        let product = num1 ^ num2;
        self.flags.zero = product == 0;
        self.flags.sign = (product & (1 << 7)) != 0;
        self.flags.overflow = false;
        self.flags.carry = false;
        self.register.set(dest, product)?;
        Ok(Delta {
            registers: vec![],
            flags: vec![],
            memory_access: None,
        })
    }

    pub fn not(&mut self, operands: &[u32]) -> Result<Delta, CPUError> {
        let dest = operands[0];
        let num1 = self.register.get(operands[1])?;
        let product = !num1;
        self.flags.zero = product == 0;
        self.flags.sign = (product & (1 << 7)) != 0;
        self.flags.overflow = false;
        self.flags.carry = false;
        self.register.set(dest, product)?;
        Ok(Delta {
            registers: vec![],
            flags: vec![],
            memory_access: None,
        })
    }

    pub fn shl(&mut self, operands: &[u32]) -> Result<Delta, CPUError> {
        let reg = operands[0];
        let value = self.register.get(reg)?;
        self.flags.carry = (value & (1 << 7)) != 0;
        let shifted_value = value << 1;
        self.flags.zero = shifted_value == 0;
        self.flags.sign = (shifted_value & (1 << 7)) != 0;
        self.flags.overflow = ((shifted_value ^ value) & (1 << 7)) != 0;
        self.register.set(operands[0], shifted_value)?;
        Ok(Delta {
            registers: vec![],
            flags: vec![],
            memory_access: None,
        })
    }

    pub fn shr(&mut self, operands: &[u32]) -> Result<Delta, CPUError> {
        let reg = operands[0];
        let value = self.register.get(reg)? as i8;
        self.flags.carry = (value & 1) != 0;
        let value = value >> 1;
        self.flags.zero = value == 0;
        self.flags.sign = (value & (1 << 7)) != 0;
        self.flags.overflow = false;
        self.register.set(operands[0], value as u8)?;
        Ok(Delta {
            registers: vec![],
            flags: vec![],
            memory_access: None,
        })
    }

    pub fn push(&mut self, operands: &[u32]) -> Result<Delta, CPUError> {
        let reg = operands[0];
        let value = self.register.get(reg)?;
        self.stack_pointer -= 1;
        self.data_memory.set(self.stack_pointer, value)?;
        Ok(Delta {
            registers: vec![],
            flags: vec![],
            memory_access: None,
        })
    }

    pub fn pop(&mut self, operands: &[u32]) -> Result<Delta, CPUError> {
        let reg = operands[0];
        let value = self.data_memory.get(self.stack_pointer)?;
        self.stack_pointer += 1;
        self.register.set(reg, value)?;
        Ok(Delta {
            registers: vec![],
            flags: vec![],
            memory_access: None,
        })
    }

    pub fn call(&mut self, operands: &[u32]) -> Result<Delta, CPUError> {
        self.stack_pointer -= 1;
        self.data_memory
            .set(self.stack_pointer, self.program_counter as u8)?;
        // self.stack_pointer -= 1;
        // self.data_memory
        //     .set(self.stack_pointer, (self.program_counter >> 8) as u8)?;
        // self.stack_pointer -= 1;
        // self.data_memory
        //     .set(self.stack_pointer, (self.program_counter >> 16) as u8)?;
        // self.stack_pointer -= 1;
        // self.data_memory
        //     .set(self.stack_pointer, (self.program_counter >> 24) as u8)?;
        self.program_counter = operands[0];
        Ok(Delta {
            registers: vec![],
            flags: vec![],
            memory_access: None,
        })
    }

    pub fn ret(&mut self, _: &[u32]) -> Result<Delta, CPUError> {
        let mut location: u32 = 0;
        location |= self.data_memory.get(self.stack_pointer)? as u32;
        self.stack_pointer += 1;
        // location |= (self.data_memory.get(self.stack_pointer)? as u32) << 8;
        // self.stack_pointer += 1;
        // location |= (self.data_memory.get(self.stack_pointer)? as u32) << 16;
        // self.stack_pointer += 1;
        // location |= (self.data_memory.get(self.stack_pointer)? as u32) << 24;
        // self.stack_pointer += 1;
        self.program_counter = location;
        Ok(Delta {
            registers: vec![],
            flags: vec![],
            memory_access: None,
        })
    }

    pub fn jmp(&mut self, operands: &[u32]) -> Result<Delta, CPUError> {
        let address = operands[0];
        self.program_counter = address;
        Ok(Delta {
            registers: vec![],
            flags: vec![],
            memory_access: None,
        })
    }

    pub fn jz(&mut self, operands: &[u32]) -> Result<Delta, CPUError> {
        let address = operands[0];
        if self.flags.zero {
            self.program_counter = address;
        };
        Ok(Delta {
            registers: vec![],
            flags: vec![],
            memory_access: None,
        })
    }

    pub fn jnz(&mut self, operands: &[u32]) -> Result<Delta, CPUError> {
        let address = operands[0];
        if !self.flags.zero {
            self.program_counter = address;
        };
        Ok(Delta {
            registers: vec![],
            flags: vec![],
            memory_access: None,
        })
    }

    pub fn jge(&mut self, operands: &[u32]) -> Result<Delta, CPUError> {
        let address = operands[0];
        if !self.flags.zero && !self.flags.sign && self.flags.carry {
            self.program_counter = address;
        };
        Ok(Delta {
            registers: vec![],
            flags: vec![],
            memory_access: None,
        })
    }

    pub fn jl(&mut self, _operands: &[u32]) -> Result<Delta, CPUError> {
        Ok(Delta {
            registers: vec![],
            flags: vec![],
            memory_access: None,
        })
    }
}
