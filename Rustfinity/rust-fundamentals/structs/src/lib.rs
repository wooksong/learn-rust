pub struct Person {
    pub name: String,
    pub age: u8,
}

pub fn is_adult(person: &Person) -> bool {
    if person.age >= 18 {
        return true;
    }
    false
}
