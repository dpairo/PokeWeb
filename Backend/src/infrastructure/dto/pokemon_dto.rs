use serde::Deserialize;
use crate::infrastructure::dto::shared::named_api_resource::NamedAPIResource;

#[derive(Deserialize, Debug, Clone)]
pub struct PokemonDTO {
    pub id: u16,
    pub name: String,
    pub base_experience: Option<u16>,
    pub height: Option<u16>,
    pub weight: Option<u16>,
    pub species: NamedAPIResource,

    #[serde(default)]
    pub stats: Vec<PokemonStatDTO>,

    #[serde(default)]
    pub types: Vec<PokemonTypeDTO>,

    #[serde(default)]
    pub moves: Vec<PokemonMoveDTO>,

    #[serde(default)]
    pub abilities: Vec<PokemonAbilityDTO>,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct PokemonStatDTO {
    pub base_stat: u16,
    pub effort: u8,
    pub stat: NamedAPIResource,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PokemonTypeDTO {
    pub slot: u8,
    pub r#type: NamedAPIResource,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PokemonAbilityDTO {
    pub ability: NamedAPIResource,
    pub is_hidden: bool,
    pub slot: u8,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PokemonMoveDTO {
    pub r#move: NamedAPIResource,
    #[serde(default)]
    pub version_group_details: Vec<MoveVersionDTO>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct MoveVersionDTO {
    pub level_learned_at: u8,
    pub move_learn_method: NamedAPIResource,
    pub version_group: NamedAPIResource,
}