use rushterm::{Item, Menu};

fn main() {
  let menu = Menu {
    name: "My Main Menu".to_string(),
    items: vec![
      Item::Action {
        name: "Action0".to_string(),
        hotkey: Some('a'),
        exp: Some("Action0 Explanation. This Has Been Assigned To A Hotkey.".to_string()),
      },
      Item::Action {
        name: "Action1".to_string(),
        hotkey: None,
        exp: Some("Action1 Explanation. This Has No Hotkey.".to_string()),
      },
      Item::InputText {
        name: "InputText0".to_string(),
        hotkey: Some('i'),
        exp: Some("InputText0 Explanation.".to_string()),
      },
      Item::SubMenu {
        name: "Submenu0".to_string(),
        hotkey: Some('s'),
        exp: Some("Submenu0 explanation.".to_string()),
        items: vec![
          Item::Action {
            name: "Sub Action0".to_string(),
            hotkey: Some('a'),
            exp: Some("Sub Action0 Explanation. This Has Been Assigned To A Hotkey.".to_string()),
          },
          Item::Action {
            name: "Sub Action1".to_string(),
            hotkey: Some('c'),
            exp: Some("Sub Action1 Explanation. This Has Been Assigned To A Hotkey.".to_string()),
          },
          Item::SubMenu {
            name: "Deepermenu0".to_string(),
            hotkey: Some('d'),
            exp: Some("Deepermenu0 Explanation.".to_string()),
            items: vec![
              Item::Action {
                name: "Deeper Action0".to_string(),
                hotkey: Some('f'),
                exp: None,
              },
              Item::Action {
                name: "Deeper Action1".to_string(),
                hotkey: Some('g'),
                exp: Some("Deeper Action1 Explanation.".to_string()),
              },
            ],
          },
        ],
      },
    ],
    exp: Some("My Main Menu Explanation.".to_string()),
    esc: false,
  };
  let selection = menu.run();
  dbg!(&selection);
}
