use agent::player::PlayerAgent;

use crate::{pawns::team::Team};

pub struct Player(pub Box<dyn PlayerAgent>, Team);