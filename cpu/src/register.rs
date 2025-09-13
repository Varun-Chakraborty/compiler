use std::error::Error;

pub struct Register {
    count: u32,
    regs: Vec<u8>
}

impl Register {
    pub fn new(count: u32) -> Self {
        return Self {
            count,
            regs: vec![0; count as usize]
        };
    }

    pub fn set(&mut self, register: u32, value: u8) -> Result<(), Box<dyn std::error::Error>> {
        if register > self.count - 1 {
            return Err("Invalid register".into());
        }
        self.regs[register as usize] = value;
        return Ok(());
    }

    pub fn get(&self, register: u32) -> Result<u8, Box<dyn Error>> {
        if register > self.count - 1 {
            return Err("Invalid register {register}".into());
        }
        return Ok(self.regs[register as usize]);
    }
}