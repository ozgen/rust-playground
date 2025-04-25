/**
1- each value has an owner, there is no value in memory
2- there is only one owner of a value, there is no share for ownership
3- value gets dropped if its owner goes out of the scope
       stack            heap
     - in order      - unordered
     - fixed-size    - variable-size
     - LIFO          - unordered
     - fast          - slow

when the value is dropped:
- Destructor
- Free heap
- Pop stack
that saves the code from memory-leak

* you can have either, exactly one mutable reference or any number of immutable references
*/
fn main() {
    let s1 = String::from("abc");
    let s2 = s1;
    // println!("{}", s1) // Error!
    let s3 = s2.clone();
    println!("{}, {}", s2, s3);

    do_stuff(&s3);
    println!("{}", s3);

    let mut s4 = String::from("mutable-reference");
    do_stuff_mut(&mut s4);
    println!("{}", s4);
}

// references must always be VALID
fn do_stuff(s: &String) {
    // & means reference of s
    // do stuff
    println!("reference: {}", s);
}

fn do_stuff_mut(s: &mut String) {
    s.insert_str(0, "Hi, ");
    // or
    //(*s).insert_str(0, "Hi, ");
}
