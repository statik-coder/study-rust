fn main() {
    //*
    // Tuples
    //*

    // There I create tuple of some values!
    //
    // the main difference between array
    // is that tuple can store values with different types
    let tuple: (&str, u8, char) = ("Some value", 65, '🌅');

    // There I destructure values from typle
    let (string, number, char) = tuple;

    // Also can access to values with dot (.)
    let str = tuple.0;
    let num = tuple.1;
    let character = tuple.2;

    println!("The string value is: {} \nThe number is: {} \nThe char is: {}!", string, number, char);

    //*
    // Arrays
    //*

    // Arrays must be only with fixed length and each element must be array common type
    let str_array: [&str; 4] = ["Array", "with", "4", "values"];

    println!("Third array value is: {}", str_array[3]);
}
