use crate::game::utils::PlayerNumber;

#[derive(Debug)]
pub struct Player {
    name: String,
}

impl Player {
    pub fn new(name: String, number: PlayerNumber) -> Player {        
        Player{ name }
    }    
}
