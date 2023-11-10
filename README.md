# Rust Scientific
 This repository has functions written in rust for scientific porpuses, or math porpuses, for now has the following functions:
## Function`sci_2_num`

This function is designed to convert a scientific notation string into a regular integer (`u128`). It performs a series of checks and transformations to ensure the input is valid and can be correctly converted.

### Parameters

- `sci_not` (type: `&str`): The scientific notation string to be converted into an integer.

### Returns

- `u128`: The converted integer value.

### Exceptions

- If the scientific notation string contains more than one decimal point, an `Invalid scientific notation` panic is raised.
- If the integer part of the scientific notation string is empty, an `Invalid scientific notation` panic is raised.
- If the integer value is greater than 10, an `Invalid scientific notation` panic is raised.
- If the power part of the scientific notation string does not contain exactly one exponentiation operator (^), an `Invalid scientific notation` panic is raised.
- If any of the parsed values (integer part, decimal part, power value) fail to parse as `u128`, an `Invalid scientific notation` panic is raised.
- If the number fails to parse as `u128` after the necessary transformations, a `Failed to parse number` panic is raised.

### Format

The function `sci_2_num` only accepts input in the following scientific notation format: "number*10^exponent". The input must adhere to the structure where a numerical value is multiplied by 10 raised to a certain exponent.

#### Examples of Valid Input

- "1.32*10^5": The decimal part is optional, and the exponent can be a positive integer.
- "3*10^4": The decimal part can be omitted, and the exponent should be a non-negative integer.

#### Invalid Input

Any deviation from the specified format may result in panics or errors, as outlined in the Exceptions section of the documentation.

### Example

```rust
let result = sci_2_num("1.23*10^5");
println!("{}", result);
// Output: 123000
