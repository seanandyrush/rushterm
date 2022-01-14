//! # Rushterm
//! Make your CLI app easy by adding menu. Create nested menus, navigate with hotkeys. Data-driven. No function/macro complexity.
//! # Example
//! Firstly, we'll need to construct a `Menu` instance with its `Item`s. Bring them into scope. `Menu` instance doesn't need to be mutable. Next, we'll invoke `.run()` method on the instance to execute our menu:
//! ```rust
//! use rushterm::{Item, Menu};
//!
//! fn main() {
//!     let menu = Menu {
//!         name: "My Main Menu",
//!         items: vec![
//!             Item::Action {
//!                 name: "Action0",
//!                 hotkey: Some('a'),
//!                 exp: Some("Action0 Explanation. This Has Been Assigned To A Hotkey."),
//!             },
//!             Item::Action {
//!                 name: "Action1",
//!                 hotkey: None,
//!                 exp: Some("Action1 Explanation. This Has No Hotkey."),
//!             },
//!             Item::SubMenu {
//!                 name: "Submenu0",
//!                 hotkey: Some('s'),
//!                 exp: Some("Submenu0 explanation."),
//!                 items: vec![
//!                     Item::Action {
//!                         name: "Sub0 Action0",
//!                         hotkey: Some('a'),
//!                         exp: Some("Sub Action0 Explanation. This Has Been Assigned To A Hotkey."),
//!                     },
//!                     Item::SubMenu {
//!                         name: "Deepermenu0",
//!                         hotkey: Some('d'),
//!                         exp: Some("Deepermenu0 Explanation."),
//!                         items: vec![
//!                             Item::Action {
//!                                 name: "Deeper Action0",
//!                                 hotkey: Some('f'),
//!                                 exp: None,
//!                             },
//!                             Item::Action {
//!                                 name: "Deeper Action1",
//!                                 hotkey: Some('g'),
//!                                 exp: Some("Deeper Action1 Explanation."),
//!                             },
//!                         ],
//!                     },
//!                 ],
//!             },
//!         ],
//!         exp: Some("My Main Menu Explanation."),
//!         esc: true,
//!     };
//!     let selection = menu.run();
//!     dbg!(&selection);
//! }
//! ```
//! If selection is successful, `run()` method will return us `Selection` type in `Ok()` variant to get information we may need in ongoing execution. If not, exits the execution with an `Err()` variant.

use crossterm::{
    cursor,
    event::{poll, read, Event, KeyCode, KeyEvent},
    style::Stylize,
    terminal::{self, ClearType},
    QueueableCommand,
};
use std::{
    io::{stdout, Stdout, Write},
    time::Duration,
};
/// Anything that can be listed in `Menu`.
#[derive(Clone)]
pub enum Item<'a> {
    /// A menu item to execute an action. Exits `Menu`.
    Action {
        /// Action name.
        name: &'a str,
        /// Assigning a hotkey to the item is optional. The hotkey is displayed in yellow.
        hotkey: Option<char>,
        /// Optional explanation in gray color is displayed next to the item.
        exp: Option<&'a str>,
    },
    /// A menu item to enter branch menus. Eclipses `Menu` or another `SubMenu`.
    SubMenu {
        /// Sub menu name. It can be distinguished by the `+` character before it.
        name: &'a str,
        /// Assigning a hotkey to the item is optional. The hotkey is displayed in yellow.
        hotkey: Option<char>,
        /// Optional explanation in gray color is displayed next to the item.
        exp: Option<&'a str>,
        /// `SubMenu` items should be vector of `Item`s.
        items: Vec<Item<'a>>,
    },
}
/// Starting point for creating a menu instance.
pub struct Menu<'a> {
    /// `Menu` name is displayed at the top.
    pub name: &'a str,
    /// Optional explanation in gray color next to the menu name.
    pub exp: Option<&'a str>,
    /// `Menu` items should be vector of `Item`s.
    pub items: Vec<Item<'a>>,
    /// Enable exiting menu by `Esc` hotkey. Usually set it to `true`. But it may be useful to set to `false` when you want to restrict the user from escaping without any selection.
    pub esc: bool,
}
/// Gives the data of the selection made in the menu.
#[derive(Debug, PartialEq)]
pub struct Selection {
    /// Name of selected `Item`.
    name: String,
    /// Vector containing direction of the selected item in the menu tree.
    path: Vec<String>,
}

impl<'a> Menu<'a> {
    /// Prints out `Item`s, executes the `Menu` and returns `Result`.
    pub fn run(&self) -> Result<Selection, String> {
        let mut stdout_ins = stdout();
        let mut hover = 0 as usize;
        self.printer(&mut stdout_ins, &mut hover)
    }
    fn printer(&self, stdout_ins: &mut Stdout, hover: &mut usize) -> Result<Selection, String> {
        self.print_top(&vec![self.name.to_string()]);
        self.print_items(false, hover);
        self.print_bottom();
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
        self.print_items(true, hover);
        self.print_bottom();
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
        if let Some(exp) = self.exp {
            print!(" {}", exp.dark_grey());
        }
        println!();
    }
    fn print_items(&self, is_sub: bool, hover: &mut usize) {
        for (i, item) in self.items.iter().enumerate() {
            match *item {
                Item::Action { name, hotkey, exp } => {
                    print!("{}{}", i.to_string().yellow(), ".".dark_grey());
                    match hotkey {
                        Some(chr) => print!(
                            "{}{}{}",
                            "(".dark_grey(),
                            chr.to_string().to_uppercase().yellow(),
                            ")".dark_grey()
                        ),
                        None => print!("   "),
                    }
                    if i == *hover {
                        print!("  {}", name.cyan());
                    } else {
                        print!("  {}", name);
                    }
                    if let Some(exp_str) = exp {
                        print!(" {}", exp_str.dark_grey());
                    }
                    println!();
                }
                Item::SubMenu {
                    name, hotkey, exp, ..
                } => {
                    print!("{}{}", i.to_string().yellow(), ".".dark_grey());
                    match hotkey {
                        Some(chr) => print!(
                            "{}{}{}",
                            "(".dark_grey(),
                            chr.to_string().to_uppercase().yellow(),
                            ")".dark_grey()
                        ),
                        None => print!("   "),
                    }
                    if i == *hover {
                        print!(" +{}", name.cyan());
                    } else {
                        print!(" +{}", name);
                    }
                    if let Some(exp_str) = exp {
                        print!(" {}", exp_str.dark_grey());
                    }
                    println!();
                }
            }
        }
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
    }
    fn print_bottom(&self) {
        println!(
            "{}",
            "Press an index number or a hotkey to select:".dark_grey()
        )
    }
    fn poll_read(&self) -> KeyCode {
        if let Ok(true) = poll(Duration::MAX) {
            if let Ok(Event::Key(KeyEvent { code, .. })) = read() {
                return code;
            } else {
                return KeyCode::Esc;
            }
        } else {
            return KeyCode::Esc;
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
            self.flush_stdout(stdout_ins, is_sub);
            return Err("Back".to_string());
        } else if *key == Some("Exit".to_string()) {
            if self.esc {
                self.flush_stdout(stdout_ins, is_sub);
                stdout_ins.flush().unwrap();
                return Err("Exit".to_string());
            }
        } else if *key == Some("Up".to_string()) {
            if *hover > 0 {
                *hover -= 1;
                self.flush_stdout(stdout_ins, is_sub);
                if path.len() == 1 {
                    return self.printer(stdout_ins, hover);
                } else {
                    return self.printer_sub(path, stdout_ins, hover);
                }
            }
        } else if *key == Some("Down".to_string()) {
            if (*hover + 1) < self.items.len() {
                *hover += 1;
                self.flush_stdout(stdout_ins, is_sub);
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
                        self.flush_stdout(stdout_ins, is_sub);
                        stdout_ins.flush().unwrap();
                        path.push(name.to_string());
                        return Ok(Selection {
                            name: name.to_string(),
                            path: path.to_vec(),
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
                        self.flush_stdout(stdout_ins, is_sub);
                        path.push(name.to_string());
                        let sub_menu = Menu {
                            name: *name,
                            items: items.clone(),
                            exp: *exp,
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
            };
        }
        Err("No Selection".to_string())
    }
    fn flush_stdout(&self, stdout_ins: &mut Stdout, is_sub: bool) {
        let mut rows = 3;
        if !self.esc && !is_sub {
            rows -= 1;
        }
        stdout_ins
            .queue(cursor::MoveUp(self.items.len() as u16 + rows))
            .unwrap();
        stdout_ins
            .queue(terminal::Clear(ClearType::FromCursorDown))
            .unwrap();
    }
}