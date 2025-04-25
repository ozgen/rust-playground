
// there are 6 types of strings
// &str -> can not be modified, has ptr and length
// String -> can be modified, has ptr, length and capacity
// Strings can not be index by character position

fn main() {

    let msg = "hello world".to_string();
    // or
    let msg2 = String::from("hello world");

    let _ = msg.bytes();
    let _ = msg.chars();
}
