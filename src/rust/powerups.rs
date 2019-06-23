#[derive(Clone, Copy, PartialEq)]
pub enum PowerUp {
    Extension,       // {code: 'B'},
    Boost,           // {code: 'F'},
    Drill,           // {code: 'L'},
    MysteriousPoint, // {code: 'X'},
    Teleport,        // {code: 'R'},
    Cloner,             // {code: 'C'},
}
impl PowerUp {
    pub fn to_char(&self) -> char {
        match self {
            PowerUp::Extension => 'B',
            PowerUp::Boost => 'F',
            PowerUp::Drill => 'L',
            PowerUp::MysteriousPoint => 'X',
            PowerUp::Teleport => 'R',
            PowerUp::Cloner => 'C',
        }
    }
}

pub trait ByCode {
    fn by_code(code: char) -> Self;
}

impl ByCode for PowerUp {
    fn by_code(code: char) -> Self {
        match code {
            'B' => PowerUp::Extension,
            'F' => PowerUp::Boost,
            'L' => PowerUp::Drill,
            'X' => PowerUp::MysteriousPoint,
            'R' => PowerUp::Teleport,
            'C' => PowerUp::Cloner,
            _ => panic!("Unknown powerup code '{}'", code),
        }
    }
}
