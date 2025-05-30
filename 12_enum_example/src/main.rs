use std::fs::File;

/**
Enums is like algebraic data types
Enums works like unions in C and much more optimized...
*/

enum Color {
    Red,
    Green,
    Blue,
}

enum TestItem {
    Empty,
    Ammo(u8),
    Things(String, u32),
    Place { x: i32, y: i32 },
}

// enum Option<T> {
//     Some(T),
//     None,
// }

fn main() {
    let color = Color::Red;

    use TestItem::*;
    let item0 = Empty;
    let item1 = Ammo(6);
    let item2 = Things("test".to_string(), 45);
    let item3 = Place { x: 12, y: 45 };

    let mut x: Option<i32> = None;
    x = Some(7);
    if x.is_some() {
        println!("{}", x.unwrap())
    }

    for i in x {
        println!("{}", i); // print 7
    }

    let res = File::open("foo");
    match res {
        Ok(fl) => {}
        Err(e) => {}
    }
    // if res.is_ok() {
    //     let f = res.unwrap();
    // }
}
