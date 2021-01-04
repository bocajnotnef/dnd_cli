use std::io::{Write, stdin, stdout};

pub fn prompt_and_read<T: std::str::FromStr>(prompt: &String) -> T {
  let mut input = String::new();
  print!("{}: ", prompt);
  let _ = stdout().flush();

  loop {
      stdin()
             .read_line(&mut input)
             .expect("Failed to read line");

      let _: T = match input.trim().parse() {
          Ok(value) => return value,
          Err(_) => continue,
      };
  }
}