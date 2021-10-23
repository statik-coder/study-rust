fn separate(label: &str) {
    println!(" ");
    println!("=========####=========");
    println!("{}", label);
    println!("=========####=========");
    println!(" ");
}

fn main() {
    // Tuples
    separate("Tuples");

    // There I create tuple of some values!
    //
    // the main difference between array
    // is that tuple can store values with different types
    let tuple: (&str, u8, char) = ("Some value", 65, '🌅');

    // There I destructure values from tuple
    let (string, number, char) = tuple;

    // Also can access to values with dot (.)
    let str = tuple.0;
    let num = tuple.1;
    let character = tuple.2;

    println!(
        "The string value is: {} \nThe number is: {} \nThe char is: {}!",
        string, number, char
    );

    // Arrays
    separate("Arrays");

    // Arrays must be only with fixed length and each element must be array common type
    let str_array: [&str; 4] = ["Array", "with", "4", "values"];

    println!("Third array value is: {}", str_array[3]);

    // If-else statements
    let condition: bool = true;

    // Conditions must be a boolean type (not the same as in JS like statement number will be true)
    // Check an example below
    if condition {
        println!("The condition is true");
    } else {
        println!("The condition is false")
    }

    let some_number: i32 = 53;

    // There we need to compare some_number with 0 to get boolean type
    if some_number > 0 {
        println!("Number is bigger then 0")
    }
    // Of course RUST has if-else-if construction

    // Loops
    separate("Loops");

    // There 3 loop types: loop (infinite), for and while
    // Also you can assign values from loop to variable 'cause loop it's expression
    let mut count = 3;

    // Loop
    'loop_name: loop {
        if count == 0 {
            // In RUST may name loops and continue or break loops from other loops just by name
            break 'loop_name;
        }
        println!("Current count is {}", count);
        count -= 1;
    }

    let mut while_count = 3;

    // While
    while while_count > 0 {
        println!("Current count is {}", while_count);
        while_count -= 1;
    }

    // For
    let array: [i32; 11] = [0, 10, 20, 30, 40, 50, 60, 70, 80, 90, 100];

    for num in array {
        println!("- {}", num)
    }
    println!("LIFT OFF!!!")
}
