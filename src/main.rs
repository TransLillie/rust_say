use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let _message_argument = match args.get(1) {
        Some(msg_arg) => msg_arg,
        None => {
          println!("No input! Exiting");
          std::process::exit(-1);
        }
    };
    let message = &args[1];
    println!("{}", message);
}