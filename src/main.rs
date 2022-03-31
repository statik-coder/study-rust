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

    // There I de structure values from tuple
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
    println!("LIFT OFF!!!");

    separate("Structs");

    // derive(Debug) - this some kind of prefix implement Debug method (pretty output)
    #[derive(Debug)]
    struct Player {
        name: String,
        username: String,
        age: u8,
        skill_level: String
    }

    impl Player {
        fn say_hi(&self) -> () {
            println!("User {} says: Hi!", self.username)
        }

        fn send_message(&self, receiver: &Player, message: &str) -> () {
            println!(
                "User {} sends message to {}: {}",
                self.username,
                receiver.username,
                message
            )
        }
    }

    let player1 = Player {
        name: String::from("Artem"),
        username: String::from("stat1k"),
        age: 19,
        skill_level: String::from("High++")
    };

    let player2 = Player {
        name: String::from("Oleg"),
        username: String::from("oleg-33"),
        age: 20,
        skill_level:String::from("High++")
    };

    player1.say_hi();
    player2.send_message(&player1, "HI, mate! What's going?");


    // To print objects in console we should in braces type "{:#?}"
    // use debug syntax
    println!("This is the first player: {:#?}", &player1);

    separate("Enums");

    #[derive(Debug)]
    enum IPVersion {
        V4,
        V6
    }

    struct IPAddress {
        version: IPVersion,
        value: String
    }

    let home_addr = IPAddress {
        version: IPVersion::V4,
        value: String::from("192.168.0.1")
    };

    println!("My home address is {} (version: {:?})", &home_addr.value, &home_addr.version);

    // Also possible to create instances with associated functions
    // That functions implements in types and calls with this syntax - TYPE::AF
    // For example - String::from()
    // Let's create own function in IPAddress struct
    //
    // AF - associated functions

    impl IPAddress {
        fn create_v4(value: &str) -> IPAddress {
            IPAddress {
                version: IPVersion::V4,
                value: String::from(value)
            }
        }
    }

    let my_v4_ip_address = IPAddress::create_v4("127.0.0.1");
    println!("My local ip v4 address is {}", my_v4_ip_address.value);

    // There is one more opportunity how to create struct instance from "enum creators"
    #[derive(Debug)]
    enum IpAddr {
        V4(String),
        V6(String)
    };

    let v4addr = IpAddr::V4(String::from("localhost"));
    println!("Device local host is - {:#?}", &v4addr);

    let some_num: Option<i32> = Some(5);

    match some_num {
        None => {
            println!("There is no value")
        }
        Some(num) => println!("The value is: {}", num)
    }

    // ============================================
    //
    // In The Rust book there was a big part of explaining of packages,
    // exports, modules, crates ans so on...
    //
    // ============================================

    separate("Standard library");

    // Vector
    let vector: Vec<i32> = Vec::new();

    // Also we can create vector with `vec!` macros
    let mut v = vec![1, 2, 3]; // It automatically

    // In vector we can, for example, push some values (but variable must be mutable)
    v.push(8);
    v.push(3);

    let s1 = String::from("hello,");
    let s2 = String::from("world!");

    let s3 = s1 + &s2;

    println!("{}", s3);
}
