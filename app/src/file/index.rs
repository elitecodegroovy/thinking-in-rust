#[allow(dead_code)]
pub fn function() {
    println!("called `file::index::function()`");
}

#[allow(dead_code)]
fn file() {
    println!("called `file::index::private_function()`");
}