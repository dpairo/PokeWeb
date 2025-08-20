use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub struct NamedAPIResource {
    pub name: String,
    pub url: String,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
pub struct EvolutionAPIResource {
    pub url: String,
}

impl EvolutionAPIResource {
    pub fn id(&self) -> Option<u32> { id_from_url(&self.url) }
}

#[derive(Deserialize, Debug, Clone)]
pub struct EvolutionChainDTO {
    pub chain: ChainLink,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ChainLink {
    pub species: NamedAPIResource,
    #[serde(default)]
    pub evolves_to: Vec<ChainLink>,
}
pub fn id_from_url(url: &str) -> Option<u32> {
    url.trim_end_matches('/')
        .rsplit('/')
        .next()
        .and_then(|s| s.parse::<u32>().ok())
}
