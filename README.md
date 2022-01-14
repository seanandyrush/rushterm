# Rushterm
Make your CLI app easy by adding menu. Create nested menus, navigate with hotkeys. Data-driven. No function/macro complexity.
# Example
Firstly, we'll need to construct a `Menu` instance. Bring `Menu` and necessary sub types into scope. `Menu` instance doesn't need to be mutable. Next, we'll invoke `.run()` method on the instance to execute our menu:
```rust
use rushterm::{Item, Menu};

fn main() {
    let menu = Menu {
        name: "My Main Menu",
        items: vec![
            Item::Action {
                name: "Action0",
                hotkey: Some('a'),
                exp: Some("Action0 Explanation. This Has Been Assigned To A Hotkey."),
            },
            Item::Action {
                name: "Action1",
                hotkey: None,
                exp: Some("Action1 Explanation. This Has No Hotkey."),
            },
            Item::SubMenu {
                name: "Submenu0",
                hotkey: Some('s'),
                exp: Some("Submenu0 explanation."),
                items: vec![
                    Item::Action {
                        name: "Sub0 Action0",
                        hotkey: Some('a'),
                        exp: Some("Sub Action0 Explanation. This Has Been Assigned To A Hotkey."),
                    },
                    Item::SubMenu {
                        name: "Deepermenu0",
                        hotkey: Some('d'),
                        exp: Some("Deepermenu0 Explanation."),
                        items: vec![
                            Item::Action {
                                name: "Deeper Action0",
                                hotkey: Some('f'),
                                exp: None,
                            },
                            Item::Action {
                                name: "Deeper Action1",
                                hotkey: Some('g'),
                                exp: Some("Deeper Action1 Explanation."),
                            },
                        ],
                    },
                ],
            },
        ],
        exp: Some("My Main Menu Explanation."),
        exit: true,
    };
    let selection = menu.run();
    dbg!(&selection);
}
```
If selection is successful, `run()` method will return us a `Selection` type in `Ok()` variant to get information we may need in ongoing execution. If not, exits the running with an `Err()` variant.