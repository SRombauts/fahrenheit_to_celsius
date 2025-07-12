use std::io;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        let args_processed = process_command_line_args(args);
        if !args_processed {
            print_help();
            return;
        }
    } else {
        println!("No command line arguments provided.");
        loop {
            let done = interactive_prompt();
            if done {
                break;
            }
        }
    }
}

fn process_command_line_args(args: Vec<String>) -> bool {
    println!("{} command line arguments: process + {:?}", args.len() - 1, &args[1..]);

    if args[1] == "--help" || args[1] == "-h" {
        return print_help();
    }

    let farentheit: f32 = match args[1].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid number provided: {}", args[1]);
            return false;
        },
    };

    let celsius = fahrenheit_to_celsius(farentheit);
    println!("{}", celsius);

    true
}

fn interactive_prompt() -> bool {
    println!("Please enter a temperature in Fahrenheit to convert to Celsius:");
    let mut fahrenheit = String::new();
    io::stdin().read_line(&mut fahrenheit).expect("Failed to read line");

    let fahrenheit: f32 = match fahrenheit.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid number provided: {}", fahrenheit.trim());
            return false;
        },
    };

    let celsius = fahrenheit_to_celsius(fahrenheit);
    println!("{}", celsius);

    true
}

fn print_help() -> bool {
    println!("Usage: fahrenheit_to_celsius <temperature_i.0n_fahrenheit>");
    println!("Example: fahrenheit_to_celsius 100.0");
    true
}

fn fahrenheit_to_celsius(farentheit: f32) -> f32 {
    let celsius = (farentheit - 32.0) * 5.0 / 9.0;
    celsius
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_freezing_temperature() {
        assert_eq!(fahrenheit_to_celsius(32.0), 0.0);
    }

    #[test]
    fn test_boiling_temperature() {
        assert_eq!(fahrenheit_to_celsius(212.0), 100.0);
    }
}