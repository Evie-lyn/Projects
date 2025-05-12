pub fn cali_income_tax(income: f64, filing_status: &str) -> f64 {
    match filing_status.to_lowercase().as_str() {
        "single" | "married filing separately" => cali_single_tax(income),
        "married filing jointly" | "qualifying surviving spouse" => cali_joint_tax(income),
        "head of household" => cali_headhouse_tax(income),
        _ => {
            eprintln!("Warning: Invalid filing status '{}'. Please enter 'single, 'married filing seperately', 'head of household', 'married filing jointly', or 'qualifying surviving spouse'.", filing_status);
            cali_single_tax(income)
        }
    }
}

fn cali_single_tax(mut income: f64) -> f64 {
    let mut tax = 0.0;

    if income > 721314.0 {
        tax += (income - 721314.0) * 0.123;
        income = 721314.0;
    }
    if income > 432787.0 {
        tax += (income - 432787.0) * 0.113;
        income = 432787.0;
    }
    if income > 360659.0 {
        tax += (income - 360659.0) * 0.103;
        income = 360659.0;
    }
    if income > 70606.0 {
        tax += (income - 70606.0) * 0.093;
        income = 70606.0;
    }
    if income > 55866.0 {
        tax += (income - 55866.0) * 0.08;
        income = 55866.0;
    }
    if income > 40245.0 {
        tax += (income - 40245.0) * 0.06;
        income = 40245.0;
    }
    if income > 25499.0 {
        tax += (income - 25499.0) * 0.04;
        income = 25499.0;
    }
    if income > 10756.0 {
        tax += (income - 10756.0) * 0.02;
        income = 10756.0;
    }
    tax +=income * 0.01;

    tax
}

fn cali_joint_tax(mut income: f64) -> f64 {
    let mut tax = 0.0;

    if income > 1442628.0 {
        tax += (income - 1442628.0) * 0.123;
        income = 1442628.0;
    }
    if income > 865574.0 {
        tax += (income - 865574.0) * 0.113;
        income = 865574.0;
    }
    if income > 721318.0 {
        tax += (income - 721318.0) * 0.103;
        income = 721318.0;
    }
    if income > 141212.0 {
        tax += (income - 141212.0) * 0.093;
        income = 141212.0;
    }
    if income > 111732.0 {
        tax += (income - 111732.0) * 0.08;
        income = 111732.0;
    }
    if income > 80490.0 {
        tax += (income - 80490.0) * 0.06;
        income = 80490.0;
    }
    if income > 50998.0 {
        tax += (income - 50998.0) * 0.04;
        income = 50998.0;
    }
    if income > 21512.0 {
        tax += (income - 21512.0) * 0.02;
        income = 21512.0;
    }
    tax += income * 0.01;

    tax
}

fn cali_headhouse_tax(mut income: f64) -> f64 {
    let mut tax = 0.0;

    if income > 980988.0 {
        tax += (income - 980988.0) * 0.123;
        income = 980988.0;
    }
    if income > 588594.0 {
        tax += (income - 588594.0) * 0.113;
        income = 588594.0;
    }
    if income > 490494.0 {
        tax += (income - 490494.0) * 0.103;
        income = 490494.0;
    }
    if income > 96108.0 {
        tax += (income - 96108.0) * 0.093;
        income = 96108.0;
    }
    if income > 81365.0 {
        tax += (income - 81365.0) * 0.08;
        income = 81365.0;
    }
    if income > 65745.0 {
        tax += (income - 65745.0) * 0.06;
        income = 65745.0;
    }
    if income > 51001.0 {
        tax += (income - 51001.0) * 0.04;
        income = 51001.0;
    }
    if income > 21528.0 {
        tax += (income - 21528.0) * 0.02;
        income = 21528.0;
    }
    tax += income * 0.01;

    tax
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_low_income() {
        let income = 5000.0;
        let filing_status = "single";
        let expected = 50.0;
        let actual = cali_income_tax(income, filing_status);
        let epsilon = 0.0001;
        assert!((actual - expected).abs() < epsilon, "Expected close to {}, got {}", expected, actual);
    }

    #[test]
    fn test_single_bracket_2() {
        let income = 20000.0;
        let filing_status = "single";
        let expected = (10756.0 * 0.01) + (20000.0 - 10756.0) * 0.02;
        let actual = cali_income_tax(income, filing_status);
        let epsilon = 0.0001;
        assert!((actual - expected).abs() < epsilon, "Expected close to {}, got {}", expected, actual);
    }

    #[test]
    fn test_single_bracket_4() {
        let income = 30000.0;
        let filing_status = "single";
        let expected = (10756.0 * 0.01) + ((25499.0 - 10756.0) * 0.02) + ((30000.0 - 25499.0) * 0.04);
        let actual = cali_income_tax(income, filing_status);
        let epsilon = 0.0001;
        assert!((actual - expected).abs() < epsilon, "Expected close to {}, got {}", expected, actual);
    }

    #[test]
    fn test_joint_low_income() {
        let income = 10000.0;
        let filing_status = "married filing jointly";
        let expected = 100.0;
        let actual = cali_income_tax(income, filing_status);
        let epsilon = 0.0001;
        assert!((actual - expected).abs() < epsilon, "Expected close to {}, got {}", expected, actual);
    }

    #[test]
    fn test_joint_bracket_3() {
        let income = 60000.0;
        let filing_status = "married filing jointly";
        let expected = (21512.0 * 0.01) + ((50998.0 - 21512.0) * 0.02) + ((60000.0 - 50998.0) * 0.04);
        let actual = cali_income_tax(income, filing_status);
        let epsilon = 0.0001;
        assert!((actual - expected).abs() < epsilon, "Expected close to {}, got {}", expected, actual);
    }

    #[test]
    fn test_head_of_household_mid_range() {
        let income = 70000.0;
        let filing_status = "head of household";
        let expected = 1649.8; 
        let actual = cali_income_tax(income, filing_status);
        let epsilon = 0.0001;
        assert!((actual - expected).abs() < epsilon, "Expected close to {}, got {}", expected, actual);
    }

    #[test]
    fn test_invalid_status_defaults_to_single() {
        let income = 40000.0;
        let filing_status = "invalid";
        let expected = cali_single_tax(income);
        let actual = cali_income_tax(income, filing_status);
        let epsilon = 0.0001;
        assert!((actual - expected).abs() < epsilon, "Expected close to {}, got {}", expected, actual);
    }
}