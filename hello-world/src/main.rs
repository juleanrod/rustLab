fn main() {
    println!("Hello, world!");
    let x = 5; //all variables are inmutable in rust, if we want them to be mutable
    println!("The value of the inmutable variable 'x' is: {}", x);
    let mut y = 20;
    println!("The value of the mutable variable 'y' is: {}", y);
    y = 1;
    println!("The value after mutating variable 'y' is: {}", y);

    //specifying tpye of a variable
    let n: i32 = 5; //the type here is a signed 32 bit integer

    //boolean
    let _boolean: bool = true;

    //tupples
    let a = (n, "Hello, world", _boolean);
    println!(
        "The first two values of the tupple are '{}' and '{}'",
        a.0, a.1
    );

    //arrays => are sets of the same type of values.
    let arr = [1, 2, 3, 4, 5];
    let arr_v2 = [1; 10]; //an array where theres 10 values equaling to 1;
    let arr_type: [i32; 10] = [0; 10]; //in this arry we specify the type and the length frist '[i32; 10]'
                                       // an is initialized after the '=' operator and we give it a value of 0 10 times '[0; 10]'

    println!("The value at index [0] is: {}", arr[0]);
    println!("The array is: {:?}", arr_v2);
    println!("The specified array w type i32 is equal to: {:?}", arr_type);
    // CONTROL FLOW
    // If-else-if ladder

    //if 'num' is 1, print you won!
    //else if 'num' is 2, print try again
    //else print you lots.

    let num = 1;

    if num == 1 {
        println!("You won!");
    } else if num == 2 {
        println!("Try again");
    } else {
        println!("You lost.")
    }

    //LOOPS
    loop {
        y = y * 2; //using 'y' that we defined earlier since is mutable;
        if y > 100 {
            break;
        }
        println!("The value is now {}", y);
    }

    println!("You are now outside of the loop");

    //rust has while loops.. same as other languages

    //for loops.. they are a little different from other languages

    for j in 0..10 {
        //not inclusive!
        println!("The value of 'y' is: {}", j);
    }

    //what if you wanted to write something more explicit?
    // for t in 0..=9 { //this is the inclusive version
    // println!("The value of 't' is: {}", t);
    // }

    //for loops can be run with iterators
    for value in arr {
        println!(
            "The values in the 'arr' at index {}: {}",
            arr.iter().position(|&r| r == value).unwrap(),
            value
        );
    }

    // MATCH STATEMENTS
    // is a mix of switch and if statements on otehr languages

    match x {
        1 => println!("Value of x is 1"),
        2 => println!("Value of x is 2"),
        _ => println!("Value of x is invalid"), //default equivalent in rust is '_'
    }

    //exhaustive matches
}
