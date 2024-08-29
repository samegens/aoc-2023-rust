use std::str::FromStr;

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum CubeColor {
    Red,
    Green,
    Blue
}

impl FromStr for CubeColor {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "red" => Ok(CubeColor::Red),
            "green" => Ok(CubeColor::Green),
            "blue" => Ok(CubeColor::Blue),
            _ => Err(()),
        }
    }
}
