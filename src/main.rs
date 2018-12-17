extern crate oss_advent_code_18;
// Will have menu control and call of different functions from the lib.rs
// need flexibility on calls

// Answer: Lets just make a big match
fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    println!("Reading file: {}", filename);

    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
}
