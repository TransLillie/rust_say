use std::env;

fn main() {
    // Get input
    let args: Vec<String> = env::args().collect();
    // Check if there is no input, and if there is not, exit
    let _message_argument = match args.get(1) {
        Some(msg_arg) => msg_arg,
        None => {
          println!("No input!");
          std::process::exit(-1);
        }
    };
    // Assign input to var message
    let message = &args[1];
    // Get the length of message and repeat an underscore for each character
    let borders = "_".repeat(message.len());
    
    // Print the top border
    println!("{whitespace}{print_border}{whitespace2}",
             whitespace=" _",
             print_border=borders,
             whitespace2="_");
    // Print the input and sides of the border
    println!("{left_side} {input} {right_side}",
             left_side="<",
             input=message,
             right_side=">");
    // Print the bottom border
    println!("{whitespace}{print_border}{whitespace2}",
             whitespace=" _",
             print_border=borders,
             whitespace2="_");
    // (poorly) print the cow. Probably should use newlines instead but they acted weird
    // Double backslashes are needed to make Rust not think they are an escape character. Only one backslash will show, as intended
    println!("        \\   ^__^");
    println!("         \\  (oo)\\_______");
    println!("            (__)\\       )\\/\\");
    println!("                ||----w |");
    println!("                ||     ||");
}