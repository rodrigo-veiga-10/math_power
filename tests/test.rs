#[cfg(test)]
mod tests {
    use math_power::funcs::sci;

    #[test]
    fn normal_sci_not() {
        assert_eq!(sci::sci_2_num("3.4*10^4"), 34000);
    }

    #[test]
    #[should_panic(expected = "Invalid scientific notation")]
    fn sci_not_without_period_more_than_ten() {
        sci::sci_2_num("34*10^4");
    }

    #[test]
    #[should_panic(expected = "Invalid scientific notation")]
    fn sci_not_without_ten() {
        sci::sci_2_num("3.4*^8");
    }

    #[test]
    #[should_panic(expected = "Invalid scientific notation")]
    fn sci_not_without_power() {
        sci::sci_2_num("3.4*10^");
    }

    #[test]
    #[should_panic(expected = "Invalid scientific notation")]
    fn sci_not_without_power_and_power_simbol() {
        sci::sci_2_num("3.4*10");
    }

    #[test]
    fn valid_sci_not_positive_power() {
        assert_eq!(sci::sci_2_num("2.5*10^3"), 2500);
    }

    #[test]
    fn valid_sci_not_no_decimal() {
        assert_eq!(sci::sci_2_num("4*10^2"), 400);
    }

    #[test]
    #[should_panic(expected = "Invalid scientific notation")]
    fn invalid_sci_not_invalid_power() {
        sci::sci_2_num("2.5*10^abc");
    }

    #[test]
    #[should_panic(expected = "Invalid scientific notation")]
    fn invalid_sci_not_missing_multiplication() {
        sci::sci_2_num("2.5 10^3");
    }

    #[test]
    #[should_panic(expected = "Invalid scientific notation")]
    fn invalid_sci_not_missing_power() {
        sci::sci_2_num("2.5*10");
    }

    #[test]
    #[should_panic(expected = "Invalid scientific notation")]
    fn invalid_sci_not_multiple_decimal_points() {
        sci::sci_2_num("1.2.3*10^4");
    }
}
