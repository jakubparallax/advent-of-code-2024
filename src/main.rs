use std::env;

mod one;
mod two;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    match args.get(1).unwrap().as_str() {
        "one" => one::solve(),
        "two" => two::solve(),
        _ => println!("Invalid argument"),
    }
}
