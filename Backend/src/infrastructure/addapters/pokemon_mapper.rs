use crate::domain::pokemon::{Ability, Pokemon, Statistics};
use crate::infrastructure::dto::pokemon_dto::{
    PokemonDTO, PokemonStatDTO, PokemonTypeDTO, PokemonAbilityDTO,
};
use crate::infrastructure::dto::pokemon_specie_dto::PokemonSpeciesDTO;
use crate::infrastructure::dto::shared::named_api_resource::{
    NamedAPIResource, EvolutionChainDTO, ChainLink,
};
use core::convert::TryFrom;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum StatKind { Hp, Attack, Defense, SpecialAttack, SpecialDefense, Speed }

impl TryFrom<&str> for StatKind {
    type Error = ();
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "hp"               => Ok(StatKind::Hp),
            "attack"           => Ok(StatKind::Attack),
            "defense"          => Ok(StatKind::Defense),
            "special-attack"   => Ok(StatKind::SpecialAttack),
            "special-defense"  => Ok(StatKind::SpecialDefense),
            "speed"            => Ok(StatKind::Speed),
            _ => Err(()),
        }
    }
}

fn map_statistics(stats_dto: &[PokemonStatDTO]) -> Statistics {
    let mut stats = Statistics::default();

    for s in stats_dto {
        let val: u8 = u8::try_from(s.base_stat).unwrap_or(u8::MAX);

        if let Ok(kind) = StatKind::try_from(s.stat.name.as_str()) {
            match kind {
                StatKind::Hp              => stats.hp = val,
                StatKind::Attack          => stats.attack = val,
                StatKind::Defense         => stats.defense = val,
                StatKind::SpecialAttack   => stats.special_attack = val,
                StatKind::SpecialDefense  => stats.special_defense = val,
                StatKind::Speed           => stats.speed = val,
            }
        }
    }

    stats
}

fn map_types(mut types: Vec<PokemonTypeDTO>) -> Vec<String> {
    types.sort_by_key(|t| t.slot);

    types.into_iter()
        .map(|t| t.r#type.name)
        .collect()
}

impl From<PokemonAbilityDTO> for Ability {
    fn from(dto: PokemonAbilityDTO) -> Self {
        Self {
            name: dto.ability.name,
            is_hidden: dto.is_hidden,
        }
    }
}

fn map_abilities(mut abilities: Vec<PokemonAbilityDTO>) -> Vec<Ability> {
    abilities.sort_by_key(|a| (a.is_hidden, a.slot));

    abilities.into_iter()
        .map(Ability::from)
        .collect()
}

fn map_egg_groups(eggs: Vec<NamedAPIResource>) -> Vec<String> {
    eggs.into_iter()
        .map(|g| g.name)
        .collect()
}
fn flatten_chain(link: &ChainLink, out: &mut Vec<String>) {
    out.push(link.species.name.clone());

    for next in &link.evolves_to {
        flatten_chain(next, out);
    }
}

fn map_evolution_names(chain: Option<EvolutionChainDTO>) -> Vec<String> {
    let mut v = Vec::new();
    if let Some(c) = chain {
        flatten_chain(&c.chain, &mut v);
    }
    v
}

pub fn to_domain(
    p: PokemonDTO,
    s: PokemonSpeciesDTO,
    chain: Option<EvolutionChainDTO>,
) -> Pokemon {
    let PokemonDTO {
        id,
        name,
        base_experience,
        stats,
        types,
        abilities,
        ..
    } = p;

    let PokemonSpeciesDTO {
        hatch_counter,
        egg_groups,
        ..
    } = s;

    let stats           = map_statistics(&stats);
    let typing          = map_types(types);
    let abilities       = map_abilities(abilities);
    let egg_groups      = map_egg_groups(egg_groups);
    let evolution_chain = map_evolution_names(chain);

    let hatch_counter   = hatch_counter;
    let base_experience = base_experience.unwrap_or_default();

    Pokemon::new(
        id,
        name,
        stats,
        hatch_counter,
        base_experience,
        egg_groups,
        typing,
        evolution_chain,
        abilities,
    )
}