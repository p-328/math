use std::env;
use std::process;
mod parse_func;
fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        return Err(String::from("Not enough arguments. Note: If you're looking for help, just enter '--help ' and any random number, and same with '--version'"));
    }
    let num = parse_func::parse_float(args[2].to_string());
    match args[1].as_str() {
        "-ln" => println!("{}", num.unwrap().ln()),
        "-l" => {
            if args.len() < 4 {
                return Err("Less than 3 arguments not by the name of math supplied".to_string());
            } else {
                let second_num = parse_func::parse_float(args[3].to_string());
                let log = second_num.unwrap().ln() / num.unwrap().ln();
                println!("{}", log);
            }
        }
        "-lx" => println!("{}", num.unwrap().log10()),
        "-pi" => {
            if args.len() < 4 {
                return Err("Less than 3 arguments not by the name of math supplied".to_string());
            } else {

                let second_num = parse_func::parse_int(args[3].to_string());
                println!("{}", num.unwrap().powi(second_num.unwrap()));
            }
        }
        "-pf" => {
            if args.len() < 4 {
                return Err("Less than 3 arguments not by the name of math supplied".to_string());
            } else {
                let second_num = parse_func::parse_float(args[3].to_string());

                println!("{}", num.unwrap().powf(second_num.unwrap()));
            }
        }
        "--version" => {
            const VERSION: &str = env!("CARGO_PKG_VERSION");
            println!("{}", VERSION);
        }
        "--help" => {
            println!("Commands: ");
            println!("-ln - natural log(1 number required)");
            println!("-l - log base first number of the second number passed in.(2 numbers required)");
            println!("-lx - log base 10 of your first number passed in. (1 number required)");
            println!("-pi - raises your first number passed in to the second number passed in as an integer.");
            println!("-pf - raises your first number passed in to the second number passed in as a floating decimal.");
            println!("--version - prints the version of this command-line tool");
        }
        _ => {
            println!("Error: invalid arg");
            process::exit(-1);
        }
    }
    Ok(())
}
