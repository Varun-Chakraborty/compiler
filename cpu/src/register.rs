#[derive(Debug, thiserror::Error)]
pub enum RegisterError {
    #[error("Invalid register {0}")]
    InvalidRegister(u32),
}

pub struct Register {
    count: u32,
    regs: Vec<i8>,
}

impl Register {
    pub fn new(count: u32) -> Self {
        return Self {
            count,
            regs: vec![0; count as usize],
        };
    }

    pub fn set(&mut self, register: u32, value: i8) -> Result<(), RegisterError> {
        if register > self.count - 1 {
            return Err(RegisterError::InvalidRegister(register));
        }
        self.regs[register as usize] = value;
        Ok(())
    }

    pub fn get(&self, register: u32) -> Result<i8, RegisterError> {
        if register > self.count - 1 {
            return Err(RegisterError::InvalidRegister(register));
        }
        Ok(self.regs[register as usize])
    }
}
