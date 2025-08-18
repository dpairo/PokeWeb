use serde::Deserialize;
use crate::infrastructure::dto::shared::named_api_resource::{
    NamedAPIResource,
    EvolutionAPIResource,
};

#[derive(Deserialize, Debug, Clone)]
pub struct PokemonSpeciesDTO {
    pub id: u16,
    pub name: String,

    #[serde(default)]
    pub hatch_counter: u8,

    #[serde(default)]
    pub egg_groups: Vec<NamedAPIResource>,

    pub evolution_chain: Option<EvolutionAPIResource>,
}