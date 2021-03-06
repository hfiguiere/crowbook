use yaml_rust::{YamlLoader, Yaml};
use yaml_rust::yaml::Hash;

static EN: &'static str = include_str!("../../lang/en.yaml");
static FR: &'static str = include_str!("../../lang/fr.yaml");

/// Get the hashmap for a given language
pub fn get_hash(lang: &str) -> Hash {
    let docs = match lang {
        "fr" => YamlLoader::load_from_str(FR).unwrap(),
        _ => YamlLoader::load_from_str(EN).unwrap(),
    };
    let elem = docs.into_iter().next().unwrap();
    if let Yaml::Hash(hash) = elem {
        hash
    } else {
        panic!(lformat!("Yaml file for language {lang} didn't contain a hash",
                        lang = lang));
    }
}


/// Get a string for a given language
pub fn get_str(lang: &str, s: &str) -> String {
    let hash = get_hash(lang);
    let yaml = hash.get(&Yaml::String(s.to_owned()))
        .expect(&lformat!("Could not find translation for {key} in language {lang}",
                          key = s,
                          lang = lang));
    if let &Yaml::String(ref result) = yaml {
        result.clone()
    } else {
        panic!(lformat!("Yaml for {key} in lang {lang} is not a String!",
                        key = s,
                        lang = lang));
    }
}
