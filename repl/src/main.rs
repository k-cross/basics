use std::io::Write;
fn main() {
    let mut input = String::new();

    loop {
        print!("> ");
        std::io::stdout().flush().ok();

        match std::io::stdin().read_line(&mut input) {
            Ok(0) => break,
            Ok(_) => {
                if eval(&input) == 0 {
                    break;
                }
                input.clear();
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                break;
            }
        }
    }
}

fn eval(input: &str) -> u8 {
    match input.trim() {
        "quit" => 0,
        x => {
            println!("You entered: {}", x);
            1
        }
    }
}
