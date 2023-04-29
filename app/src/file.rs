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

