use apis::Edge;

use super::team::Team;

#[derive(Copy, Clone)]
pub struct PawnPosition {
    pub team: Team,
    pub facing: Edge,
}

impl PawnPosition {
    pub fn new(team: Team, facing: Edge) -> Self {
        Self { team, facing }
    }
}
