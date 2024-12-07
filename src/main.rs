use dialoguer::{theme::ColorfulTheme, Input};

fn main() {
    let num = get_valid_number("Enter a number", 100);
    let hex_num = get_valid_hex_number("Enter a hexadecimal number: ");
    let num2 = get_number("Enter a number");

    println!(
        "You entered: {} \
         hex: 0x{:x} \
         num2: {}",
        num, hex_num, num2
    );
}

fn get_valid_number(prompt: &str, max: u32) -> u32 {
    Input::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .validate_with(move |input: &String| -> Result<(), &str> {
            match input.parse::<u32>() {
                Ok(value) if value < max => Ok(()),
                Ok(_) => Err("Please enter a number less than 100"),
                Err(_) => Err("Please enter a valid number"),
            }
        })
        .interact()
        .map(|s| s.parse::<u32>().unwrap())
        .unwrap()
}

fn get_valid_hex_number(prompt: &str) -> i64 {
    let hex_input: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .validate_with(|input: &String| -> Result<(), &str> {
            match i64::from_str_radix(input, 16) {
                Ok(_) => Ok(()),
                Err(_) => Err("Please enter a valid hexadecimal number"),
            }
        })
        .interact()
        .unwrap();
    i64::from_str_radix(&hex_input, 16).unwrap()
}

fn get_number(prompt: &str) -> u32 {
    Input::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .interact()
        .unwrap()
}
