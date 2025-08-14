use crate::length::imp_length_unit::ImpLengthUnit;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum LengthUnit {
	Imp(ImpLengthUnit),
}

pub struct Length {
	value: f64,        // value in base unit
	base_unit: LengthUnit,
}

impl Length {
	pub fn new(value: f64, base_unit: LengthUnit) -> Self {
		Length { value, base_unit }
	}

	pub fn as_mile(&self) -> f64 {
		match self.base_unit {
			LengthUnit::Imp(unit) => match unit {
				ImpLengthUnit::Mile => self.value,
				ImpLengthUnit::Foot => self.value / 5280.0,
				ImpLengthUnit::Yard => self.value / 1760.0,
				ImpLengthUnit::Inch => self.value / (5280.0 * 12.0),
			}
		}
	}

	pub fn as_foot(&self) -> f64 {
		match self.base_unit {
			LengthUnit::Imp(unit) => match unit {
				ImpLengthUnit::Mile => self.value * 5280.0,
				ImpLengthUnit::Foot => self.value,
				ImpLengthUnit::Yard => self.value * 3.0,
				ImpLengthUnit::Inch => self.value / 12.0,
			}
		}
	}

	pub fn as_inch(&self) -> f64 {
		match self.base_unit {
			LengthUnit::Imp(unit) => match unit {
				ImpLengthUnit::Mile => self.value * 5280.0 * 12.0,
				ImpLengthUnit::Foot => self.value * 12.0,
				ImpLengthUnit::Yard => self.value * 36.0,
				ImpLengthUnit::Inch => self.value,
			}
		}
	}

	pub fn as_yard(&self) -> f64 {
		match self.base_unit {
			LengthUnit::Imp(unit) => match unit {
				ImpLengthUnit::Mile => self.value * 1760.0,
				ImpLengthUnit::Foot => self.value / 3.0,
				ImpLengthUnit::Yard => self.value,
				ImpLengthUnit::Inch => self.value / 36.0,
			}
		}
	}
}