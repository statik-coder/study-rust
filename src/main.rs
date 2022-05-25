use std::{
    arch::asm,
    collections::{hash_map::Entry, HashMap},
    fmt::Result,
};

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
        skill_level: String,
    }

    impl Player {
        fn say_hi(&self) -> () {
            println!("User {} says: Hi!", self.username)
        }

        fn send_message(&self, receiver: &Player, message: &str) -> () {
            println!(
                "User {} sends message to {}: {}",
                self.username, receiver.username, message
            )
        }
    }

    let player1 = Player {
        name: String::from("Artem"),
        username: String::from("stat1k"),
        age: 19,
        skill_level: String::from("High++"),
    };

    let player2 = Player {
        name: String::from("Oleg"),
        username: String::from("oleg-33"),
        age: 20,
        skill_level: String::from("High++"),
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
        V6,
    }

    struct IPAddress {
        version: IPVersion,
        value: String,
    }

    let home_addr = IPAddress {
        version: IPVersion::V4,
        value: String::from("192.168.0.1"),
    };

    println!(
        "My home address is {} (version: {:?})",
        &home_addr.value, &home_addr.version
    );

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
                value: String::from(value),
            }
        }
    }

    let my_v4_ip_address = IPAddress::create_v4("127.0.0.1");
    println!("My local ip v4 address is {}", my_v4_ip_address.value);

    // There is one more opportunity how to create struct instance from "enum creators"
    #[derive(Debug)]
    enum IpAddr {
        V4(String),
        V6(String),
    };

    let v4addr = IpAddr::V4(String::from("localhost"));
    println!("Device local host is - {:#?}", &v4addr);

    let some_num: Option<i32> = Some(5);

    match some_num {
        None => {
            println!("There is no value")
        }
        Some(num) => println!("The value is: {}", num),
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

    // Hash maps
    separate("Hash maps");

    let mut scores: HashMap<String, i32> = HashMap::new();

    let blue_team_name = String::from("Eagles");
    let yellow_team_name = String::from("Bears");

    scores.insert(blue_team_name, 50);
    scores.insert(yellow_team_name, 20);

    match scores.get(&String::from("Eagles")) {
        Some(val) => println!("The 'Eagles' team has {} points.", val),
        None => println!(";( not found"),
    }

    // Exercises
    let mut num_vector: Vec<usize> = vec![
        7, 2, 43, 7, 3, 2, 6, 8, 3, 3, 7, 8, 9, 5, 2, 4, 6, 6, 123, 321, 213, 456, 456, 457, 324,
        23, 65, 45, 678, 5, 6345, 34, 534, 5, 375675, 67865, 9678, 65654435, 43, 5, 34, 5, 7643,
        6575, 7, 7, 5, 5, 5, 5, 5, 5, 5, 5, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
        2, 2, 2, 2, 2, 2,
    ];
    num_vector.sort();
    let num_vector_length: usize = num_vector.len();

    let mut mediane: usize = 0;

    let middle_value: usize = num_vector_length / 2;

    if num_vector_length % 2 != 0 {
        match num_vector.get(middle_value + 1) {
            Some(right_val) => match num_vector.get(middle_value) {
                Some(left_value) => mediane = (left_value + right_val) / 2,
                None => println!("Can't find left value!"),
            },
            None => {
                println!("Can't find right value!")
            }
        }
    } else {
        match num_vector.get(middle_value) {
            Some(val) => mediane = val.to_owned(),
            None => println!("Can't find value!"),
        }
    }

    let mut moda: usize = 0;
    let mut moda_count: usize = 0;

    let mut value_counts: HashMap<usize, usize> = HashMap::new();

    for value in num_vector {
        let possible_value = value_counts.entry(value).or_insert(0);
        *possible_value += 1;
    }

    let values: Vec<usize> = value_counts.values().cloned().collect();
    let keys: Vec<usize> = value_counts.keys().cloned().collect();

    match values.iter().max() {
        Some(max_value_times_repeated) => {
            moda_count = max_value_times_repeated.to_owned();

            for key in keys {
                match value_counts.get(&key) {
                    Some(value) => {
                        if value == max_value_times_repeated {
                            moda = key
                        }
                    }
                    None => println!("There no such value!"),
                }
            }
        }
        None => println!("No max value in the vector!"),
    };

    println!(
        "The moda is: {} and it's value is repeated {} times!",
        moda, moda_count
    );
    println!("The mediane is: {}", mediane);

    separate("Traits, lifetimes, etc");

    struct Location<T> {
        longitude: T,
        latitude: T,
    }

    impl<T> Location<T> {
        fn get_longitude(&self) -> &T {
            return &self.longitude;
        }

        fn get_latitude(&self) -> &T {
            return &self.latitude;
        }
    }

    let london_city = Location {
        latitude: 12.121231231,
        longitude: 180.12121233,
    };

    println!(
        "The London center is on point: {}, {}",
        london_city.get_longitude(),
        london_city.get_latitude()
    );

    struct Point<T> {
        x: T,
        y: T,
    }

    pub trait Summary {
        fn summerize(&self) -> String;
    }

    impl Summary for Point<i32> {
        fn summerize(&self) -> String {
            return format!("{}, {}", self.x, self.y);
        }
    }

    impl Summary for Location<String> {
        fn summerize(&self) -> String {
            return format!("{}, {}", self.latitude, self.longitude);
        }
    }

    let point1 = Point { x: 12, y: 180 };

    println!("{}", point1.summerize());

    separate("Links lifetimes");

    {
        // let r: i32;
        {
            // let x = 5;
            // I can't pass the "x" reference to the "r" varible, because x not be valid on 375 line and will be droped on 372 line (end of scope)
            // r = &x
        }
        // println!("r: {}", r)
    }

    fn longest<'a>(a: &'a String, b: &'a String) -> &'a String {
        if a.len() > b.len() {
            return a;
        } else {
            return b;
        }
    }

    let string1 = String::from("long string");
    let string2 = String::from("short");

    println!("The longest string is: {}", longest(&string1, &string2));

    separate("Closures");

    // Own implemented cacher (patter: memoization or lazy evaluation)
    struct Cacher<T, K, V>
    where
        T: Fn(&K) -> V,
        K: std::cmp::Eq + std::hash::Hash,
    {
        _worker: T,
        _values: HashMap<K, V>,
    }

    impl<T, K, V> Cacher<T, K, V>
    where
        T: Fn(&K) -> V,
        K: std::cmp::Eq + std::hash::Hash,
    {
        pub fn new(worker: T) -> Cacher<T, K, V> {
            Cacher {
                _worker: worker,
                _values: HashMap::new(),
            }
        }

        pub fn value(&mut self, arg: K) -> &V {
            match self._values.entry(arg) {
                Entry::Occupied(entry) => entry.into_mut(),
                Entry::Vacant(entry) => {
                    let result = (self._worker)(entry.key());
                    entry.insert(result)
                }
            }
        }
    }
}

// Check this code for clues: https://paste.gg/p/anonymous/66782f48cca543ff932b099809f78c46
