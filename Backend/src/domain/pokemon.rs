use crate::infrastructure::dto::pokemon_dto::PokemonDTO;
use crate::infrastructure::dto::pokemon_specie_dto::PokemonSpeciesDTO;
#[derive(Debug, Clone)]
pub struct Ability {
    pub(crate) name: String,
    pub(crate) is_hidden: bool
}

#[derive(Debug, Clone, Default)]
pub struct Statistics {
    pub hp: u8,
    pub attack : u8,
    pub defense: u8,
    pub special_attack: u8,
    pub special_defense: u8,
    pub speed : u8
}
#[derive(Debug, Clone)]
pub struct Pokemon {
    pub id: u16,
    pub name: String,
    pub stats: Statistics,
    pub hatch_counter: u8,
    pub base_experience: u16,
    pub egg_groups: Vec<String>,
    pub typing: Vec<String>,
    pub evolution_chain: Vec<String>,
    pub abilities: Vec<Ability>
}

impl Pokemon {
    pub fn new(
        id: u16,
        name: String,
        stats: Statistics,
        hatch_counter: u8,
        base_experience: u16,
        egg_groups: Vec<String>,
        typing: Vec<String>,
        evolution_chain: Vec<String>,
        abilities: Vec<Ability>,
    ) -> Self {
        Self {
            id,
            name,
            stats,
            hatch_counter,
            base_experience,
            egg_groups,
            typing,
            evolution_chain,
            abilities,
        }
    }
}