use std::fmt::Display;

#[derive(PartialEq, Copy, Clone)]
pub struct Team(u16);

impl Display for Team {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:X}", self.0)
    }
}