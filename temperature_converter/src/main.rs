use std::io;

fn main() {
    println!("Temperature Converter");

    loop {
        // Ask user for input
        let mut temperature: String = String::new();
        println!("Please enter the temperature value: ");
        io::stdin()
            .read_line(&mut temperature)
            .expect("Failed to read line.");

        // .parse() is "turbofish" compatible (-> many possible types)
        // Type annotation of variable (: f64) is where it infers type
        let temperature: f64 = match temperature.trim().parse() {
            Ok(value) => value,
            Err(_) => continue,
        };

        // Ask user which conversion formula to use
        let mut conversion: String = String::new();
        println!(
            "Enter the character for conversion: 
                \n F - convert to Fahrenheit
                \n C - convert to Celsius"
        );
        io::stdin()
            .read_line(&mut conversion)
            .expect("Failed to read line.");

        let conversion: char = match conversion.trim().to_lowercase().parse() {
            Ok(char) => char,
            Err(_) => continue,
        };

        // Match according to which character was given

        /*
        There are two approaches to 'match' in this scenario

        1. Direct Approach (don't save match result into a var)

        2. Expression Approach (all cases must provide the same value)
        */

        // Direct Approach
        // match conversion {
        //     'f' => {
        //         let result = to_fahrenheit(temperature);
        //         println!("The temperature in Fahrenheit is {result}F");
        //     }
        //     'c' => {
        //         let result = to_celsius(temperature);
        //         println!("The temperature in Celsius is {result}C");
        //     }
        //     _ => println!("Invalid character."),
        // }

        // Expression Approach
        let result: f64 = match conversion {
            'f' => to_fahrenheit(temperature),
            'c' => to_celsius(temperature),
            _ => 0.0, // All arms must return f64
        };

        println!("Result: {result}\n");
    }
}

fn to_celsius(fahrenheit: f64) -> f64 {
    // No 'return' or ';' makes this an expression as blocks can
    // evaluate to a value
    (fahrenheit - 32.0_f64) * (5.0_f64 / 9.0_f64)
}

fn to_fahrenheit(celsius: f64) -> f64 {
    // No 'return' or ';' makes this an expression as blocks can
    // evaluate to a value
    (celsius * (9.0_f64 / 5.0_f64)) + 32.0_f64
}
