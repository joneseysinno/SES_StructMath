use dimensions::length::length::{Length, LengthUnit};
use dimensions::length::imp_length_unit::ImpLengthUnit;

#[test]
fn test_foot_to_mile() {
	let l = Length::new(5280.0, LengthUnit::Imp(ImpLengthUnit::Foot));
	assert_eq!(l.as_mile(), 1.0);
}

#[test]
fn test_inch_to_foot() {
	let l = Length::new(12.0, LengthUnit::Imp(ImpLengthUnit::Inch));
	assert_eq!(l.as_foot(), 1.0);
}
