#[cfg(test)]
mod tests {
    use math_power::funcs::sci;

    #[test]
    fn normal_sci_not() {
        assert_eq!(sci::sci_2_num("3.4*10^4"), Ok(34000));
    }

    #[test]
    fn sci_not_without_period_more_than_ten() {
        assert_eq!(sci::sci_2_num("34*10^4"), Err("Invalid scientific notation".to_string()));
    }
    #[test]
    fn sci_not_without_ten() {
        assert_eq!(sci::sci_2_num("3.4*^8"), Err("Invalid scientific notation".to_string()));
    }
    #[test]
    fn sci_not_without_power() {
        assert_eq!(sci::sci_2_num("3.4*10^"), Err("Invalid scientific notation".to_string()));
    }
    #[test]
    fn sci_not_without_power_and_power_simbol() {
        assert_eq!(sci::sci_2_num("3.4*10"), Err("Invalid scientific notation".to_string()));
    }
    #[test]
    fn valid_sci_not_positive_power() {
        assert_eq!(sci::sci_2_num("2.5*10^3"), Ok(2500));
    }
    
    
    #[test]
    fn valid_sci_not_no_decimal() {
        assert_eq!(sci::sci_2_num("4*10^2"), Ok(400));
    }
    
    #[test]
    fn invalid_sci_not_invalid_power() {
        assert_eq!(sci::sci_2_num("2.5*10^abc"), Err("Invalid scientific notation".to_string()));
    }
    
    #[test]
    fn invalid_sci_not_missing_multiplication() {
        assert_eq!(sci::sci_2_num("2.5 10^3"), Err("Invalid scientific notation".to_string()));
    }
    
    #[test]
    fn invalid_sci_not_missing_power() {
        assert_eq!(sci::sci_2_num("2.5*10"), Err("Invalid scientific notation".to_string()));
    }
    
    #[test]
    fn invalid_sci_not_multiple_decimal_points() {
        assert_eq!(sci::sci_2_num("1.2.3*10^4"), Err("Invalid scientific notation".to_string()));
    }
}