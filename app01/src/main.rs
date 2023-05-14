#![allow(dead_code)]

use std::collections::HashMap;
use std::fmt; // Import the `fmt` module.
use std::io;
use std::str::FromStr;
#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

// Define a structure named `List` containing a `Vec`.
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Extract the value using tuple indexing,
        // and create a reference to `vec`.
        let vec = &self.0;

        write!(f, "[")?;

        // Iterate over `v` in `vec` while enumerating the iteration
        // count in `count`.
        for (count, v) in vec.iter().enumerate() {
            // For every element except the first, add a comma.
            // Use the ? operator to return on errors.
            if count != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", v)?;
        }

        // Close the opened bracket and return a fmt::Result value.
        write!(f, "]")
    }
}

// Tuples can be used as function arguments and as return values.
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // `let` can be used to bind the members of a tuple to variables.
    let (int_param, bool_param) = pair;

    (bool_param, int_param)
}

// The following struct is for the activity.
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

// Create an `enum` to classify a web event. Note how both
// names and type information together specify the variant:
// `PageLoad != PageUnload` and `KeyPress(char) != Paste(String)`.
// Each is different and independent.
enum WebEvent {
    // An `enum` variant may either be `unit-like`,
    PageLoad,
    PageUnload,
    // like tuple structs,
    KeyPress(char),
    Paste(String),
    // or c-like structures.
    Click { x: i64, y: i64 },
}

// A function which takes a `WebEvent` enum as an argument and
// returns nothing.
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        // Destructure `c` from inside the `enum` variant.
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        // Destructure `Click` into `x` and `y`.
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        }
    }
}

struct Circle {
    radius: i32,
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle {}", self.radius)
    }
}

mod file;

#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("You are running linux!");
}

// And this function only gets compiled if the target OS is *not* linux
#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
    println!("You are *not* running linux!");
}

#[cfg(c_os = "linux")]
fn are_you_c_os() {
    println!("You are running c os linux!");
}

// And this function only gets compiled if the target OS is *not* linux
#[cfg(not(c_os = "linux"))]
fn are_you_c_os() {
    println!("You are *not* c os Linux!");
}

fn function() {
    println!("called `function()`");
    are_you_on_linux();
    println!("Are you sure?");
    if cfg!(target_os = "linux") {
        println!("Yes. It's definitely linux!");
    } else {
        println!("Yes. It's definitely *not* linux!");
    }
    // custom
    are_you_c_os();
}

fn do_mod() {
    file::index::function();
    file::indirect_access();
    file::function();
    function();
    file::inaccessible_fn();
}
fn do_HOF() {
    println!("Find the sum of all the squared odd numbers under 1000");
    let upper = 1000;

    // Imperative approach
    // Declare accumulator variable
    let mut acc = 0;

    fn is_odd(n: u32) -> bool {
        n % 2 == 1
    }

    // Functional approach
    let sum_of_squared_odd_numbers: u32 = (0..)
        .map(|n| n * n) // All natural numbers squared
        .take_while(|&n_squared| n_squared < upper) // Below upper limit
        .filter(|&n_squared| is_odd(n_squared)) // That are odd
        .sum(); // Sum them
    println!("functional style: {}", sum_of_squared_odd_numbers);

    fn sum_odd_numbers(up_to: u32) -> u32 {
        let mut acc = 0;
        for i in 0..up_to {
            // Notice that the return type of this match expression must be u32
            // because of the type of the "addition" variable.
            let addition: u32 = match i % 2 == 1 {
                // The "i" variable is of type u32, which is perfectly fine.
                true => i,
                // On the other hand, the "continue" expression does not return
                // u32, but it is still fine, because it never returns and therefore
                // does not violate the type requirements of the match expression.
                false => continue,
            };
            acc += addition;
        }
        acc
    }
    println!(
        "Sum of odd numbers up to 9 (excluding): {}",
        sum_odd_numbers(9)
    );

    do_mod();
}
fn do_output_fn() {
    fn create_fn() -> impl Fn() {
        let text = "Fn".to_owned();

        move || println!("This is a: {}", text)
    }

    fn create_fnmut() -> impl FnMut() {
        let text = "FnMut".to_owned();

        move || println!("This is a: {}", text)
    }

    fn create_fnonce() -> impl FnOnce() {
        let text = "FnOnce".to_owned();

        move || println!("This is a: {}", text)
    }

    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();
    let fn_once = create_fnonce();

    fn_plain();
    fn_mut();
    fn_once();
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // `iter()` for vecs yields `&i32`. Destructure to `i32`.
    println!("2 in vec1: {}", vec1.iter().any(|&x| x == 2));
    // `into_iter()` for vecs yields `i32`. No destructuring required.
    println!("2 in vec2: {}", vec2.into_iter().any(|x| x == 2));

    // `iter()` only borrows `vec1` and its elements, so they can be used again
    println!("vec1 len: {}", vec1.len());
    println!("First element of vec1 is: {}", vec1[0]);

    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // `iter()` for vecs yields `&i32`.
    let mut iter = vec1.iter();
    // `into_iter()` for vecs yields `i32`.
    let mut into_iter = vec2.into_iter();

    // `iter()` for vecs yields `&i32`, and we want to reference one of its
    // items, so we have to destructure `&&i32` to `i32`
    println!("Find 2 in vec1: {:?}", iter.find(|&&x| x == 2));
    // `into_iter()` for vecs yields `i32`, and we want to reference one of
    // its items, so we have to destructure `&i32` to `i32`
    println!("Find 2 in vec2: {:?}", into_iter.find(|&x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    // `iter()` for arrays yields `&i32`
    println!("Find 2 in array1: {:?}", array1.iter().find(|&&x| x == 2));
    // `into_iter()` for arrays yields `i32`
    // println!("Find 2 in array2: {:?}", array2.into_iter().find(|&x| x == 2));

    do_HOF()
}

fn input_param_closure() {
    // <F> denotes that F is a "Generic type parameter"
    fn apply<F>(f: F)
    where
        // The closure takes no input and returns nothing.
        F: FnOnce(),
    {
        // ^ TODO: Try changing this to `Fn` or `FnMut`.

        f();
    }

    // A function which takes a closure and returns an `i32`.
    fn apply_to_3<F>(f: F) -> i32
    where
        // The closure takes an `i32` and returns an `i32`.
        F: Fn(i32) -> i32,
    {
        f(3)
    }
    use std::mem;

    let greeting = "hello";
    // A non-copy type.
    // `to_owned` creates owned data from borrowed one
    let mut farewell = "goodbye".to_owned();

    // Capture 2 variables: `greeting` by reference and
    // `farewell` by value.
    let diary = || {
        // `greeting` is by reference: requires `Fn`.
        println!("I said {}.", greeting);

        // Mutation forces `farewell` to be captured by
        // mutable reference. Now requires `FnMut`.
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzzz");

        // Manually calling drop forces `farewell` to
        // be captured by value. Now requires `FnOnce`.
        mem::drop(farewell);
    };

    // Call the function which applies the closure.
    apply(diary);

    // `double` satisfies `apply_to_3`'s trait bound
    let double = |x| 2 * x;

    println!("3 doubled: {}", apply_to_3(double));

    do_output_fn();
}
fn do_closure() {
    use std::mem;

    let color = String::from("green");

    // A closure to print `color` which immediately borrows (`&`) `color` and
    // stores the borrow and closure in the `print` variable. It will remain
    // borrowed until `print` is used the last time.
    //
    // `println!` only requires arguments by immutable reference so it doesn't
    // impose anything more restrictive.
    let print = || println!("`color`: {}", color);

    // Call the closure using the borrow.
    print();

    // `color` can be borrowed immutably again, because the closure only holds
    // an immutable reference to `color`.
    let _reborrow = &color;
    print();

    // A move or reborrow is allowed after the final use of `print`
    let _color_moved = color;

    let mut count = 0;
    // A closure to increment `count` could take either `&mut count` or `count`
    // but `&mut count` is less restrictive so it takes that. Immediately
    // borrows `count`.
    //
    // A `mut` is required on `inc` because a `&mut` is stored inside. Thus,
    // calling the closure mutates the closure which requires a `mut`.
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    // Call the closure using a mutable borrow.
    inc();

    // The closure still mutably borrows `count` because it is called later.
    // An attempt to reborrow will lead to an error.
    inc();
    let _count_reborrowed = &mut count;

    // A non-copy type.
    let movable = Box::new(3);

    // `mem::drop` requires `T` so this must take by value. A copy type
    // would copy into the closure leaving the original untouched.
    // A non-copy must move and so `movable` immediately moves into
    // the closure.
    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };

    // `consume` consumes the variable so this can only be called once.
    consume();
    let _count_reborrowed = &mut count;

    // A non-copy type.
    let movable = Box::new(3);

    // `mem::drop` requires `T` so this must take by value. A copy type
    // would copy into the closure leaving the original untouched.
    // A non-copy must move and so `movable` immediately moves into
    // the closure.
    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };

    // `consume` consumes the variable so this can only be called once.
    consume();
    input_param_closure()
}

fn do_print() {
    println!("{} days", 31);

    // at 0 immediately after the format string.
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    // As can named arguments.
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );
    // Different formatting can be invoked by specifying the format character
    // after a `:`.
    println!("Base 10:               {}", 69420); // 69420
    println!("Base 2 (binary):       {:b}", 69420); // 10000111100101100
    println!("Base 8 (octal):        {:o}", 69420); // 207454
    println!("Base 16 (hexadecimal): {:x}", 69420); // 10f2c
    println!("Base 16 (hexadecimal): {:X}", 69420); // 10F2C

    // You can right-justify text with a specified width. This will
    // output "    1". (Four white spaces and a "1", for a total width of 5.)
    println!("{number:>5}", number = 1);

    // You can pad numbers with extra zeroes,
    // and left-adjust by flipping the sign. This will output "10000".
    println!("{number:0<5}", number = 1);

    // You can use named arguments in the format specifier by appending a `$`.
    println!("{number:0>width$}", number = 1, width = 5);

    // Rust even checks to make sure the correct number of arguments are used.
    println!("My name is {0}, {1} {0}", "Bond", "avc");
    // FIXME ^ Add the missing argument: "James"

    // Only types that implement fmt::Display can be formatted with `{}`. User-
    // defined types do not implement fmt::Display by default.

    #[allow(dead_code)] // disable `dead_code` which warn against unused module
    struct Structure(i32);
    // This will not compile because `Structure` does not implement
    // fmt::Display.
    //println!("This struct `{}` won't print...", Structure(3));
    // TODO ^ Try uncommenting this line

    // For Rust 1.58 and above, you can directly capture the argument from a
    // surrounding variable. Just like the above, this will output
    // "    1", 4 white spaces and a "1".
    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");

    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Pretty print
    println!("{:#?}", peter);

    let v = List(vec![10, 20, 30]);
    println!("{}", v);

    // Variables can be type annotated.
    let logical: bool = true;

    let a_float: f64 = 1.0; // Regular annotation
    let an_integer = 5i32; // Suffix annotation

    // Or a default will be used.
    let default_float = 3.0; // `f64`
    let default_integer = 7; // `i32`

    // A type can also be inferred from context.
    let mut inferred_type = 12; // Type i64 is inferred from another line.
    inferred_type = 4294967296i64;

    // A mutable variable's value can be changed.
    let mut mutable = 12; // Mutable `i32`
    mutable = 21;

    // Integer addition
    println!("1 + 2 = {}", 1u32 + 2);

    // Integer subtraction
    println!("1 - 2 = {}", 1i32 - 2);
    // TODO ^ Try changing `1i32` to `1u32` to see why the type is important

    // Scientific notation
    println!("1e4 is {}, -2.5e-3 is {}", 1e4, -2.5e-3);

    // Short-circuiting boolean logic
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // Use underscores to improve readability!
    println!("One million is written as {}", 1_000_000u32);

    // A tuple with a bunch of different types.
    let long_tuple = (
        1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true,
    );

    // Values can be extracted from the tuple using tuple indexing.
    println!("Long tuple first value: {}", long_tuple.0);
    println!("Long tuple second value: {}", long_tuple.1);

    // Tuples can be tuple members.
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    // Tuples are printable.
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    // But long Tuples (more than 12 elements) cannot be printed.
    //let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    //println!("Too long tuple: {:?}", too_long_tuple);
    // TODO ^ Uncomment the above 2 lines to see the compiler error

    let pair = (1, true);
    println!("Pair is {:?}", pair);

    println!("The reversed pair is {:?}", reverse(pair));

    // To create one element tuples, the comma is required to tell them apart
    // from a literal surrounded by parentheses.
    println!("One element tuple: {:?}", (5u32,));
    println!("Just an integer: {:?}", (5u32));

    // Tuples can be destructured to create bindings.
    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);

    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` creates an owned `String` from a string slice.
    let pasted = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click { x: 20, y: 80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    // 1000 already fits in a u16
    println!("1000 as a u16 is: {}", 1000 as u16);

    // 1000 - 256 - 256 - 256 = 232
    // Under the hood, the first 8 least significant bits (LSB) are kept,
    // while the rest towards the most significant bit (MSB) get truncated.
    // println!("1000 as a u8 is : {}", 1000 as u8);
    // -1 + 256 = 255
    println!("  -1 as a u8 is : {}", (-1i8) as u8);

    // For positive numbers, this is the same as the modulus
    println!("1000 mod 256 is : {}", 1000 % 256);

    // When casting to a signed type, the (bitwise) result is the same as
    // first casting to the corresponding unsigned type. If the most significant
    // bit of that value is 1, then the value is negative.

    // Unless it already fits, of course.
    println!(" 128 as a i16 is: {}", 128 as i16);

    // 128 as u8 -> 128, whose value in 8-bit two's complement representation is:
    // println!(" 128 as a i8 is : {}", 128 as i8);

    // repeating the example above
    // 1000 as u8 -> 232
    //println!("1000 as a u8 is : {}", 1000 as u8);
    // and the value of 232 in 8-bit two's complement representation is -24
    // the literal `232` does not fit into the type `i8` whose range is `-128..=127`
    // println!(" 232 as a i8 is : {}", 232 as i8);

    // Since Rust 1.45, the `as` keyword performs a *saturating cast*
    // when casting from float to int. If the floating point value exceeds
    // the upper bound or is less than the lower bound, the returned value
    // will be equal to the bound crossed.

    // 300.0 as u8 is 255
    println!(" 300.0 as u8 is : {}", 300.0_f32 as u8);
    // -100.0 as u8 is 0
    println!("-100.0 as u8 is : {}", -100.0_f32 as u8);
    // nan as u8 is 0
    println!("   nan as u8 is : {}", f32::NAN as u8);

    // This behavior incurs a small runtime cost and can be avoided
    // with unsafe methods, however the results might overflow and
    // return **unsound values**. Use these methods wisely:
    unsafe {
        // 300.0 as u8 is 44
        println!(" 300.0 as u8 is : {}", 300.0_f32.to_int_unchecked::<u8>());
        // -100.0 as u8 is 156
        println!(
            "-100.0 as u8 is : {}",
            (-100.0_f32).to_int_unchecked::<u8>()
        );
        // nan as u8 is 0
        println!("   nan as u8 is : {}", f32::NAN.to_int_unchecked::<u8>());
    }

    // Suffixed literals, their types are known at initialization
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    // Unsuffixed literals, their types depend on how they are used
    let i = 1;
    let f = 1.0;

    // `size_of_val` returns the size of a variable in bytes
    println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
    println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
    println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
    println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));

    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // This expression will be assigned to `y`
        x_cube + x_squared + x
    };

    let z = {
        // The semicolon suppresses this expression and `()` is assigned to `z`
        2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);

    let circle = Circle { radius: 6 };
    let circleStr = circle.to_string();
    println!(" circle radius: {}", circleStr);

    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!("Sum: {:?}", sum);

    // 1. iter_mut
    let mut names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }
    println!("names: {:?}", names);

    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    // println!("names: {:?}", names);
    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            // TODO ^ Try deleting the & and matching just "Ferris"
            _ => println!("Hello {}", name),
        }
    }

    println!("names: {:?}", names);

    let number = 13;
    // TODO ^ Try different values for `number`

    println!("Tell me about {}", number);
    match number {
        // Match a single value
        13 => println!("13!"),
        // Match several values
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        // TODO ^ Try adding 13 to the list of prime values
        // Match an inclusive range
        13..=19 => println!("A teen"),
        // Handle the rest of cases
        _ => println!("Ain't special"),
        // TODO ^ Try commenting out this catch-all arm
    }

    let number: u8 = 4;

    match number {
        i if i == 0 => println!("Zero"),
        i if i > 0 => println!("Greater than zero"),
        // _ => unreachable!("Should never happen."),
        // TODO ^ uncomment to fix compilation
        _ => {}
    }

    fn some_number() -> Option<u32> {
        Some(42)
    }
    match some_number() {
        // Got `Some` variant, match if its value, bound to `n`,
        // is equal to 42.
        Some(n @ 42) => println!("The Answer: {}!", n),
        // Match any other number.
        Some(n) => println!("Not interesting... {}", n),
        // Match anything else (`None` variant).
        _ => (),
    }

    // All have type `Option<i32>`
    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    // The `if let` construct reads: "if `let` destructures `number` into
    // `Some(i)`, evaluate the block (`{}`).
    if let Some(i) = number {
        println!("Matched {:?}!", i);
    }

    // If you need to specify a failure, use an else:
    if let Some(i) = letter {
        println!("Matched {:?}!", i);
    } else {
        // Destructure failed. Change to the failure case.
        println!("Didn't match a number. Let's go with a letter!");
    }

    // Provide an altered failing condition.
    let i_like_letters = false;

    if let Some(i) = emoticon {
        println!("Matched {:?}!", i);
        // Destructure failed. Evaluate an `else if` condition to see if the
        // alternate failure branch should be taken:
    } else if i_like_letters {
        println!("Didn't match a number. Let's go with a letter!");
    } else {
        // The condition evaluated false. This branch is the default:
        println!("I don't like letters. Let's go with an emoticon :)!");
    }

    fn get_count_item(s: &str) -> (u64, &str) {
        let mut it = s.split(' ');
        let (Some(count_str), Some(item)) = (it.next(), it.next()) else {
            panic!("Can't segment count item pair: '{s}'");
        };
        let Ok(count) = u64::from_str(count_str) else {
            panic!("Can't parse integer: '{count_str}'");
        };
        (count, item)
    }

    assert_eq!(get_count_item("3 chairs"), (3, "chairs"));
    // Make `optional` of type `Option<i32>`
    let mut optional = Some(0);

    // This reads: "while `let` destructures `optional` into
    // `Some(i)`, evaluate the block (`{}`). Else `break`.
    while let Some(i) = optional {
        if i > 2 {
            println!("Greater than 9, quit!");
            optional = None;
        } else {
            println!("`i` is `{:?}`. Try again.", i);
            optional = Some(i + 1);
        }
        // ^ Less rightward drift and doesn't require
        // explicitly handling the failing case.
    }
    // ^ `if let` had additional optional `else`/`else if`
    // clauses. `while let` does not have these.
    let outer_var = 42;

    // A regular function can't refer to variables in the enclosing environment
    //fn function(i: i32) -> i32 { i + outer_var }
    // TODO: uncomment the line above and see the compiler error. The compiler
    // suggests that we define a closure instead.

    // Closures are anonymous, here we are binding them to references
    // Annotation is identical to function annotation but is optional
    // as are the `{}` wrapping the body. These nameless functions
    // are assigned to appropriately named variables.
    let closure_annotated = |i: i32| -> i32 { i + outer_var };
    // let closure_inferred  = |i     |          i + outer_var  ;

    // Call the closures.
    println!("closure_annotated: {}", closure_annotated(1));
    // println!("closure_inferred: {}", closure_inferred(1));
    do_closure();
}

fn entry_point() {
    do_print();
    for _ in 0..10 {
        println!("uuid: {}", Uuid::new_v4().hyphenated().to_string().replace("-", ""));
    }
}

use std::io::{Error, ErrorKind};
use uuid::Uuid;
use warp::Filter;

#[tokio::main]
async fn main() {
    entry_point();

    let hello = warp::get().map(|| format!("Hello, World!"));
    warp::serve(hello).run(([127, 0, 0, 1], 3030)).await;
}
