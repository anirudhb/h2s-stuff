/* Defines a cell. */
use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Cell {
    Dead,
    Alive,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} cell", match self {
            &Cell::Dead => "dead".to_owned(),
            &Cell::Alive => "alive".to_owned(),
        })
    }
}
