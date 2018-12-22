extern crate oss_advent_code_18;

use std::env;

// Will have menu control and call of different functions from the lib.rs
// need flexibility on calls
// Answer: Lets just make a big match
fn main() {
    let args: Vec<String> = env::args().collect();
    oss_advent_code_18::work_to_do(args);
}
