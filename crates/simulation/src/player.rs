use crate::{agent::player::PlayerAgent, pawns::team::Team};

pub struct Player(pub Box<dyn PlayerAgent>, Team);