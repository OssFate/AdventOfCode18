use std::fs::File;
use std::io::{Read};

mod day;

pub fn work_to_do(args: Vec<String>) {
    let (command, file) =  get_args(args);

    let com = match command {
        UserInput::Command(x) => x,
        _ => String::from("None")
    };
    println!("Running Command: {}", com);

    let file_content = read_file(file);

    match com.as_ref() {
        "one" => day::one::do_work(file_content),
        "two" => day::two::do_work(file_content),
        "three" => day::three::do_work(file_content),
        _ => println!("None option")
    }

}

fn read_file(path: UserInput) -> Option<String> {

    let filename = match path {
        UserInput::File(input) => input,
        _  => {
                println!("Not a file!");
                return None;
            }
    };

    println!("Reading file: {}", filename);
    println!("");
    
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    
    Some(contents)
}

fn get_args(args: Vec<String>) -> (UserInput, UserInput) {
    let command = UserInput::Command(args[1].clone());
    let file = UserInput::File(args[2].clone());
    // println!("{:?}", args);

    (command, file)    
}

enum UserInput {
    Command(String),
    File(String)
}
