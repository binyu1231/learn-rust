pub struct Screen {
    pub components: Vec<Box<dyn Draw>>
}
impl Screen {
    pub fn run(&self) {
        for cmp in self.components.iter() {
            cmp.draw();
        }
    }
}

pub trait Draw {
    fn draw(&self);
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Draw a button {}.", &self.label);
    }
}

pub struct TextField {
    pub width: u32,
    pub height: u32,
    pub label: String,
    pub placeholder: String,
}

impl Draw for TextField {
    fn draw(&self) {
        println!("Draw a textfield {}.", &self.label);
    }
}
