fn main() {
    let msg;
    let num = 5;

    // if num == 5 {
    //     msg = "five";
    // } else if num == 4 {
    //     msg = "four";
    // } else {
    //     msg = "zero";
    // }
    // Instead

    msg = if num == 5 {
        // curly braces are mandatory
        "five" // do not use return
    } else if num == 4 {
        "four"
    } else {
        "zero" // all the return value should be the same TYPE...
    }; // we need to add semicolon in the end

    println!("message : {}", msg);

    'bob: loop {
        loop {
            loop {
                break 'bob; // break the bob loop, if you don't set the label inner loop will break
            }
        }
    }

    //   'test: loop {
    //       loop {
    //           continue 'test; /* continue test loop instead of inner, without label it continues inner loop */
    //       }
    //   }
    while dizzy(false) {
        // do stuff
        break;
    }
    // do while

    loop {
        //do stuff
        if dizzy(true) {
            break;
        }
    }

    for num in [1, 2, 3].iter() {
        println!("number : {}", num);
    }

    let arr = [(1, 2), (3, 4)];

    for (x, y) in arr.iter() {
        println!("x : {} - y : {}", x, y);
    }

    for n in 1..11 {
        println!("number : {}", n);
    }
}

fn dizzy(flag: bool) -> bool {
    return flag;
}
