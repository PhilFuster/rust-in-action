use std::fs::File;
use std::io::prelude::*;
use std::env;

const BYTES_PER_LINE: usize = 16;

#[allow(dead_code)]
fn main() {
  let arg1 = env::args().nth(1);

  let fname = arg1.expect("usage: fview FILENAME");

  let mut f = File::open(&fname).expect("Unable to open file.");
  const BYTES_PER_LINE: usize = 16;
  const INPUT: &'static [u8] = br#"
  fn main() {
    println!("Hello, world!");
  }"#;

  fn main() -> std::io::Result<()> {
    let mut buffer: Vec<u8> = vec!();
    INPUT.read_to_end(&mut buffer)?;
    let mut position_in_input = 0;
    for line in buffer.chunks(BYTES_PER_LINE){
      print!("[0x{:08x}] ", position_in_input);
      for byte in line {
        print!("{:02x} ", byte);
      }
      println!();
      position_in_input += BYTES_PER_LINE;
    }
    Ok(())
  }
  main();
}