use core::panic;

fn main() {
  #[allow(unused_variables, dead_code)]
  struct CPU {
    registers: [u8; 16],
    memory: [u8; 4096],
    position_in_memory: usize,
    stack: [u16; 16],
    stack_pointer: usize,
  }

  impl CPU {
    fn call(&mut self, addr: u16) {
      let sp = self.stack_pointer;
      let stack = &mut self.stack;

      if sp > stack.len() {
        panic!("stack overflow!")
      }

      stack[sp] = self.position_in_memory as u16;
      self.stack_pointer += 1;
      self.position_in_memory = addr as usize;
    }

    fn ret(&mut self) {
      if self.stack_pointer == 0 {
        panic!("Stack underflow");
      }

      self.stack_pointer -= 1;
      let call_addr = self.stack[self.stack_pointer];
      self.position_in_memory = call_addr as usize;
    }
  }

  let mut memory: [u8; 4096] = [0; 4096];
  let mem = &mut memory;

  let add_twice = [
    0x80, 0x14,
    0x80, 0x14,
    0x00, 0xEE,
  ];

  mem[0x100..0x106].copy_from_slice(&add_twice);

  println!("{:?}", &mem[0x100..0x106]);
}

fn main() {
}
