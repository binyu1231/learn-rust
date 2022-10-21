struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

#[derive(Debug)] // 外部属性
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    let user_foo = User {
        email: String::from("foo@bar.com"),
        username: String::from("foo"),
        active: true,
        sign_in_count: 1,
    };

    println!("{}, {}, {}, {}", user_foo.email, user_foo.username, user_foo.active, user_foo.sign_in_count);
    
    let mut user_baz = User {
        email: String::from("baz@bar.com"),
        username: String::from("baz"),
        active: true,
        sign_in_count: 1,
    };
    
    user_baz.active = false;
    
    println!("{}, {}, {}, {}", user_baz.email, user_baz.username, user_baz.active, user_baz.sign_in_count);
    
    let user_bao = build_user(String::from("bao@foo.com"), String::from("bao"));
    println!("{}, {}, {}, {}", user_bao.email, user_bao.username, user_bao.active, user_bao.sign_in_count);
    
    let user_baf = User {
        // email: String::from("baf@foo.com"),
        username: String::from("baf"),
        ..user_foo
    };
    println!("{}, {}, {}, {}", user_baf.email, user_baf.username, user_baf.active, user_baf.sign_in_count);
    println!("{}, {}, {}, {}", user_baf.email, user_foo.username, user_foo.active, user_foo.sign_in_count);

    let black = Color(0, 0, 0);
    let origin = Point(1, 1, 1);

    let r = black.0;
    let z = origin.2;

    
    println!("r {}", r);
    println!("z {}", z);
    println!("r {}, g {}, b {}", black.0, black.1, black.2);

    let rect = Rectangle {
        width: 30, height: 50,
    };

    println!("width {}, height {}", rect.width, rect.height);
    // 告诉println! 使用叫做 Debug 的输出格式{:?}
    println!("rect is {:?}", rect);
    println!(">>>>>>>>>>>>>");
    dbg!(&rect);
    println!("rect area is {}", rect.area());

    println!("square area is {}", Rectangle::square(20).area());
   
}
