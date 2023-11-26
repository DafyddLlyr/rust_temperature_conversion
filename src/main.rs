use std::io;

enum Scales {
    Fahrenheit,
    Celsius,
}

fn main() {
    println!("Let's convert temperatures!");
    
    let mut input_scale = String::new();
    let scale: Scales;
    
    loop {
        println!("Select a scale. Please type 'fahrenheit' or 'celsius'");
        io::stdin().read_line(&mut input_scale).expect("Please type 'fahrenheit' or 'celsius'");

        match input_scale.as_str().trim() {
            "fahrenheit" => {
                scale = Scales::Fahrenheit;
                break;
            },
            "celsius" => {
                scale = Scales::Celsius;
                break;
            },
            _ => continue,
        }
    }

    let mut input_temp = String::new();

    println!("Enter a value:");

    io::stdin().read_line(&mut input_temp).expect("Please enter a number");

    let input_temp: i32 = input_temp.trim().parse().expect("Failed to parse to number");

    let result = match scale {
        Scales::Fahrenheit => convert_fahrenheit_to_celsius(input_temp),
        Scales::Celsius => convert_celsius_to_fahrenheit(input_temp),
    };

    println!("Result: {}", result)
}

fn convert_fahrenheit_to_celsius(fahrenheit: i32) -> i32 {
    (fahrenheit - 32) * 5/9
}

fn convert_celsius_to_fahrenheit(celsius: i32) -> i32 {
    (celsius * 9/5) + 32
}
