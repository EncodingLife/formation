use std::slice::Iter;

use apis::{Edge, StaticMap, HexCoord, HexCoordinate, HexWorld};

use crate::{
    pawns::{
        position::PawnPosition,
        team::{MatchTeam::{Green, Purple}, Team, TeamId},
    },
    player::Player,
};

use super::spawning::get_spawn_coords;

pub struct PawnMap {
    map: StaticMap<PawnPosition>,
}

impl PawnMap {
    pub fn new(
        world: HexWorld<f32>,
        pawn_count: usize,
        player_a: Player,
        player_b: Player,
    ) -> Self {
        let mut map = StaticMap::new(world.world_shape);

        Self::spawn(&mut map, world, pawn_count, player_a, player_b);

        Self { map }
    }

    pub fn coord_iter(&self) -> impl Iterator<Item = (HexCoord, Option<&PawnPosition>)> {
        self.map.coord_iter()
    }

    fn spawn(
        map: &mut StaticMap<PawnPosition>,
        world: HexWorld<f32>,
        pawn_count: usize,
        player_a: Player,
        player_b: Player,
    ) {
        let center = world.center();

        let player_a_spawns = get_spawn_coords(world.world_shape, pawn_count);
        let player_b_spawns: Vec<HexCoord> = player_a_spawns
            .iter()
            .map(|&h| (h - center).reflect_q() + center)
            .collect();

        for coords in player_a_spawns {
            map.set(coords, Some(PawnPosition::new(Team::new(player_a.1, Purple), Edge::Q)));
        }

        for coords in player_b_spawns {
            map.set(coords, Some(PawnPosition::new(Team::new(player_b.1, Green), Edge::R)));
        }
    }
}
