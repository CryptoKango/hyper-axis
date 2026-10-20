
use super::{MarketKind, MarketPrecision, hip3, regular_perp, spot};

#[test]
fn computes_asset_ids_from_official_formulas() {
    assert_eq!(regular_perp(7).get(), 7);
    assert_eq!(spot(12).unwrap().get(), 10_012);
    assert_eq!(hip3(2, 17).unwrap().get(), 120_017);
}

#[test]
fn enforces_perp_price_and_size_precision() {
    let precision = MarketPrecision::new(MarketKind::Perpetual, 2).unwrap();
    assert_eq!(precision.price("123.45").unwrap().as_str(), "123.45");
    assert!(precision.price("123.456").is_err());
    assert!(precision.price("0.00001").is_err());
    assert_eq!(precision.size("1.20").unwrap().as_str(), "1.2");
    assert!(precision.size("1.234").is_err());
}

#[test]
fn spot_allows_two_more_price_decimals() {
    let precision = MarketPrecision::new(MarketKind::Spot, 2).unwrap();
    assert!(precision.price("0.000001").is_ok());
    assert!(precision.price("0.0000001").is_err());
}
