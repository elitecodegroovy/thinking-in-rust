
mod utils;

pub fn print_info() {
    println!("print_info!");
}

pub fn filter_str() {
    utils::do_skip();
}

pub use utils::print_msg;