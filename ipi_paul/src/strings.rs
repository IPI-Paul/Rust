extern crate clipboard;

use std::io::{Write, stdin, stdout};

use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;

pub fn swap_around_clip(args: Vec<String>) {
  use termion::color;
  let mut res = String::new();
  let mut command;
  let keep;
  let lose;

  if !args.is_empty() {
    command = args.join(" ");
  } else {
    command = String::new();
    println!(
      "\n\n{}When not in iteractive mode surround special characters {}|;&`\"'()\\#~<> {}with apostrophes '|' or quotes \"'\"{}.", 
      color::Fg(color::Yellow), color::Fg(color::Red), color::Fg(color::Yellow), color::Fg(color::Reset)
    );
    println!("{}In both interactive and non-interactive modes, separate entries with a space:", color::Fg(color::Yellow));
    print!("{}--swap{} character/string to swap on ({}required{}) ", color::Fg(color::Green), color::Fg(color::Reset), color::Fg(color::Red), color::Fg(color::Reset));
    print!("{}--unswapped{} characters/strings not to swap around ({}optional{}) ", color::Fg(color::Green), color::Fg(color::Reset), color::Fg(color::Red), color::Fg(color::Reset));
    print!("{}--remove{} characters/strings not wanted ({}optional{})\n> ", color::Fg(color::Green), color::Fg(color::Reset), color::Fg(color::Red), color::Fg(color::Reset));
    stdout().flush().unwrap();
    stdin().read_line(&mut command).unwrap();
    command = command.replace('\n', "");
  }

  if command.split("--remove").collect::<Vec<&str>>().len() > 1 {
    lose = String::from(command.split("--remove").collect::<Vec<&str>>()[1].trim().to_string());    
    command = command.split("--remove").collect::<Vec<&str>>()[0].trim().to_string();
  } else {
    lose = String::new();
  }
  if command.split("--unswapped").collect::<Vec<&str>>().len() > 1 {
    keep = String::from(command.split("--unswapped").collect::<Vec<&str>>()[1].trim().to_string());
    command = command.split("--unswapped").collect::<Vec<&str>>()[0].trim().to_string();
  } else {
    keep = String::new();
  }
  if command.split("--swap").collect::<Vec<&str>>().len() > 1 {
    let sep = command.split("--swap").collect::<Vec<&str>>()[1];
    let binding = keep.replace('\n', "");
    let keep = binding.split(" ").collect::<Vec<&str>>();
  
    let lost = lose.replace('\n', "");
    let lose = lost.split(" ").collect::<Vec<&str>>();
    
    let mut clip: ClipboardContext = ClipboardProvider::new().unwrap();
    for line in format!("{}", clip.get_contents().unwrap()).split('\n') {
      let mut new_line = String::from(line);
      let mut to_keep = Vec::<String>::new();
      for ch in line.split(" ") {
        if !ch.is_empty() {
          for item in &keep {
            if ch.contains(item) {
              to_keep.push(item.to_string());
            }
          }
        }
      }
      for del in lose.iter() {
        new_line = line.replace(del, "");
      }
      if new_line.contains(&sep) {
        if keep.iter().any(|x| new_line.contains(x)) {
          let mut keeping = Vec::new();
          for kept in keep.iter() {
            if new_line.contains(kept) {
              new_line = new_line.replace(kept, "");
              keeping.push(kept.to_string());
            }
          }
          res.push_str(
            &format!(
              "{} {} {} {}\n", 
              new_line.split(&sep).collect::<Vec<&str>>()[1].trim(), 
              &sep,
              to_keep.join(" "), 
              new_line.split(&sep).collect::<Vec<&str>>()[0].trim()
            )
          );
        } else {
          res.push_str(
            &format!(
              "{} {} {}\n", 
              new_line.split(&sep).collect::<Vec<&str>>()[1].trim(), 
              &sep,
              new_line.split(&sep).collect::<Vec<&str>>()[0].trim()
            )
          );
        }
      }
    }
    clip.set_contents(res).unwrap();
    std::thread::sleep(std::time::Duration::from_secs(10));
  }
}

pub fn toggle_case(txt: &str) {
  let mut res = String::new();
  for char in txt.chars() {
    if char.to_string() == char.to_uppercase().to_string() {
      res.push_str(&char.to_lowercase().to_string())
    } else {
      res.push_str(&char.to_uppercase().to_string())
    }
  }
  println!("{}", res);
}

pub fn toggle_case_clip() {
  let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
  println!("{:?}", ctx.get_contents());
  let mut txt = String::new();
  match ctx.get_contents() {
      Ok(content) => txt = content,
      Err(error) => println!("There was an error: {}", error),
  }
  let mut res = String::new();
  for char in txt.chars() {
    if char.to_string() == char.to_uppercase().to_string() {
      res.push_str(&char.to_lowercase().to_string())
    } else {
      res.push_str(&char.to_uppercase().to_string())
    }
  }
  println!("{}", &res);
  ctx.set_contents(res).unwrap();
  std::thread::sleep(std::time::Duration::from_secs(5));
}