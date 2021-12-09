use std::env;

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
    // Double backslashes are needed to make Rust not think they are an escape character. Only one backslash will show, as intended
    println!(" \\   ^__^");
    println!("  \\  (oo)\\_______");
    println!("     (__)\\       )\\/\\");
    println!("         ||----w |");
    println!("         ||     ||");
}