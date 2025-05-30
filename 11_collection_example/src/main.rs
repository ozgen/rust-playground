use std::collections::HashMap;

fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(2);
    v.push(4);
    v.push(6);
    let x = v.pop(); // x is 6 
    
    println!("{} and x {:?}", v[1], x);
    
    let v1 = vec![2,4,6];

    for num in v1.iter() {
        println!("number : {}", num);
    }
    
    let mut hm: HashMap<u8, bool> = HashMap::new();
    hm.insert(5, true);
    hm.insert(6, false);
    
    let have_five = hm.remove(&5).unwrap();
    println!("{}", have_five);
}
