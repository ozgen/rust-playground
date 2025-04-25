use hello::greetings; // 'use' words is used for import
const T_FACTOR: f64 = 9.9; // const must write Capital snake case,
                           // type is mandatory, and it is immutable
                           // the variable is global and compile time really fast

fn main() {
    println!("Hello, world!");
    let count = 1; // variables are immutable by default
    // count = 2; // ERROR!
    let mut number = 6; // use mut if you want variable mutable
    number = 7;
    println!("{}", number);

    // variable has scope
    let x = 5;
    {
        let y = 99;
        println!("{} - {}", x, y);
    }
    // println!("{} - {}", x, y); // ERROR! because y deleted after the {} passed
    let mut z = 5;
    {
        z = 99;
        println!("{}", z); // print 99
    }
    println!("{}", z);  // print 99

    let mut a = 5; // a is mutable
    let a = a;     // a is now immutable
    println!("{}", a);

    let enigma: i32; // you net to set variable value rather than 'C'
    if true {
        enigma = 42;
    } else {
        enigma = 7;
    }
    println!("enigma is {}", enigma);

    let mut res = do_stuff(2.0, 12.5);

    println!("result: {}", res);

    res = do_same_stuff(2.0, 12.5);

    println!("result 2: {}", res);

    // call func from another rs file
   // 1_hello::greetings();
    // if use 'use'
    greetings();

    let my_letter = 'a';

    // compound types
    let info :(u8, f64, i32) = (1, 3.5, 100); // tuples max variable count : 12
    let first = info.0;
    let sec = info.1;
    let third = info.2;
     // OR
    // let (first, sec, third) = info;
}


fn do_stuff(qty:f64, oz:f64) -> f64 {
    return qty * oz;
}
fn do_same_stuff(qty:f64, oz:f64) -> f64 {
     qty * oz
}