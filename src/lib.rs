//! # Rushterm
//! Create nested menus on the terminal, navigate with hotkeys. Data-driven. No function/macro complexity.
//! # Example
//! Firstly, we'll need to construct a `Menu` instance. Bring `Menu` and necessary sub types into scope. `Menu` instance doesn't need to be mutable. Next, we'll chain `.print()` and `.run()` methods on the instance to execute our menu:
//! ```
//! use rushterm::{Action, Item, Menu, SubMenu};
//!
//! fn main() {
//!     let menu = Menu {
//!         name: "My Main Menu",
//!         items: vec![
//!             Item::Action(Action {
//!                 name: "Action0",
//!                 hotkey: Some('a'),
//!                 exp: Some("Action0 Explanation. This Has Been Assigned To A Hotkey."),
//!             }),
//!             Item::Action(Action {
//!                 name: "Action1",
//!                 hotkey: None,
//!                 exp: Some("Action1 Explanation. This Has No Hotkey."),
//!             }),
//!             Item::SubMenu(SubMenu {
//!                 name: "Submenu0",
//!                 hotkey: Some('s'),
//!                 exp: Some("Submenu0 explanation."),
//!                 items: vec![
//!                     Item::Action(Action {
//!                         name: "Sub0 Action0",
//!                         hotkey: Some('a'),
//!                         exp: Some("Sub Action0 Explanation. This Has Been Assigned To A Hotkey."),
//!                     }),
//!                     Item::SubMenu(SubMenu {
//!                         name: "Deepermenu0",
//!                         hotkey: Some('d'),
//!                         exp: Some("Deepermenu0 Explanation."),
//!                         items: vec![
//!                             Item::Action(Action {
//!                                 name: "Deeper Action0",
//!                                 hotkey: Some('f'),
//!                                 exp: None,
//!                             }),
//!                             Item::Action(Action {
//!                                 name: "Deeper Action1",
//!                                 hotkey: Some('g'),
//!                                 exp: Some("Deeper Action1 Explanation."),
//!                             }),
//!                         ],
//!                     }),
//!                 ],
//!             }),
//!         ],
//!         exp: Some("My Main Menu Explanation."),
//!     };
//!     let selection = menu.print().run();
//!     dbg!(&selection);
//! }
//! ```
//! If selection is successful, `run()` method will return us a `Selection` type in `Ok()` variant to get information we may need in ongoing execution. If not, exits the execution with an `Err()` variant.

use crossterm::event::{poll, read, Event, KeyCode, KeyEvent};
use std::time::Duration;
/// Anything that can be listed in a menu.
#[derive(Clone)]
pub enum Item<'a> {
    SubMenu(SubMenu<'a>),
    Action(Action<'a>),
}
/// Starting point for creating a menu instance.
#[derive(Clone)]
pub struct Menu<'a> {
    pub name: &'a str,
    pub exp: Option<&'a str>,
    pub items: Vec<Item<'a>>,
}
/// A menu item to enter branch menus.
#[derive(Clone)]
pub struct SubMenu<'a> {
    pub name: &'a str,
    pub hotkey: Option<char>,
    pub exp: Option<&'a str>,
    pub items: Vec<Item<'a>>,
}
/// A menu item to execute an action.
#[derive(Clone)]
pub struct Action<'a> {
    pub name: &'a str,
    pub hotkey: Option<char>,
    pub exp: Option<&'a str>,
}
/// Gives the data of the selection made in the menu.
#[derive(Debug, PartialEq)]
pub struct Selection {
    name: String,
    path: Vec<String>,
}

impl<'a> Menu<'a> {
    /// Prints the items in the menu.
    pub fn print(&self) -> &Self {
        self.print_top(&vec![self.name.to_string()]);
        self.print_items(false);
        self.print_bottom();
        self
    }
    /// Executes the menu and returns a `Result`.
    pub fn run(&self) -> Result<Selection, String> {
        let keycode = self.poll_read();
        let key = self.match_keycode(keycode);
        let res = self.match_selection(&key, false, &mut vec![self.name.to_string()]);
        if res == Err("No Selection".to_string()) {
            self.run()
        } else {
            res
        }
    }
    fn print_sub(&self, path: &Vec<String>) -> &Self {
        self.print_top(path);
        self.print_items(true);
        self.print_bottom();
        self
    }
    fn run_sub(&self, path: &mut Vec<String>) -> Result<Selection, String> {
        let keycode = self.poll_read();
        let key = self.match_keycode(keycode);
        let res = self.match_selection(&key, true, path);
        if res == Err("No Selection".to_string()) {
            self.run_sub(path)
        } else {
            res
        }
    }
    fn print_top(&self, path: &Vec<String>) {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        for dir in path {
            print!("{}/", dir);
        }
        println!();
        if let Some(exp) = &self.exp {
            println!("{}", exp);
        }
        println!();
    }
    fn print_items(&self, is_sub: bool) {
        for (i, item) in self.items.iter().enumerate() {
            match item {
                Item::SubMenu(submenu) => {
                    print!("{}.", i);
                    match &submenu.hotkey {
                        Some(hotkey) => print!("({})", hotkey.to_uppercase()),
                        None => print!("   "),
                    }
                    print!(" +{}", &submenu.name);
                    if let Some(exp) = &submenu.exp {
                        print!(": \"{}\"", exp);
                    }
                    println!();
                }
                Item::Action(action) => {
                    print!("{}.", i);
                    match &action.hotkey {
                        Some(hotkey) => print!("({})", hotkey.to_uppercase()),
                        None => print!("   "),
                    }
                    print!("  {}", &action.name);
                    if let Some(exp) = &action.exp {
                        print!(": \"{}\"", exp);
                    }
                    println!();
                }
            }
        }
        if is_sub {
            println!("(Bksp) Back");
            println!("(Esc)  Exit");
        } else {
            println!("(Esc)  Exit");
        }
    }
    fn print_bottom(&self) {
        println!();
        println!("Press a index number or a hotkey to select:")
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
        path: &mut Vec<String>,
    ) -> Result<Selection, String> {
        if *key == None {
            return Err("No Selection".to_string());
        }
        if is_sub && *key == Some("Back".to_string()) {
            return Err("Back".to_string());
        }
        if *key == Some("Exit".to_string()) {
            return Err("Exit".to_string());
        }
        for (i, item) in self.items.iter().enumerate() {
            match item {
                Item::Action(action) => {
                    if (*key == action.hotkey.map(|f| f.to_string()))
                        || (*key == Some(i.to_string()))
                    {
                        path.push(action.name.to_string());
                        return Ok(Selection {
                            name: action.name.to_string(),
                            path: path.to_vec(),
                        });
                    } else {
                        continue;
                    }
                }
                Item::SubMenu(submenu) => {
                    if (*key == submenu.hotkey.map(|f| f.to_string()))
                        || (*key == Some(i.to_string()))
                    {
                        path.push(submenu.name.to_string());
                        let menu = Menu {
                            name: submenu.name,
                            items: submenu.items.clone(),
                            exp: submenu.exp,
                        };
                        let sub_result = menu.print_sub(path).run_sub(path);
                        match sub_result {
                            Ok(ok) => return Ok(ok),
                            Err(err) if &err == "Back" => {
                                path.pop();
                                if path.len() == 1 {
                                    return self.print().run();
                                } else {
                                    return self.print_sub(path).run_sub(path);
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
}
