use std::io::{self, Write};
/// prompt user, retrieve input from command line & return it
/// 
pub fn get_input() -> io::Result<String> {
  let mut input = String::new();
  io::stdin().read_line(&mut input)?;
  Ok(input.trim().to_string())
}
pub fn prompt_user(prompt: &str) {
  print!("{}", prompt.to_string());
  io::stdout().flush().unwrap();
}