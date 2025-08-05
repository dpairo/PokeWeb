use serde::Deserialize;

#[derive(Deserialize)]
pub struct PokemonDTO {
    pub id: Option<u16>,
    pub name: Option<String>,
    pub base_experience: Option<u16>,
    pub stats: StatDTO,
    pub types: Option<Vec<TypeDTO>>,
    pub moves: Option<Vec<MovesDTO>>,
    pub abilities: Option<Vec<AbilityDTO>>,
    pub
}

#[derive(Deserialize)]
pub struct MovesDTO {
    pub r#move: Option<NamedAPIResource>,
    pub version_group_details: Option<Vec<MoveVersionDTO>>,
}

#[derive(Deserialize)]
pub struct MoveVersionDTO {
    pub move_learn_method: Option<NamedAPIResource>,
    pub version_group: Option<NamedAPIResource>
}

#[derive(Deserialize)]
pub struct StatDTO {
    pub base_stat: Option<u8>,
    pub stat: Option<NamedAPIResource>,
}

#[derive(Deserialize)]
pub struct AbilityDTO {
    pub ability: Option<NamedAPIResource>,
    pub is_hidden: Option<bool>,
}

#[derive(Deserialize)]
pub struct TypeDTO {
    pub r#type: Option<NamedAPIResource>,
}

#[derive(Deserialize)]
pub struct NamedAPIResource {
    pub id: Option<String>,
}