pub struct MemoryRegister {
    value: u16,
    memory_limit: u16,
}

impl MemoryRegister {
    pub fn new(memory_limit: u16) -> Self {
        MemoryRegister {
            value: 0,
            memory_limit,
        }
    }

    pub fn set(&mut self, value: u16) {
        self.value = value & self.memory_limit;
    }

    pub fn get(&self) -> u16 {
        self.value
    }

    pub fn add(&self, value: u16) -> u16 {
        (self.value.wrapping_add(value)) & self.memory_limit
    }
}
