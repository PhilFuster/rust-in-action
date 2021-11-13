use std::io;
/// prompt user, retrieve input from command line & return it
/// 
pub fn get_input() -> io::Result<String> {
  let mut input = String::new();
  io::stdin().read_line(&mut input)?;
  Ok(input.trim().to_string())
}