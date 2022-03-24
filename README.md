# Donation
If you like this crate, you could become a backer or sponsor or donator via PayPal. Any amount is welcome.

link: https://www.paypal.com/donate/?hosted_button_id=AN536BKRLP8MG

<form action="https://www.paypal.com/donate" method="post" target="_top">
<input type="hidden" name="hosted_button_id" value="AN536BKRLP8MG" />
<input type="image" src="https://www.paypalobjects.com/en_US/i/btn/btn_donateCC_LG.gif" border="0" name="submit" title="PayPal - The safer, easier way to pay online!" alt="Donate with PayPal button" />
<img alt="" border="0" src="https://www.paypal.com/en_CO/i/scr/pixel.gif" width="1" height="1" />
</form>

Thank you!

Sean Andy Rush.

# Rushterm
Make your CLI app easy by adding menu. Create nested menus, navigate with hotkeys. Data-driven. No function/macro complexity.
# Example
Firstly, we'll need to construct a `Menu` instance with its `Item`s. Bring them into scope. `Menu` instance doesn't need to be mutable. Next, we'll invoke `.run()` method on the instance to execute our menu:
```rust
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
            Item::SubMenu {
                name: "Submenu0".to_string(),
                hotkey: Some('s'),
                exp: Some("Submenu0 explanation.".to_string()),
                items: vec![
                    Item::Action {
                        name: "Sub Action0".to_string(),
                        hotkey: Some('a'),
                        exp: Some(
                            "Sub Action0 Explanation. This Has Been Assigned To A Hotkey."
                                .to_string(),
                        ),
                    },
                    Item::Action {
                        name: "Sub Action1".to_string(),
                        hotkey: Some('c'),
                        exp: Some(
                            "Sub Action1 Explanation. This Has Been Assigned To A Hotkey."
                                .to_string(),
                        ),
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
            Item::Bool {
                name: "Bool0".to_string(),
                hotkey: Some('b'),
                exp: Some("Bool0 Explanation.".to_string()),
            },
            Item::Char {
                name: "Char0".to_string(),
                hotkey: Some('c'),
                exp: Some("Char0 Explanation.".to_string()),
            },
            Item::String {
                name: "String0".to_string(),
                hotkey: Some('t'),
                exp: Some("String0 Explanation.".to_string()),
            },
            Item::F32 {
                name: "F32".to_string(),
                hotkey: Some('f'),
                exp: Some("F32 Explanation.".to_string()),
            },
            Item::I32 {
                name: "I32".to_string(),
                hotkey: Some('i'),
                exp: Some("I32 Explanation.".to_string()),
            },
            Item::U32 {
                name: "U32".to_string(),
                hotkey: Some('u'),
                exp: Some("U32 Explanation.".to_string()),
            },
        ],
        exp: Some("My Main Menu Explanation.".to_string()),
        esc: true,
    };
    let selection = menu.run();
    dbg!(&selection);
}

```
If selection is successful, `run()` method will return us `Selection` type in `Ok()` variant to get information we may need in ongoing execution. You may also bring `Selection` and `Value` into scope in this case. But, if not, exits the execution with an `Err()` variant.