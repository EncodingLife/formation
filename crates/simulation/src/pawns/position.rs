use apis::{Edge, HexCoord};

use super::team::Team;

#[derive(Copy, Clone)]
pub struct PawnPosition {
    pos: HexCoord,
    facing: Edge,
    team: Team
}