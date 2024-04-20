use super::*;

#[test]
fn test_positive_integers() {
    assert_eq!(decompose(15f64).to_string(), "10011");
    assert_eq!(decompose(55f64).to_string(), "1001011");
    assert_eq!(decompose(85f64).to_string(), "1010101");
    assert_eq!(decompose(124f64).to_string(), "110001100");
}

#[test]
fn test_positive_decimals() {
    assert_eq!(decompose(0.1875).to_string(), "0.0111");
    assert_eq!(decompose(10.6875).to_string(), "11111.1111");
    assert_eq!(decompose(0.666666).to_string().chars().take(12).collect::<String>(), "1.1111111111");
    assert_eq!(decompose(20.34375).to_string(), "10101.10101");
    assert_eq!(decompose(0.328125).to_string(), "0.010101");
    assert_eq!(decompose(0.109375).to_string(), "0.011011");
    assert_eq!(decompose(0.5).to_string(), "1.1");
    assert_eq!(decompose(31.5).to_string(), "1100000.1");
}

fn test_negative_integers() {
    assert_eq!(decompose(-2f64).to_string(), "10");
    assert_eq!(decompose(-10f64).to_string(), "1010");
    assert_eq!(decompose(-6f64).to_string(), "1110");
    assert_eq!(decompose(-19f64).to_string(), "111101");
}

#[test]
fn test_negative_decimals() {
    assert_eq!(decompose(-0.5).to_string(), "0.1");
    assert_eq!(decompose(-0.625).to_string(), "0.101");
    assert_eq!(decompose(-0.375).to_string(), "0.111");
    assert_eq!(decompose(-34.875).to_string(), "101101.011");
}
