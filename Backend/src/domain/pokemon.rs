use crate::infrastructure::dto::pokemon_dto::PokemonDTO;
#[derive(Debug, Clone)]
struct Ability {
    name: String,
    is_hidden: bool
}

#[derive(Debug, Clone)]
struct Statistics {
    pub hp: u8,
    pub attack : u8,
    pub defense: u8,
    pub special_attack: u8,
    pub special_defense: u8,
    pub speed : u8
}
#[derive(Debug, Clone)]
pub struct Pokemon {
    id: u16,
    name: String,
    stats: Statistics,
    hatch_counter: u8,
    base_experience: u16,
    egg_groups: Vec<String>,
    typing: Vec<String>,
    evolution_chain: Vec<String>,
    abilities: Vec<Ability>
}

impl Pokemon {
    
}