/*
    This function is designed to convert a scientific notation string into a regular integer (i128)
    It performs a series of checks and transformations 
    to ensure the input is valid and can be correctly converted
*/
pub fn sci_2_num(sci_not: &str) -> u128 {

    let sci_parts: Vec<&str> = sci_not.split("*").collect();

    if sci_parts[0].matches(".").count() > 1 {
        panic!("Invalid scientific notation");
    }

    let num_parts: Vec<&str> = sci_parts[0].split(".").collect();


    let (integer_part, decimal_part) = if num_parts[0].len() == 0 {
        panic!("Invalid scientific notation");
    } else if num_parts.len() == 1 {
        (num_parts[0], "")
    } else {
        (num_parts[0], num_parts[1])
    };


    let integer_value = integer_part.parse::<u128>().expect("Invalid scientific notation");

    let decimal_value = if decimal_part.len() > 0 {
        Some(decimal_part.parse::<u128>().expect("Invalid scientific notation"))
    } else {
        None
    };

    if integer_value > 10  {
        panic!("Invalid scientific notation");
    }


    let power: &str = sci_parts[1];
    let power_parts: Vec<&str> = power.split("^").collect();


    if power_parts.len() != 2 {
        panic!("Invalid scientific notation");
    }


    let _power_value1 = power_parts[0].parse::<u128>().expect("Invalid scientific notation");
    let power_value2 = power_parts[1].parse::<u128>().expect("Invalid scientific notation");

    let power_ten: u128 = power_value2 - decimal_value.unwrap_or(0).to_string().len() as u128;


    let num_zeros = "0".repeat(power_ten as usize);

    let num = format!("{}{}{}", integer_value, decimal_value.unwrap_or(0), num_zeros);


    num.parse::<u128>().expect("Failed to parse number")
}
