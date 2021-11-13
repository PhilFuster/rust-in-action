#[allow(unused_variables)]
fn main() {
  struct CPU {
    current_operation: u16,
    registers: [u8; 2],
  }

  impl CPU {
    fn read_opcode(&self) -> u16 {
      self.current_operation
    }
    fn run(&mut self) {
      let opcode = self.read_opcode();
      // taking each part of the operation passed.
      // operation: 0x8014
      // isolates the 8 - by masking the 4 bits we want (the 8) then shifting 12 bits rigth (to get the least significant bit position)
      let c = ((opcode & 0xF000) >> 12) as u8;
      // isolates the 0 - by using AND masking with the 2nd set of 4 bits (the 0) and shifting those 4 bits to the least signifant bit position
      let x = ((opcode & 0x0F00) >> 8) as u8;
      // isolates the 1 - by using AND mask with the 3rd set of 4 bits (the 1) and shifting those bits 4 bits to the right to the least significant bit position
      let y = ((opcode & 0x00F0) >> 4) as u8;
      // isolates the 4 - by using AND mask with the 4th set of 4 bits (the 4) and not shifting those bits b/c already at least significant bit position
      let d = ((opcode & 0x000F) >> 0) as u8;
      println!("c: {}", c);
      println!("x: {}", x);
      println!("y: {}", y);
      println!("d: {}", d);

      match (c, x, y, d) {
        (0x8, _, _, 0x4) => self.add_xy(x, y),
        _ => todo!("opcode {:04x}", opcode),
      }
    }

    fn add_xy(&mut self, x: u8, y: u8) {
      println!("adding {} + {}", self.registers[x as usize], self.registers[y as usize]);
      self.registers[x as usize] += self.registers[y as usize];
      println!("result: {}", self.registers[0]);
    }


  }

  let mut cpu = CPU {
    current_operation: 0, // initializes with no op
    registers: [0; 2],   
  };

  cpu.current_operation = 0x8014;
  cpu.registers[0] = 5;
  cpu.registers[1] = 10;
  cpu.run();

  assert_eq!(cpu.registers[0], 15);

  println!("5 + 10 = {}", cpu.registers[0]);
}
