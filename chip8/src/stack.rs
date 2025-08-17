// http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#2.2
pub struct Stack {
    /// It is used to point to the topmost level of the stack.
    stack_pointer: u8,
    stack: [u16; 16],
}

impl Stack {
    pub fn push(&mut self, val: u16) {
        todo!()
    }

    pub fn pull(&mut self) -> u16 {
        todo!()
    }
}

impl Default for Stack {
    fn default() -> Self {
        Stack {
            stack_pointer: 0,
            stack: [0; 16],
        }
    }
}
