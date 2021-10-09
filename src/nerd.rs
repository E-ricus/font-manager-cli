use std::str::FromStr;

#[derive(Debug)]
pub(crate) enum NerdFonts {
    FiraCode(String),
    SourceCode(String),
}

impl FromStr for NerdFonts {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "FiraCode" => Ok(Self::FiraCode(s.to_string())),
            "SourceCodePro" => Ok(Self::SourceCode(s.to_string())),
            _ => Err("This font doesn't exsist on the nerd aggregator".into()),
        }
    }
}

pub(crate) fn install_nerd(font: NerdFonts) {
    match font {
        NerdFonts::FiraCode(value) => print!("{}", value),
        NerdFonts::SourceCode(value) => print!("{}", value),
    }
}
