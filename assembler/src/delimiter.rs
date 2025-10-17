#[derive(Clone, Debug)]
pub struct Delimiter {
    pub symbol: String,
    pub address: usize,
}

#[derive(Debug)]
pub struct DelimiterTable {
    table: Vec<Delimiter>,
    current: Option<Delimiter>,
    current_address: usize,
}

impl DelimiterTable {
    pub fn new() -> Self {
        Self {
            table: Vec::new(),
            current: None,
            current_address: 0,
        }
    }

    pub fn append(&mut self, symbol: String, address: usize) {
        self.table.push(Delimiter { symbol, address });
    }

    pub fn delete_last(&mut self) {
        self.table.pop();
    }

    pub fn next(&mut self) -> Option<usize> {
        if let Some(_) = self.current {
            self.current_address += 1;
        }
        self.current = self.table.get(self.current_address).cloned();
        None
    }

    pub fn get_current(&self) -> Option<Delimiter> {
        self.current.clone()
    }
}
