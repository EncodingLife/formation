use agent::player::PlayerAgent;

use crate::{pawns::team::TeamId};

pub struct EmptyAgent();

impl PlayerAgent for EmptyAgent {}

pub struct Player(pub Box<dyn PlayerAgent>, pub TeamId);

impl Player {
    pub fn empty(id: u16) -> Self {
        Self(Box::new(EmptyAgent()), TeamId(id))
    }
}