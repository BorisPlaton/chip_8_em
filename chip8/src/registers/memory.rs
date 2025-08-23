use crate::memory::Memory;

#[derive(Default)]
pub struct MemoryRegister {
    value: u16,
}

impl MemoryRegister {
    pub fn set(&mut self, value: u16) {
        self.value = value & Memory::MEMORY_SIZE;
    }

    pub fn get(&self) -> u16 {
        self.value
    }

    pub fn add(&self, value: u16) -> u16 {
        (self.value.wrapping_add(value)) & Memory::MEMORY_SIZE
    }
}
