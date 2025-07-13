use std::io;

fn main() {
    match std::env::args().nth(1) {
        Some(arg) => {
            match process_command_line_arg(arg) {
                Ok(_) => {},
                Err(e) => {
                    eprintln!("{}", e);
                    print_help();
                }
            }
        }
        None => {
            loop_interactive_prompt();
        },
    }
}

fn process_command_line_arg(arg: String) -> Result<(), String> {
    if arg == "--help" || arg == "-h" {
        return Err("Help requested. Use --help or -h to see usage.".to_string());
    }

    match arg.parse() {
        Ok(fahrenheit) => {
            let celsius = fahrenheit_to_celsius(fahrenheit);
            println!("{}", celsius);
            Ok(())
        }
        Err(_) => {
            Err(format!("Invalid number provided: {}", arg.trim()))
        }
    }
}

fn loop_interactive_prompt() {
    loop {
        match interactive_prompt() {
            Ok(_) => {
                break
            },
            Err(e) => {
                eprintln!("{}", e);
            },
        }
    }
}

fn interactive_prompt() -> Result<(), String> {
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
            return Err(format!("Invalid number provided: {}", input.trim()))
        }
    }
}

fn print_help() {
    println!("Usage: fahrenheit_to_celsius <temperature_i.0n_fahrenheit>");
    println!("Example: fahrenheit_to_celsius 100.0");
}

fn fahrenheit_to_celsius(fahrenheit: f32) -> f32 {
    (fahrenheit - 32.0) * 5.0 / 9.0
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