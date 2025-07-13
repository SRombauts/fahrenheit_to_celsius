use std::io;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        process_command_line_args(args);
    } else {
        loop_interractive_prompt();
    }
}

fn process_command_line_args(args: Vec<String>) {
    if args[1] == "--help" || args[1] == "-h" {
        print_help();
        return;
    }

    let fahrenheit: f32 = match args[1].parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Invalid number provided: {}", args[1]);
            print_help();
            return;
        },
    };

    let celsius = fahrenheit_to_celsius(fahrenheit);
    println!("{}", celsius);
}

fn loop_interractive_prompt() {
    loop {
        let done = interactive_prompt();
        if done {
            break;
        }
    }
}

fn interactive_prompt() -> bool {
    println!("Please enter a temperature in Fahrenheit to convert to Celsius:");
    let mut fahrenheit = String::new();
    io::stdin().read_line(&mut fahrenheit).expect("Failed to read line");

    let fahrenheit: f32 = match fahrenheit.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Invalid number provided: {}", fahrenheit.trim());
            return false;
        },
    };

    let celsius = fahrenheit_to_celsius(fahrenheit);
    println!("{}", celsius);

    true
}

fn print_help() {
    println!("Usage: fahrenheit_to_celsius <temperature_i.0n_fahrenheit>");
    println!("Example: fahrenheit_to_celsius 100.0");
}

fn fahrenheit_to_celsius(fahrenheit: f32) -> f32 {
    let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;
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
    fn test_body_temperature() {
        assert_eq!(fahrenheit_to_celsius(98.6), 37.0);
    }

    #[test]
    fn test_boiling_temperature() {
        assert_eq!(fahrenheit_to_celsius(212.0), 100.0);
    }
}