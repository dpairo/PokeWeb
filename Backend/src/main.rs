mod domain;
mod infrastructure;

use domain::pokemon::Pokemon;
use infrastructure::addapters::pokemon_mapper::to_domain;
use infrastructure::dto::pokemon_dto::{
    PokemonDTO, PokemonStatDTO, PokemonTypeDTO, PokemonAbilityDTO,
};
use infrastructure::dto::pokemon_specie_dto::PokemonSpeciesDTO;
use infrastructure::dto::shared::named_api_resource::{NamedAPIResource, EvolutionChainDTO};

fn main() {
    let stats = vec![
        PokemonStatDTO {
            base_stat: 45,
            effort: 0,
            stat: NamedAPIResource { name: "hp".into(), url: "".into() },
        },
        PokemonStatDTO {
            base_stat: 49,
            effort: 0,
            stat: NamedAPIResource { name: "attack".into(), url: "".into() },
        },
        PokemonStatDTO {
            base_stat: 49,
            effort: 0,
            stat: NamedAPIResource { name: "defense".into(), url: "".into() },
        },
        PokemonStatDTO {
            base_stat: 65,
            effort: 0,
            stat: NamedAPIResource { name: "special-attack".into(), url: "".into() },
        },
        PokemonStatDTO {
            base_stat: 65,
            effort: 0,
            stat: NamedAPIResource { name: "special-defense".into(), url: "".into() },
        },
        PokemonStatDTO {
            base_stat: 45,
            effort: 0,
            stat: NamedAPIResource { name: "speed".into(), url: "".into() },
        },
    ];

    let types = vec![
        PokemonTypeDTO {
            slot: 1,
            r#type: NamedAPIResource { name: "grass".into(), url: "".into() },
        },
        PokemonTypeDTO {
            slot: 2,
            r#type: NamedAPIResource { name: "poison".into(), url: "".into() },
        },
    ];

    let abilities = vec![
        PokemonAbilityDTO {
            ability: NamedAPIResource { name: "overgrow".into(), url: "".into() },
            is_hidden: false,
            slot: 1,
        },
        PokemonAbilityDTO {
            ability: NamedAPIResource { name: "chlorophyll".into(), url: "".into() },
            is_hidden: true,
            slot: 3,
        },
    ];

    let pokemon_dto = PokemonDTO {
        id: 1,
        name: "bulbasaur".into(),
        base_experience: Some(64),
        height: Some(7),
        weight: Some(69),
        species: NamedAPIResource { name: "bulbasaur".into(), url: "".into() },
        stats,
        types,
        moves: vec![],
        abilities,
    };

    let species_dto = PokemonSpeciesDTO {
        id: 1,
        name: "bulbasaur".into(),
        hatch_counter: 20,
        egg_groups: vec![
            NamedAPIResource { name: "monster".into(), url: "".into() },
            NamedAPIResource { name: "plant".into(), url: "".into() },
        ],
        evolution_chain: None,
    };

    let evolution_chain: Option<EvolutionChainDTO> = None;
    
    let pokemon: Pokemon = to_domain(pokemon_dto, species_dto, evolution_chain);
    
    println!("{:#?}", pokemon);
}
