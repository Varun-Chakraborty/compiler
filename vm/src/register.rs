use std::fmt::Debug;

use num_traits::PrimInt;

#[derive(Debug, thiserror::Error)]
pub enum RegisterError {
    #[error("Invalid register {0}")]
    InvalidRegister(u32),
}

#[derive(Debug, Clone)]
pub struct Register<T> {
    pub count: u32,
    pub regs: Vec<T>,
}

impl<T: Copy + Default + PrimInt + Debug> Register<T> {
    pub fn new(count: u32) -> Self {
        return Self {
            count,
            regs: vec![T::default(); count as usize],
        };
    }

    pub fn set(&mut self, register: u32, value: T) -> Result<(), RegisterError> {
        if register > self.count - 1 {
            return Err(RegisterError::InvalidRegister(register));
        }
        self.regs[register as usize] = value;
        Ok(())
    }

    pub fn get(&self, register: u32) -> Result<T, RegisterError> {
        if register > self.count - 1 {
            return Err(RegisterError::InvalidRegister(register));
        }
        Ok(self.regs[register as usize])
    }
}
