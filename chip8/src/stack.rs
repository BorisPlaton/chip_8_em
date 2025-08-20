use crate::chip::Chip8;

/// The stack is an array of 16 16-bit values, used to store the address
/// that the interpreter should return to when finished with a subroutine.
/// Chip-8 allows for up to 16 levels of nested subroutines.
pub struct Stack {
    /// It is used to point to the topmost level of the stack.
    stack_pointer: u8,
    stack: [u16; 16],
}

impl Stack {
    pub fn push(&mut self, val: u16) {
        if self.stack_pointer > 16 {
            panic!("Stack is full.");
        }
        self.stack[self.stack_pointer as usize] = val;
        self.stack_pointer += 1;
    }

    pub fn pull(&mut self) -> u16 {
        if self.stack_pointer == 0 {
            panic!("Can't pull because stack is empty.");
        }
        self.stack_pointer -= 1;
        self.stack[self.stack_pointer as usize] & Chip8::ADDRESS_MIRRORING
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
