use apis::{FlatMap, HexWorld};

use crate::{pawns::position::PawnPosition, player::Player};

pub struct PawnMap {
    map: FlatMap<Option<PawnPosition>>
}

impl PawnMap {
    pub fn new(world: HexWorld<f32>, pawn_count: usize, player_a: Player, player_b: Player) -> Self {
        let mut map = FlatMap::new(world.world_shape);

        Self::spawn(&mut map, pawn_count, player_a, player_b);

        Self {
            map
        }
    }

    fn spawn(map: &mut FlatMap<Option<PawnPosition>>, pawn_count: usize, player_a: Player, player_b: Player) {

    }
}