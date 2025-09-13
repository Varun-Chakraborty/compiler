use std::error::Error;

pub struct Memory {
    mem: Vec<u8>
}

impl Memory {
    pub fn new(size: u32) -> Self {
        return Self {
            mem: vec![0; size as usize]
        }
    }

    pub fn size(&self) -> u32 {
        return self.mem.len() as u32;
    }

    pub fn set(&mut self, cell: u32, value: u8) -> Result<(), Box<dyn Error>> {
        if cell > self.mem.len() as u32 - 1 {
            return Err("Memory address out of bounds".into());
        }
        self.mem[cell as usize] = value;
        return Ok(());
    }

    pub fn get(&self, cell: u32) -> Result<u8, Box<dyn Error>> {
        if cell > self.mem.len() as u32 - 1 {
            return  Err("Memory address out of bounds".into());
        }
        return Ok(self.mem[cell as usize]);
    }
}