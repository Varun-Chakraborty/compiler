pub struct Register {
    count: u32,
    regs: Vec<u8>
}

impl Register {
    pub fn new(count: u32) -> Self {
        if count < 1 || count > 4 {
            panic!("Register count must be between 1 and 4");
        }
        return Self {
            count,
            regs: vec![0; count as usize]
        }
    }

    pub fn set(&mut self, register: u32, value: u8) {
        if register > self.count - 1 {
            panic!("Invalid register");
        }
        self.regs[register as usize] = value;
    }

    pub fn get(&self, register: u32) -> u8 {
        if register > self.count - 1 {
            panic!("Invalid register");
        }
        return self.regs[register as usize];
    }
}