use std::io;

fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("--help") | Some("-h") => print_help(),
        Some(arg) => {
            if let Err(e) = process_command_line_arg(arg) {
                eprintln!("{e}");
                print_help();
            }
        }
        None => loop_interactive_prompt()
    }
}

fn process_command_line_arg(arg: &str) -> Result<(), String> {
    match arg.trim().parse::<f32>() {
        Ok(fahrenheit) => {
            println!("{}", fahrenheit_to_celsius(fahrenheit));
            Ok(())
        }
        Err(_) => Err(format!("Invalid argument: {}", arg.trim()))
    }
}

fn loop_interactive_prompt() {
    loop {
        match interactive_prompt() {
            Ok(_) => break,
            Err(e) => eprintln!("{e}")
        }
    }
}

fn interactive_prompt() -> Result<(), String> {
    println!("Please enter a temperature in Fahrenheit to convert to Celsius:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    match input.trim().parse::<f32>() {
        Ok(fahrenheit) => {
            println!("{}", fahrenheit_to_celsius(fahrenheit));
            Ok(())
        },
        Err(_) => {
            return Err(format!("Invalid argument: {}", input.trim()))
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
    fn test_process_command_line_arg_invalid_number() {
        let result = process_command_line_arg("abc");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid argument: abc");
    }

    #[test]
    fn test_process_command_line_arg_valid_number() {
        let result = process_command_line_arg("212.0");
        assert!(result.is_ok());
    }

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