use std::fmt::Debug;
use num_traits::PrimInt;

#[derive(Debug, thiserror::Error)]
pub enum MemoryError {
    #[error("Memory address out of bounds")]
    OutOfBounds(),
}

pub struct Memory<T> {
    mem: Vec<T>,
}

impl <T: Copy + Default + PrimInt + Debug> Memory <T> {
    pub fn new(size: u32) -> Self {
        return Self {
            mem: vec![T::default(); size as usize],
        };
    }

    pub fn size(&self) -> u32 {
        return self.mem.len() as u32;
    }

    pub fn set(&mut self, cell: u32, value: T) -> Result<(), MemoryError> {
        if cell > self.mem.len() as u32 - 1 {
            return Err(MemoryError::OutOfBounds());
        }
        self.mem[cell as usize] = value;
        Ok(())
    }

    pub fn get(&self, cell: u32) -> Result<T, MemoryError> {
        if cell > self.mem.len() as u32 - 1 {
            return Err(MemoryError::OutOfBounds());
        }
        Ok(self.mem[cell as usize])
    }
}
