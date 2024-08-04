use apis::{FlatMap, HexWorld};

use crate::{pawns::position::PawnPosition, player::Player};

pub struct PawnMap {
    map: FlatMap<PawnPosition>
}

impl PawnMap {
    pub fn new(world: HexWorld<f32>, pawn_count: usize, player_a: Player, player_b: Player) -> Self {
        todo!();
    }
}