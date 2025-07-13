use std::io;

fn main() {
    match std::env::args().nth(1) {
        Some(arg) => {
            process_command_line_arg(arg);
        }
        None => {
            loop_interractive_prompt();
        },
    }
}

fn process_command_line_arg(arg: String) {
    if arg == "--help" || arg == "-h" {
        print_help();
        return;
    }

    let fahrenheit: f32 = match arg.parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Invalid number provided: {}", arg);
            print_help();
            return;
        },
    };

    let celsius = fahrenheit_to_celsius(fahrenheit);
    println!("{}", celsius);
}

fn loop_interractive_prompt() {
    loop {
        match interactive_prompt() {
            Ok(_) => {},
            Err(_) => {
                eprintln!("An error occurred. Please try again.");
            },
        }
    }
}

fn interactive_prompt() -> Result<(), ()> {
    println!("Please enter a temperature in Fahrenheit to convert to Celsius:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    match input.trim().parse() {
        Ok(fahrenheit) => {
            let celsius = fahrenheit_to_celsius(fahrenheit);
            println!("{}", celsius);
            Ok(())
        },
        Err(_) => {
            eprintln!("Invalid number provided: {}", input.trim());
            return Err(());
        }
    }
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