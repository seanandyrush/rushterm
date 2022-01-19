//! # Rushterm
//! Make your CLI app easy by adding menu. Create nested menus, navigate with hotkeys. Data-driven. No function/macro complexity.
//! # Example
//! Firstly, we'll need to construct a `Menu` instance with its `Item`s. Bring them into scope. `Menu` instance doesn't need to be mutable. Next, we'll invoke `.run()` method on the instance to execute our menu:
//! ```rust
//! use rushterm::{Item, Menu};

//! fn main() {
//!   let menu = Menu {
//!     name: "My Main Menu".to_string(),
//!     items: vec![
//!       Item::Action {
//!         name: "Action0".to_string(),
//!         hotkey: Some('a'),
//!         exp: Some("Action0 Explanation. This Has Been Assigned To A Hotkey.".to_string()),
//!       },
//!       Item::Action {
//!         name: "Action1".to_string(),
//!         hotkey: None,
//!         exp: Some("Action1 Explanation. This Has No Hotkey.".to_string()),
//!       },
//!       Item::Input {
//!         name: "Input0".to_string(),
//!         hotkey: Some('i'),
//!         exp: Some("Input0 Explanation.".to_string()),
//!       },
//!       Item::SubMenu {
//!         name: "Submenu0".to_string(),
//!         hotkey: Some('s'),
//!         exp: Some("Submenu0 explanation.".to_string()),
//!         items: vec![
//!           Item::Action {
//!             name: "Sub Action0".to_string(),
//!             hotkey: Some('a'),
//!             exp: Some("Sub Action0 Explanation. This Has Been Assigned To A Hotkey.".to_string()),
//!           },
//!           Item::Action {
//!             name: "Sub Action1".to_string(),
//!             hotkey: Some('c'),
//!             exp: Some("Sub Action1 Explanation. This Has Been Assigned To A Hotkey.".to_string()),
//!           },
//!           Item::SubMenu {
//!             name: "Deepermenu0".to_string(),
//!             hotkey: Some('d'),
//!             exp: Some("Deepermenu0 Explanation.".to_string()),
//!             items: vec![
//!               Item::Action {
//!                 name: "Deeper Action0".to_string(),
//!                 hotkey: Some('f'),
//!                 exp: None,
//!               },
//!               Item::Action {
//!                 name: "Deeper Action1".to_string(),
//!                 hotkey: Some('g'),
//!                 exp: Some("Deeper Action1 Explanation.".to_string()),
//!               },
//!             ],
//!           },
//!         ],
//!       },
//!     ],
//!     exp: Some("My Main Menu Explanation.".to_string()),
//!     esc: true,
//!   };
//!   let selection = menu.run();
//!   dbg!(&selection);
//! }
//! ```
//! If selection is successful, `run()` method will return us `Selection` type in `Ok()` variant to get information we may need in ongoing execution. If not, exits the execution with an `Err()` variant.

use crossterm::{
  cursor,
  event::{read, Event, KeyCode, KeyEvent},
  style::Stylize,
  terminal::{self, ClearType},
  QueueableCommand,
};
use std::{
  io::{stdin, stdout, Stdout, Write},
  str::FromStr,
};
/// Anything that can be listed in `Menu`.
#[derive(Clone)]
pub enum Item {
  /// A menu item to execute an action. Exits `Menu`.
  Action {
    /// Action name.
    name: String,
    /// Assigning a hotkey to the item is optional. The hotkey is displayed in yellow.
    hotkey: Option<char>,
    /// Optional explanation in gray color is displayed next to the item.
    exp: Option<String>,
  },
  /// A menu item to enter branch menus. Eclipses `Menu` or another `SubMenu`.
  SubMenu {
    /// Sub menu name. It can be distinguished by the `+` character before it.
    name: String,
    /// Assigning a hotkey to the item is optional. The hotkey is displayed in yellow.
    hotkey: Option<char>,
    /// Optional explanation in gray color is displayed next to the item.
    exp: Option<String>,
    /// `SubMenu` items should be vector of `Item`s.
    items: Vec<Item>,
  },
  /// A menu item to input `bool`. It can be distinguished by the `=` character after it.
  Bool {
    /// Value name.
    name: String,
    /// Assigning a hotkey to the item is optional. The hotkey is displayed in yellow.
    hotkey: Option<char>,
    /// Optional explanation in gray color is displayed next to the item.
    exp: Option<String>,
  },
  /// A menu item to input `String`. It can be distinguished by the `=` character after it.
  Char {
    /// Value name.
    name: String,
    /// Assigning a hotkey to the item is optional. The hotkey is displayed in yellow.
    hotkey: Option<char>,
    /// Optional explanation in gray color is displayed next to the item.
    exp: Option<String>,
  },
  /// A menu item to input `String`. It can be distinguished by the `=` character after it.
  String {
    /// Value name.
    name: String,
    /// Assigning a hotkey to the item is optional. The hotkey is displayed in yellow.
    hotkey: Option<char>,
    /// Optional explanation in gray color is displayed next to the item.
    exp: Option<String>,
  },
  /// A menu item to input `f64`. It can be distinguished by the `=` character after it.
  F64 {
    /// Value name.
    name: String,
    /// Assigning a hotkey to the item is optional. The hotkey is displayed in yellow.
    hotkey: Option<char>,
    /// Optional explanation in gray color is displayed next to the item.
    exp: Option<String>,
  },
  /// A menu item to input `i64`. It can be distinguished by the `=` character after it.
  I64 {
    /// Value name.
    name: String,
    /// Assigning a hotkey to the item is optional. The hotkey is displayed in yellow.
    hotkey: Option<char>,
    /// Optional explanation in gray color is displayed next to the item.
    exp: Option<String>,
  },
  /// A menu item to input `u64`. It can be distinguished by the `=` character after it.
  U64 {
    /// Value name.
    name: String,
    /// Assigning a hotkey to the item is optional. The hotkey is displayed in yellow.
    hotkey: Option<char>,
    /// Optional explanation in gray color is displayed next to the item.
    exp: Option<String>,
  },
}
/// Starting point for creating a menu instance.
pub struct Menu {
  /// `Menu` name is displayed at the top.
  pub name: String,
  /// Optional explanation in gray color next to the menu name.
  pub exp: Option<String>,
  /// `Menu` items should be vector of `Item`s.
  pub items: Vec<Item>,
  /// Enable exiting menu by `Esc` hotkey. Usually set it to `true`. But it may be useful to set to `false` when you want to restrict the user from escaping without any selection.
  pub esc: bool,
}
/// Gives the data of the selection made in the menu.
#[derive(Debug, PartialEq)]
pub struct Selection {
  /// Name of selected `Item`.
  pub name: String,
  /// Vector containing direction of the selected item in the menu tree.
  pub path: Vec<String>,
  /// Input by user, if it exists.
  pub value: Option<Value>,
}
/// Input by user.
#[derive(Debug, PartialEq)]
pub enum Value {
  Bool(bool),
  Char(char),
  String(String),
  F64(f64),
  I64(i64),
  U64(u64),
}
impl Menu {
  /// Prints out `Item`s, executes the `Menu` and returns `Result`.
  pub fn run(&self) -> Result<Selection, String> {
    let mut stdout_ins = stdout();
    let mut hover = 0 as usize;
    self.printer(&mut stdout_ins, &mut hover)
  }
  fn printer(&self, stdout_ins: &mut Stdout, hover: &mut usize) -> Result<Selection, String> {
    self.print_top(&vec![self.name.to_string()]);
    self.print_items(hover);
    self.print_bottom(false);
    self.matcher(stdout_ins, hover)
  }
  fn matcher(&self, stdout_ins: &mut Stdout, hover: &mut usize) -> Result<Selection, String> {
    let keycode = self.poll_read();
    let key = self.match_keycode(keycode);
    let res = self.match_selection(
      &key,
      false,
      stdout_ins,
      &mut vec![self.name.to_string()],
      hover,
    );
    if res == Err("No Selection".to_string()) {
      self.matcher(stdout_ins, hover)
    } else {
      res
    }
  }
  fn run_sub(&self, path: &mut Vec<String>) -> Result<Selection, String> {
    let mut stdout_ins = stdout();
    let mut hover = 0 as usize;
    self.printer_sub(path, &mut stdout_ins, &mut hover)
  }
  fn printer_sub(
    &self,
    path: &mut Vec<String>,
    stdout_ins: &mut Stdout,
    hover: &mut usize,
  ) -> Result<Selection, String> {
    self.print_top(path);
    self.print_items(hover);
    self.print_bottom(true);
    self.matcher_sub(stdout_ins, path, hover)
  }
  fn matcher_sub(
    &self,
    stdout_ins: &mut Stdout,
    path: &mut Vec<String>,
    hover: &mut usize,
  ) -> Result<Selection, String> {
    let keycode = self.poll_read();
    let key = self.match_keycode(keycode);
    let res = self.match_selection(&key, true, stdout_ins, path, hover);
    if res == Err("No Selection".to_string()) {
      self.matcher_sub(stdout_ins, path, hover)
    } else {
      res
    }
  }
  fn print_top(&self, path: &Vec<String>) {
    for dir in path {
      print!("{}/", dir);
    }
    if let Some(exp) = &self.exp {
      print!(" {}", String::from(exp).dark_grey());
    }
    println!();
  }
  fn print_items(&self, hover: &mut usize) {
    for (i, item) in self.items.iter().enumerate() {
      match item {
        Item::Action { name, hotkey, exp } => {
          self.print_hotkey(&i, hotkey);
          self.print_name_exp(&i, hover, false, name, exp);
        }
        Item::SubMenu {
          name, hotkey, exp, ..
        } => {
          self.print_hotkey(&i, hotkey);
          self.print_name_exp(&i, hover, true, &("+".to_owned() + name), exp);
        }
        Item::Bool { name, hotkey, exp }
        | Item::Char { name, hotkey, exp }
        | Item::String { name, hotkey, exp }
        | Item::F64 { name, hotkey, exp }
        | Item::I64 { name, hotkey, exp }
        | Item::U64 { name, hotkey, exp } => {
          self.print_hotkey(&i, hotkey);
          self.print_name_exp(&i, hover, false, &(name.to_owned() + "="), exp);
        }
      }
    }
  }
  fn print_bottom(&self, is_sub: bool) {
    print!(
      "{}{}{}{}{}{}{}{}{}{}",
      "(".dark_grey(),
      "Up".yellow(),
      ")".dark_grey(),
      ", (".dark_grey(),
      "Down".yellow(),
      ")".dark_grey(),
      ", (".dark_grey(),
      "Enter".yellow(),
      ") ".dark_grey(),
      "Select",
    );
    if is_sub {
      print!(
        "{}{}{}{}",
        ", (".dark_grey(),
        "Backspace".yellow(),
        ") ".dark_grey(),
        "Back",
      );
      if self.esc {
        print!(
          "{}{}{}{}",
          ", (".dark_grey(),
          "Esc".yellow(),
          ") ".dark_grey(),
          "Exit",
        );
      }
    } else {
      if self.esc {
        print!(
          "{}{}{}{}",
          ", (".dark_grey(),
          "Esc".yellow(),
          ") ".dark_grey(),
          "Exit",
        );
      }
    }
    println!();
    println!(
      "{}",
      "Press an index number or a hotkey to select:".dark_grey()
    );
  }
  fn poll_read(&self) -> KeyCode {
    loop {
      if let Ok(Event::Key(KeyEvent { code, .. })) = read() {
        break code;
      }
    }
  }
  fn match_keycode(&self, keycode: KeyCode) -> Option<String> {
    match keycode {
      KeyCode::Up => Some(String::from("Up")),
      KeyCode::Down => Some(String::from("Down")),
      KeyCode::Enter => Some(String::from("Enter")),
      KeyCode::Esc => Some(String::from("Exit")),
      KeyCode::Backspace => Some(String::from("Back")),
      KeyCode::Char(chr) => Some(chr.to_string().to_lowercase()),
      _ => None,
    }
  }
  fn match_selection(
    &self,
    key: &Option<String>,
    is_sub: bool,
    stdout_ins: &mut Stdout,
    path: &mut Vec<String>,
    hover: &mut usize,
  ) -> Result<Selection, String> {
    if *key == None {
      return Err("No Selection".to_string());
    } else if is_sub && *key == Some("Back".to_string()) {
      self.flush_stdout(stdout_ins);
      return Err("Back".to_string());
    } else if *key == Some("Exit".to_string()) {
      if self.esc {
        self.flush_stdout(stdout_ins);
        stdout_ins.flush().unwrap();
        return Err("Exit".to_string());
      }
    } else if *key == Some("Up".to_string()) {
      if *hover > 0 {
        *hover -= 1;
        self.flush_stdout(stdout_ins);
        if path.len() == 1 {
          return self.printer(stdout_ins, hover);
        } else {
          return self.printer_sub(path, stdout_ins, hover);
        }
      }
    } else if *key == Some("Down".to_string()) {
      if (*hover + 1) < self.items.len() {
        *hover += 1;
        self.flush_stdout(stdout_ins);
        if path.len() == 1 {
          return self.printer(stdout_ins, hover);
        } else {
          return self.printer_sub(path, stdout_ins, hover);
        }
      }
    }
    for (i, item) in self.items.iter().enumerate() {
      match item {
        Item::Action { name, hotkey, .. } => {
          if (*key == hotkey.map(|f| f.to_string()))
            || (*key == Some(i.to_string()))
            || (*key == Some("Enter".to_string()) && i == *hover)
          {
            self.flush_stdout(stdout_ins);
            stdout_ins.flush().unwrap();
            path.push(name.to_string());
            return Ok(Selection {
              name: name.to_string(),
              path: path.to_vec(),
              value: None,
            });
          } else {
            continue;
          }
        }
        Item::SubMenu {
          name,
          hotkey,
          exp,
          items,
        } => {
          if (*key == hotkey.map(|f| f.to_string()))
            || (*key == Some(i.to_string()))
            || (*key == Some("Enter".to_string()) && i == *hover)
          {
            self.flush_stdout(stdout_ins);
            path.push(name.to_string());
            let sub_menu = Menu {
              name: name.to_string(),
              items: items.clone(),
              exp: exp.as_ref().map(|f| String::from(f)),
              esc: self.esc,
            };
            let sub_result = sub_menu.run_sub(path);
            match sub_result {
              Ok(ok) => return Ok(ok),
              Err(err) if &err == "Back" => {
                path.pop();
                if path.len() == 1 {
                  return self.run();
                } else {
                  return self.run_sub(path);
                }
              }
              Err(err) => return Err(err),
            }
          } else {
            continue;
          }
        }
        Item::Bool { name, hotkey, exp } => {
          if (*key == hotkey.map(|f| f.to_string()))
            || (*key == Some(i.to_string()))
            || (*key == Some("Enter".to_string()) && i == *hover)
          {
            self.flush_stdout(stdout_ins);
            path.push(name.to_string());
            let sub_menu = Menu {
              name: name.to_string(),
              items: vec![
                Item::Action {
                  name: "true".to_string(),
                  exp: None,
                  hotkey: Some('t'),
                },
                Item::Action {
                  name: "false".to_string(),
                  exp: None,
                  hotkey: Some('f'),
                },
              ],
              exp: exp.as_ref().map(|f| String::from(f)),
              esc: self.esc,
            };
            let sub_result = sub_menu.run_sub(path);
            match sub_result {
              Ok(mut ok) => {
                return {
                  let last = ok.path.pop().expect("item bool path pop");
                  ok.value = Some(Value::Bool(last.parse().expect("item bool value parse")));
                  Ok(ok)
                }
              }
              Err(err) if &err == "Back" => {
                path.pop();
                if path.len() == 1 {
                  return self.run();
                } else {
                  return self.run_sub(path);
                }
              }
              Err(err) => return Err(err),
            }
          } else {
            continue;
          }
        }
        Item::Char { name, hotkey, exp }
        | Item::String { name, hotkey, exp }
        | Item::F64 { name, hotkey, exp }
        | Item::I64 { name, hotkey, exp }
        | Item::U64 { name, hotkey, exp } => {
          if (*key == hotkey.map(|f| f.to_string()))
            || (*key == Some(i.to_string()))
            || (*key == Some("Enter".to_string()) && i == *hover)
          {
            // (done): flush
            self.flush_stdout(stdout_ins);
            path.push(name.to_string());
            // (done): read line
            self.print_top(path);
            self.print_name(name, exp);
            // (done): selection
            let input = self.read_line_string();
            // (done): match input
            let selection = match item {
              Item::Char { .. } => {
                let value: char = self.match_input(input);
                Selection {
                  name: name.to_string(),
                  path: path.to_vec(),
                  value: Some(Value::Char(value)),
                }
              }
              Item::F64 { .. } => {
                let value: f64 = self.match_input(input);
                Selection {
                  name: name.to_string(),
                  path: path.to_vec(),
                  value: Some(Value::F64(value)),
                }
              }
              Item::I64 { .. } => {
                let value: i64 = self.match_input(input);
                Selection {
                  name: name.to_string(),
                  path: path.to_vec(),
                  value: Some(Value::I64(value)),
                }
              }
              Item::U64 { .. } => {
                let value: u64 = self.match_input(input);
                Selection {
                  name: name.to_string(),
                  path: path.to_vec(),
                  value: Some(Value::U64(value)),
                }
              }
              _ => Selection {
                name: name.to_string(),
                path: path.to_vec(),
                value: Some(Value::String(input)),
              },
            };
            return Ok(selection);
          } else {
            continue;
          }
        }
      };
    }
    Err("No Selection".to_string())
  }
  fn flush_stdout(&self, stdout_ins: &mut Stdout) {
    let rows = 3;
    stdout_ins
      .queue(cursor::MoveUp(self.items.len() as u16 + rows))
      .expect("cursor move up");
    stdout_ins
      .queue(terminal::Clear(ClearType::FromCursorDown))
      .expect("terminal clear");
  }
  fn print_hotkey(&self, index: &usize, hotkey: &Option<char>) {
    print!("{}{}", index.to_string().yellow(), ".".dark_grey());
    match hotkey {
      Some(chr) => print!(
        "{}{}{}",
        "(".dark_grey(),
        chr.to_string().to_uppercase().yellow(),
        ")".dark_grey()
      ),
      None => print!("   "),
    }
  }
  fn print_name(&self, name: &String, item_exp: &Option<String>) {
    if let Some(item_exp) = item_exp {
      println!(
        "       {} {}",
        String::from(name.to_owned() + "=").cyan(),
        String::from(item_exp).dark_grey()
      );
    } else {
      println!("       {} ", String::from(name.to_owned() + "=").cyan());
    }
    println!("{}", "Enter value:".dark_grey());
  }
  fn print_name_exp(
    &self,
    index: &usize,
    hover: &mut usize,
    offset: bool,
    name: &String,
    exp: &Option<String>,
  ) {
    let space;
    if offset {
      space = " ";
    } else {
      space = "  ";
    }
    if index == hover {
      print!("{}{}", space, String::from(name).cyan());
    } else {
      print!("{}{}", space, name);
    }
    if let Some(exp_str) = exp {
      print!(" {}", String::from(exp_str).dark_grey());
    }
    println!();
  }
  fn read_line_string(&self) -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("read line");
    input.trim().to_string()
  }
  fn match_input<T: FromStr>(&self, input: String) -> T {
    match input.parse() {
      Ok(ok) => ok,
      Err(_) => {
        let input = self.read_line_string();
        self.match_input(input)
      }
    }
  }
}
