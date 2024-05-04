mod inaccessible;
pub mod index;

pub fn function() {
    println!("called `file::function()`");
}

fn private_function() {
    println!("called `file::private_function()`");
}

pub fn indirect_access() {
    print!("called `file::indirect_access()`, that\n> ");
    private_function();
}

pub fn inaccessible_fn() {
    inaccessible::public_function()
}

fn divide(dividend: i32, divisor: i32) -> Result<i32, String> {
    if divisor == 0 {
        Err(format! ("Cannot divide {} by zero.", dividend))
    } else {
        Ok(dividend / divisor)
    }
}
#[test]
fn test_divide_by_zero() {
    let result = divide(10, 0);
    assert!(result.is_err()); // Ensure it’s an error
    assert_eq!(result.unwrap_err(), "Cannot divide 10 by zero.");
// Check the error message
}

