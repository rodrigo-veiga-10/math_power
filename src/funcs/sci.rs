/*
    This function is designed to convert a scientific notation string into a regular integer (u32)
    It performs a series of checks and transformations 
    to ensure the input is valid and can be correctly converted
*/
pub fn sci_2_num(sci_not: &str) -> Result<u32, String> {

    let sci_parts: Vec<&str> = sci_not.split("*").collect();

    if sci_parts[0].matches(".").count() > 1 {
        return Err("Invalid scientific notation".to_string());
    }

    let num_parts: Vec<&str> = sci_parts[0].split(".").collect();


    let (integer_part, decimal_part) = if num_parts[0].len() == 0 {
        return Err("Invalid scientific notation".to_string());
    } else if num_parts.len() == 1 {
        (num_parts[0], "")
    } else {
        (num_parts[0], num_parts[1])
    };


    let integer_value = integer_part.parse::<u32>().map_err(|_| "Invalid scientific notation".to_string())?;

    let decimal_value = if decimal_part.len() > 0 {
        Some(decimal_part.parse::<u32>().map_err(|_| "Invalid scientific notation".to_string())?)
    } else {
        None
    };

    if integer_value > 10  {
        return Err("Invalid scientific notation".to_string());
    }


    let power: &str = sci_parts[1];
    let power_parts: Vec<&str> = power.split("^").collect();


    if power_parts.len() != 2 {
        return Err("Invalid scientific notation".to_string());
    }


    let _power_value1 = power_parts[0].parse::<u32>().map_err(|_| "Invalid scientific notation".to_string())?;
    let power_value2 = power_parts[1].parse::<u32>().map_err(|_| "Invalid scientific notation".to_string())?;

    let power_ten: u32 = power_value2 - decimal_value.unwrap_or(0).to_string().len() as u32;


    let num_zeros = "0".repeat(power_ten as usize);

    let num = format!("{}{}{}", integer_value, decimal_value.unwrap_or(0), num_zeros);


    Ok(num.parse::<u32>().map_err(|_| "Failed to parse number".to_string())?)
}
