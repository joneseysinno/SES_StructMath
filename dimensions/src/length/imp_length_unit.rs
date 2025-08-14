#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ImpLengthUnit {
    Inch,
    Foot,
    Mile,
    Yard,
}

impl ImpLengthUnit {
    pub fn from_string(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "in" | "inch" | "inches" | "\"" => Some(ImpLengthUnit::Inch),
            "ft" | "foot" | "feet" | "'"=> Some(ImpLengthUnit::Foot),
            "mile" => Some(ImpLengthUnit::Mile),
            "yard" => Some(ImpLengthUnit::Yard),
            _ => None,
        }
    }
}