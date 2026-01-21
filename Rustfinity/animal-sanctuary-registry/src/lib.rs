use std::collections::HashMap;

type Collection = HashMap<String, Vec<String>>;

pub fn add_animal_to_section(animal: &str, section: &str, registry: &mut Collection) {
    let k = String::from(section);
    
    if !registry.contains_key(&k) {
        registry.insert(k, vec![String::from(animal)]);
    } else {
        if let Some(v) = registry.get_mut(&k) {
            let e = String::from(animal);

            if !v.contains(&e) {
                v.push(e);
            }            
        }
    }
}

pub fn get_animals_in_section(section: &str, registry: &Collection) -> Vec<String> {
    let k = String::from(section);

    if let Some(v) = registry.get(&k) {
        let mut v = v.clone();

        v.sort();
        v.to_vec()
    } else {
        let v: Vec<String> = vec![];

        v
    }
}

pub fn get_all_animals_sorted(registry: &Collection) -> Vec<String> {
    let mut ret_v: Vec<String> = vec![];

    for (key, val) in registry.iter() {
        for elem in val.iter() {
            ret_v.push(elem.to_string());
        }
    }

    ret_v.sort();
    ret_v
}
