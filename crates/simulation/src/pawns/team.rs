use std::fmt::Display;


#[derive(PartialEq, Copy, Clone)]
pub struct TeamId(pub u16);

impl Display for TeamId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:X}", self.0)
    }
}

#[derive(PartialEq, Copy, Clone)]
pub enum MatchTeam {
    Purple,
    Green,
}

impl MatchTeam {
    pub fn colour(&self) -> (u8, u8, u8) {
        match self {
            MatchTeam::Purple => (128, 0, 128),
            MatchTeam::Green => (0, 128, 0),
        }
    }
}

#[derive(PartialEq, Copy, Clone)]
pub struct Team {
    pub id: TeamId,
    pub match_team: MatchTeam,
}

impl Team {
    pub fn new(id: TeamId, match_team: MatchTeam) -> Self {
        Self { id, match_team }
    }
}