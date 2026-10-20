
use super::DecimalValue;

#[test]
fn canonicalizes_without_losing_precision() {
    let value = DecimalValue::parse("-00012.34000").unwrap();
    assert_eq!(value.as_str(), "-12.34");
    assert_eq!(value.scale(), 2);
    assert_eq!(value.significant_digits(), 4);
}

#[test]
fn canonicalizes_negative_zero() {
    assert_eq!(DecimalValue::parse("-0.000").unwrap().as_str(), "0");
}

#[test]
fn counts_leading_fractional_zeroes_correctly() {
    let value = DecimalValue::parse("0.001230").unwrap();
    assert_eq!(value.scale(), 5);
    assert_eq!(value.significant_digits(), 3);
}

#[test]
fn rejects_non_wire_syntax() {
    for value in ["", ".1", "1.", "+1", "1e2", "1.2.3", "--1"] {
        assert!(DecimalValue::parse(value).is_err(), "accepted {value}");
    }
}

#[test]
fn converts_to_fixed_scale_without_rounding() {
    assert_eq!(
        DecimalValue::parse("-1.25")
            .unwrap()
            .to_scaled_i64("amount", 6)
            .unwrap(),
        -1_250_000
    );
    assert!(
        DecimalValue::parse("0.0000001")
            .unwrap()
            .to_scaled_i64("amount", 6)
            .is_err()
    );
    assert!(
        DecimalValue::parse("9223372036855")
            .unwrap()
            .to_scaled_i64("amount", 6)
            .is_err()
    );
}

#[test]
fn converts_positive_values_to_unsigned_fixed_scale() {
    assert_eq!(
        DecimalValue::parse("12.5")
            .unwrap()
            .to_scaled_u64("amount", 6)
            .unwrap(),
        12_500_000
    );
    assert!(
        DecimalValue::parse("-1")
            .unwrap()
            .to_scaled_u64("amount", 6)
            .is_err()
    );
    assert!(
        DecimalValue::parse("0.0000001")
            .unwrap()
            .to_scaled_u64("amount", 6)
            .is_err()
    );
    assert!(
        DecimalValue::parse("18446744073709.551616")
            .unwrap()
            .to_scaled_u64("amount", 6)
            .is_err()
    );
}
