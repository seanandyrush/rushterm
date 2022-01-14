//! # Rushterm
//! Make your CLI app easy by adding menu. Create nested menus, navigate with hotkeys. Data-driven. No function/macro complexity.
//! # Example
//! Firstly, we'll need to construct a `Menu` instance. Bring `Menu` and necessary sub types into scope. `Menu` instance doesn't need to be mutable. Next, we'll invoke `.run()` method on the instance to execute our menu:
//! ```
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
//! If selection is successful, `run()` method will return us a `Selection` type in `Ok()` variant to get information we may need in ongoing execution. If not, exits the execution with an `Err()` variant.

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
/// Anything that can be listed in a menu.
#[derive(Clone)]
pub enum Item<'a> {
    /// A menu item to execute an action.
    Action {
        name: &'a str,
        hotkey: Option<char>,
        exp: Option<&'a str>,
    },
    /// A menu item to enter branch menus.
    SubMenu {
        name: &'a str,
        hotkey: Option<char>,
        exp: Option<&'a str>,
        items: Vec<Item<'a>>,
    },
}
/// Starting point for creating a menu instance.
pub struct Menu<'a> {
    pub name: &'a str,
    pub exp: Option<&'a str>,
    pub items: Vec<Item<'a>>,
    /// enable exiting menu by `Esc` hotkey.
    pub esc: bool,
}
/// Gives the data of the selection made in the menu.
#[derive(Debug, PartialEq)]
pub struct Selection {
    name: String,
    path: Vec<String>,
}

impl<'a> Menu<'a> {
    /// Prints the items, executes the menu and returns a `Result`.
    pub fn run(&self) -> Result<Selection, String> {
        let mut stdout_ins = stdout();
        self.print_top(&vec![self.name.to_string()]);
        self.print_items(false);
        self.print_bottom();
        self.matcher(&mut stdout_ins)
    }
    fn matcher(&self, stdout_ins: &mut Stdout) -> Result<Selection, String> {
        let keycode = self.poll_read();
        let key = self.match_keycode(keycode);
        let res = self.match_selection(&key, false, stdout_ins, &mut vec![self.name.to_string()]);
        if res == Err("No Selection".to_string()) {
            self.matcher(stdout_ins)
        } else {
            res
        }
    }
    fn run_sub(&self, path: &mut Vec<String>) -> Result<Selection, String> {
        let mut stdout_ins = stdout();
        self.print_top(path);
        self.print_items(true);
        self.print_bottom();
        self.matcher_sub(&mut stdout_ins, path)
    }
    fn matcher_sub(
        &self,
        stdout_ins: &mut Stdout,
        path: &mut Vec<String>,
    ) -> Result<Selection, String> {
        let keycode = self.poll_read();
        let key = self.match_keycode(keycode);
        let res = self.match_selection(&key, true, stdout_ins, path);
        if res == Err("No Selection".to_string()) {
            self.matcher_sub(stdout_ins, path)
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
    fn print_items(&self, is_sub: bool) {
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
                    print!("  {}", name);
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
                    print!(" +{}", name);
                    if let Some(exp_str) = exp {
                        print!(" {}", exp_str.dark_grey());
                    }
                    println!();
                }
            }
        }
        if is_sub {
            print!(
                "{}{}{}{}",
                "(".dark_grey(),
                "Backspace".yellow(),
                ")".dark_grey(),
                " Back".magenta()
            );
            if self.esc {
                print!(
                    "{}{}{}{}",
                    ", (".dark_grey(),
                    "Esc".yellow(),
                    ")".dark_grey(),
                    " Exit".red()
                );
            }
            println!();
        } else {
            if self.esc {
                println!(
                    "{}{}{}{}",
                    "(".dark_grey(),
                    "Esc".yellow(),
                    ")".dark_grey(),
                    " Exit".red()
                );
            }
        }
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
        }
        for (i, item) in self.items.iter().enumerate() {
            match item {
                Item::Action { name, hotkey, .. } => {
                    if (*key == hotkey.map(|f| f.to_string())) || (*key == Some(i.to_string())) {
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
                    if (*key == hotkey.map(|f| f.to_string())) || (*key == Some(i.to_string())) {
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
