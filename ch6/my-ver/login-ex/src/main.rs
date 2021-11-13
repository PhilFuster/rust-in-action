use std::{ io::{self, Write}};

const USERNAMES:[&'static str; 3] = ["Billy", "Jane", "Phil"];
const PASSWORDS:[&'static str; 3] = ["123", "test", "philloveskat"];

#[allow(unused_variables, unused_assignments)]
fn main() {
  /// prompt user and retrieve user input from command line 
  /// 
  /// pass in a prompt that can be converted to a string and get the
  /// users input from command line
  fn get_input<T: Into<String>>(prompt: T) -> io::Result<String>{
    print!("{}", prompt.into());
    io::stdout().flush().unwrap();
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer)
  }
  println!("Welcome to Phil's site!");
  println!("Please login to continue!");

  let res = get_input("Username: ");
  let mut username = String::new();
  match res {
    Ok(input) => username = input.trim().to_string(),
    Err(e) => panic!("Couldn't get username! error: {}",e),
  }

  let res = get_input("Password: ");
  let mut pw = String::new();
  match res {
    Ok(input) => pw = input.trim().to_string(),
    Err(e) => panic!("Couldn't get password! error: {}", e),
  }
  let user_index = USERNAMES.iter().position(|&user| String::from(user) == username);
  match user_index {
    Some(index) => {
      if PASSWORDS[index] == pw {
        println!("Sucessfully signed in as {}", username);
      }
    },
    _ => {
      println!("Username/Password incorrect");
    }
  }
}
