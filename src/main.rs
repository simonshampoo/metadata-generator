use metadata_generator::reader::get_trait_names;

fn main() {
    get_trait_names().unwrap();
}

// scripts that randomize traits and produce json
// we can then post these json onto ipfs

// posting things on ipfs much easier
// code hosting on ipfs
// nft images and metadata on ipfs

// user will have text files where the title of each text file represents the trait
//     e.g. "hat.txt"
// each text file will contain a CSV of trait descriptions to it's rarity
// we will first check to see if rarity sums to 0, else malformatted
//      e. g. "hat.txt" will look like
/*
beanie 50
pirate hat 25
baseball cap 15
cowboy hat 5,
halo 4
crown 1


after going through all of the files in traits/x.txt,

we will have a shitload of structs that look like

    Metadata {
        trait1: "blah"
        trait2: "foo"
        ...
        traitk = "waa"
    }
*/

//RUST IS KICKIGN MY ASS FFS
