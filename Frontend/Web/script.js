document.addEventListener("DOMContentLoaded", function () {
    const pokemonList = document.getElementById('pokemonList');
    const pokeSearch = document.getElementById('pokeSearch')
    const searchBtn = document.getElementById('search-button')

    const url = "https://pokeapi.co/api/v2/pokemon?limit=1000&offset=0";
    fetch(url)
        .then(response => {
            if (!response.ok) {
                throw new Error("Respuesta no válida del servidor");
            }
            return response.json();
        })
        .then(async data => {
        for (const pokemon of data.results) {
            const id = pokemon.url.split("/").filter(Boolean).pop();
            await createPokemon(id); // Espera a que termine antes de seguir con el siguiente
        }
        })
        .catch(error => {
            console.error("Error cargando Pokémon:", error);
        });

        searchBtn.addEventListener('click',async function() {
            console.log(pokeSearch.value);
            if(pokeSearch.value.trim() !== ''){
                try {
                    pokemonList.innerHTML = "";
                    createPokemon(pokeSearch.value);

                } catch (error) {
                    console.log("pokemon desconocido",error)
                }
            }
        });

        async function createPokemon(pokemon){
                try {
                    const res = await fetch(`https://pokeapi.co/api/v2/pokemon/${pokemon}`);
                    if (!res.ok) throw new Error("No se ha encontrado el Pokémon");
                    
                    const data = await res.json();
                    console.log(data);

                    const poke = document.createElement('a');
                    poke.href = "#";
                    poke.classList.add('preview');

                    const img = document.createElement('img');
                    img.src = `https://raw.githubusercontent.com/PokeAPI/sprites/master/sprites/pokemon/${data.id}.png`;
                    img.alt = data.name;

                    const pokeName = document.createElement('p');
                    pokeName.textContent = `${data.name} #${data.id}`;

                    poke.appendChild(img);
                    poke.appendChild(pokeName);

                    data.types.forEach(t => {
                        const typeName = t.type.name; // ejemplo: 'fire', 'water', etc.

                        const typeContainer = document.createElement('div');
                        typeContainer.classList.add('icon', typeName); // útil si tienes estilos CSS por tipo

                        const typeIcon = document.createElement('img');
                        typeIcon.src = `../resources/icons/${typeName}.svg`;
                        typeIcon.alt = typeName;

                        typeContainer.appendChild(typeIcon);
                        poke.appendChild(typeContainer);
                    });

                    pokemonList.appendChild(poke);

                } catch (error) {
                    console.error(error.message);
                    alert("Error al buscar el Pokémon. Intenta con otro nombre o ID.");
                }

        }
});
