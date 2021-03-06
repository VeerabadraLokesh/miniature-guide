fn  // keyword used to define a function
main() { // this is where everything starts, well almost everything
    // println! macro, macros expand into source code during compilation (seems like inline in c++)
    println!("Hello {}!!, how are you doing ?", "Durga");

    /*
    *
    * Looks like almost everyone likes these for multi line comments
    *
    */

    // data types (usual starting stuff for every programming language)
    // rust is statically typed, but the upside is that the compiler can automatically infer data types based on value assigned to it
    let name = "Durga";     // string
    let rating = 4.5;       // float
    let dumb = true;        // boolean
    let character = '♥';    // unicode character, we all need rust

    // crazy ternary operator heading your way 😂
    println!("The movie '{}' had a rating of {}, yet I think that the plot of movie is quite {}, its all revolves around {}", name, rating, if dumb {"Dumb"} else {"Awesome"}, character);
    // cool thing {}, have only used in high level programming languages like python and javascript till now

    // scalar types
    /*
        Integer
        Floating point
        booleans
        characters
    */
    
    /*
        Integer types = i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize
        interesting to see 128 bit integers
        isize and usize -> derived from the architecture of the machine, like 32 for x86 and 64 for x64 machines
        i32 seems to be the default like c++
    */
    // TODO hardcoding for the time being
    let age:u32 = 25;
    let size = 283036;
    let weight:usize = 49;
    let head_weight:u128 = u128::pow(2, 49);   // giving overflow for 10^49, lets go binary

    println!("well did't find any other interesting function to play with the variables, so... {},{},{},{}", age, size, weight, head_weight);


    /*
        Lets Float
        floats are pretty boring, there are just 2 types f32 and f64
        f64 is the default
    */
    let depth = 6.5;    // f64
    let diameter:f32 = 1.0; //f32

    println!("rust compiler warning during compilation, got to print them {}, {}", depth, diameter);

    println!("wait seems there's a way to disable compile time warnings for unused variables, start var name with _");
    
    let _a = "test";

    // automatic type casting is not allowed in rust
    // let eyesight:f32 = 2; // throws a mismatched types error
    let eyesight:f32 = 2.;
    println!("eye sight is {}", eyesight);

    // _ as number seperator, you might have seen ' as number seperator in c++, here rust comes
    let _int_with_seperator = 2_121_995;
    let _float_with_seperator = 1_995.12_02;

    // Booleans
    let isfun:bool = true;
    println!("is rust interesting ? {}", isfun);

    // Characters
    /*
        supports numbers, alphabets, unicodes and special characters (emojis);
        char type represents Unicoe scalar [U+0000 - U+D7FF] and [U+E000 - U+10FFFF]
    */
    let email_character = '@';
    let alpha:char = 'D';
    let emoji:char = '🤪';

    println!("heck with println {}{}{}", alpha, email_character, emoji);

    // Variables
    /*
        the usual stuff, name can be of letter, digits and underscores, but can't start with numbers; and case sensitive
        although data type is optional
        by default variables are immutable (const in c++ etc)
        some people claim that this adds to safety and easy concurrency, let see
    */
    let _radius = 10.0;
    // _radius = 8.0; well, a compile time error
    
    // let's declare a mutable variable whose value can be changed
    let mut distance = 30; // in centimeter
    println!("distance between poles/nodes is {}, poles have very strong attraction", distance);

    distance = 25; // re assigning (after print, because throwing unused assignments warning), as i'm not sure if it will be 30

    // println!(distance); cant do this ? WTH
    println!("{}, intersting thing is these poles are held together by a cloth, so they came closer even though the natural tendency is to move away", distance);

    // Constants
    /*
        here they come, i don't see a reason for them since all variables are immutable by default
        naming conventions: (var conventions, ...use uppercae) // waiting for the spread operator LoL
        use const key word instead of let
        and data type specification seems to be must with const type, not so flexible like variables with let
        constant can only be set to a constant expression, cannot be assigned to function calls or any other compute time/runtime values;
        and constants can be declared in any scope
    */
    const PI:f32 = 3.14;
    const SPEED_OF_LIGHT:u32 = 299792458;
    
    println!("speed of light is {} m/s", SPEED_OF_LIGHT);
    println!("pi: {}", PI);

    // Shadowing of variables (fancy name for declaring the variable again using the let keyword)
    let ego = "1.0"; // irritating warnings
    print!("some people have {}% ego; ", ego);
    let ego = 100.0;
    println!("She has {}% ego", ego);

    // Constants cannot be shadowed, since they can't be changed like ever
    
    // Lets talk about String Theory
    /*
        String Literal (&str) - use if value is known at compile time, hardcoded into variable; std::str part of core language
                                static by default, so will be available throughout the program execution
        String Object (String) - provided in standard library pub struct string. String is growable collection
                                 mutable and UTF-8 encoded. useful for runtime string values (heap)
    */
    let country:&str = "India";
    print!("lets visit {} ", country);
    let city:&'static str = "Gandhinagar";
    println!("and see the nice city of {}", city);

    let mut new_string = String::new();
    let crazy_string = String::from("Crazy Times");
    println!("{} {}, {} {}", new_string, crazy_string, new_string.len(), crazy_string.len());
    
    new_string.push_str("hello");
    println!("{}", new_string);

    new_string = "hello dd hello".to_string();
    println!("{}", new_string);

    let mut other_string = new_string.replace("hello", "fyou");
    println!("{}", other_string);

    // as_str converts from String to string literal it seems
    print_string(other_string.as_str());

    print_type_of(&other_string);
    print_type_of(&other_string.as_str());

    other_string.push(' ');
    other_string.push('🖕');
    other_string.push(' ');
    print_string(other_string.as_str());

    println!("{}", other_string.len());
    println!("{}", other_string.trim().len());

    let mut i = 1;
    for token in other_string.split_whitespace() {
        println!("token {} {}", i, token);
        i += 1;
    }

    for token in other_string.split("f") {
        println!("token is {}", token);
    }
    let tokens:Vec<&str> = other_string.split("f").collect();
    println!("{}", tokens[0]);
    for token in tokens {
        println!("{}", token);
    }
    // println!("{}", tokens[0]); // TODO check later: gives compile time error because iterator has already made it to the end

    for c in other_string.chars() {
        print!("{} ", c);
    }
    println!();

    print_type_of(&other_string.to_string());

    // string concatination => first string + reference to second string
    // only on String objects, doesn't work on string literals &str
    let s1 = "fyou ".to_string();
    let s2 = "dd".to_string();
    let s4 = format!("{}{}", s1, s2);
    let s3 = s1 + &s2;
    // let s4 = format!("{}{}", s1, s2); // borrow of moved value: `s1` compile time error
    // let s4 = format!("{}{}", s1, s2);
    println!("{}\n{}", s3, s4);

    // type casting
    let number = 1995;
    let number_as_string = number.to_string();
    println!("{} is string ? {}", number_as_string, number_as_string=="1995");

    // Operators
    /*
        Source: https://doc.rust-lang.org/book/appendix-02-operators.html
        Arithmetic: +, -, *, /, % (++ and -- are not supported)
        Relational/comparision: >, <, >=, <=, ==, !=
        Logical operators: &&, ||, !
        Bitwise operators: &, |, ^, !, << (sift left), >> (shift left), >>> (right shift with zero)
    */
    let mut a = 4;
    let b = a << 2/16;
    let c = a >> 3;
    a >>= 3;
    println!("{}, {}, {}", b, c, a);

    // Conditional
    if a > 0 {

    } else if a > -1 {

    } else {

    }

    let st = "test";
    let d = match st {  // seems ; at end of expression throws an error
        "test" => {
            println!("a is 0"); "10"
        },
        "prod" => "20",
        _ => "30"
    };
    println!("{}", d);

    for i in 1..10 {
        if i % 2 == 0 {
            continue;
        }
        println!("{}", i);
    }
    let mut x = 0;
    while x < 3 {
        println!("{}", x);
        x += 1;
    }

    x = 0;
    loop {
        x += 1;
        println!("{}", x);
        if x==3 {
            break;
        }
    }

    fn test() {
        println!("test function inside main");
    }

    test();

    /*
        Function naming rules are same as that of variables
        
        main is the caller function, which is invoking the test function

        returning functions return a value
        fn mars_launcher() -> launch_code {
            return launch_code;
        }
        shorthand syntax without return statement (as seen in match)
        fn asteroid_explorer() -> resources_found {
            resources_found // no semicolon mean this value is returned
        }
        
        function parameters - pass by value or pass by reference
    */
    let mut test_string = String::from("world");
    test_change_string(&mut test_string);
    println!("{}", test_string);

    let name:String = String::from("Durga");
    display_string(name);
    // value borrowed here after move; error[E0382]: borrow of moved value: `name`
    // println!("{}", name); // cannot access variable after moving to other function

    let num:i32 = 32;
    display_number(num);
    println!("{}", num); // number can still be accessed, confusing

    // Tuples
    /*
        tuples have a fixed length;
        let tuple_name:(data_type1, data_type2) = (value1, value2);
        let tuple_name = (value1, value2);
    */
    let tuple:(i32,f64) = (-10,3.14);
    println!("{:?}", tuple);
    println!("integer: {:?}", tuple.0);
    println!("float: {:?}", tuple.1);
    
    let t = (1,true,3.14);
    print_tuple(t);

    let person:(&str, i8, f32) = ("Durga", 25, 6.9);
    print_person(person);

    // Arrays
    /*
        sequential memory blocks, static and cannot be resized.
        and elements cannot be deleted only updated or modified
        let array = [value1];
        let array:[datatype;size] = [value1,value2];
        let array:[datatype;size] = [default_value, size];
    */
    let arr:[i32; 4] = [10;4];
    println!("{:?}, size: {}", arr, arr.len());
    let arr = [10];
    println!("{:?}", arr);
    let arr:[&str;2] = ["String", "Array"];
    println!("{:?}", arr);
    
    for index in 0..arr.len() {
        println!("arr {} value is '{}'", index, arr[index]);
    }
    for val in arr.iter() {
        println!("{}", val);
    }
    // arr[1] = "Tuple"; // cannot assign

    let mut arr:[i32;4] = [10, 20, 30, 50];
    arr[3] = 40;

    set_array_values(&mut arr);
    println!("{:?}", arr);

    let _narr: usize = 10;
    // let arr = [0; _narr]; // non-constant value error; length must be known at compile time

    const N: usize = 10;
    let arr = [0; N]; // allowed
    println!("{:?}", arr);

    // Memory
    /*
        There can only be one owner at a time, same memory location can not be owned by two variables
        let k = 10; // here k is the owner of 10; two variables cannot point to the same location
        ownership can only be transferred by assigning value of old variable to new variable, or passing or returning from function

        stack -> if size is known at compile time
        heap -> if size is not known at compile time

        object ownership changes when its assigned to new variable, so the variable cannot be used after moving the ownership to the new variable
        but if its a primitive time, the value is just copied to the new variable instead of chaning the ownership, so they can be used even after assigning the value to the new variable
    */
    let v = vec![1, 2, 3];
    let v2 = v;
    // println!("{:?}", v); // will throw compilation error
    println!("{:?}", v2);
    
    let v_return = return_vector(v2);
    println!("{:?}", v_return);
    
    /*
        Borrowing ownership => temporarily transferring the ownership
    */
    let v = vec![1, 2, 3];
    print_vector(v);
    // println!("{:?}", v);// gives an error
    let v = vec![1, 2, 3];
    print_vector_borrowed(&v);
    println!("{:?}", v);

    let mut i = 2;
    double_int(&mut i);
    println!("{}", i);

    let mut name:String = String::from("Crazy");
    change_string(&mut name);
    println!("the name is: {}", name);
    
    
}

fn change_string(name: &mut String) {
    name.push_str(" Kid");
    // *name = String::from("ASDF");
}

fn double_int(i: &mut i32) {
    *i *= 2;
}

fn print_vector_borrowed(v: &Vec<i32>) {
    println!("{:?}", v);
}
fn print_vector(v: Vec<i32>) {
    println!("{:?}", v);
}

fn return_vector(v:Vec<i32>) -> Vec<i32> {
    println!("{:?}", v);
    return v;
}

fn set_array_values(arr:&mut [i32]) {
    // let x = arr.len() as i32;
    // print_type_of(&x);
    for i in 1..arr.len()+1 {
        // print_type_of(&i);
        arr[i-1] = (i * i) as i32 ;
    }
}

fn print_person(x:(&str,i8,f32)) {
    // Destructing a tuple
    let (name, age, rating) = x;
    println!("{} {} {}", name, age, rating);
}

fn print_tuple(x:(i32,bool,f64)) {
    println!("{:?}", x);
}

fn display_string(param_name:String) {
    println!("given name is {}", param_name);
}
fn display_number(num:i32) {
    println!("given number is {}", num);
}

fn test_change_string(test_string:&mut String) {
    *test_string = String::from("hello"); // de reference
}

// source: https://stackoverflow.com/questions/21747136/how-do-i-print-the-type-of-a-variable-in-rust
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn print_string(data:&str) {
    println!("{}", data);
}

// fn print_string(data:&String) {
//     println!("{}", data);
// }
