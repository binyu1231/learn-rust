// crate name config in Cargo.toml
use gui::{Screen, Button, TextField};

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(Button {
                width: 100,
                height: 20,
                label: String::from("Create"),
            }),
            Box::new(TextField {
                width: 100,
                height: 20,
                label: String::from("Username"),
                placeholder: String::from("Please input username."),
            })
        ]
    };

    screen.run();
    println!("Hello, world!");
}
