use std::{fmt::Display, str::FromStr};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CountryCode {
    Jp,
    En,
    Kr,
    Tw,
}

impl CountryCode {
    pub const ALL: [CountryCode; 4] = [
        CountryCode::Jp,
        CountryCode::En,
        CountryCode::Kr,
        CountryCode::Tw,
    ];
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct PatchingCode(pub CountryCode);

impl PatchingCode {
    pub const ALL: [PatchingCode; 4] = [
        PatchingCode(CountryCode::Jp),
        PatchingCode(CountryCode::En),
        PatchingCode(CountryCode::Kr),
        PatchingCode(CountryCode::Tw),
    ];
}

impl FromStr for PatchingCode {
    type Err = InvalidCCStr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Ok(Self(CountryCode::Jp));
        }
        Ok(Self(s.parse()?))
    }
}

impl Display for PatchingCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0 == CountryCode::Jp {
            write!(f, "")
        } else {
            write!(f, "{}", self.0)
        }
    }
}

impl From<CountryCode> for PatchingCode {
    fn from(value: CountryCode) -> Self {
        Self(value)
    }
}

impl From<PatchingCode> for CountryCode {
    fn from(value: PatchingCode) -> Self {
        value.0
    }
}

#[derive(Debug)]
pub struct InvalidCCStr(pub String);

impl FromStr for CountryCode {
    type Err = InvalidCCStr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "jp" => Self::Jp,
            "en" => Self::En,
            "kr" => Self::Kr,
            "tw" => Self::Tw,
            _ => return Err(InvalidCCStr(s.to_string())),
        })
    }
}

impl Display for CountryCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CountryCode::Jp => "jp",
                CountryCode::En => "en",
                CountryCode::Kr => "kr",
                CountryCode::Tw => "tw",
            }
        )
    }
}
