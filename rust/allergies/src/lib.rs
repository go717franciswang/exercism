#[derive(Clone, PartialEq, Debug)]
pub enum Allergen {
    Eggs = 0,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats
}

pub struct Allergies {
    score: u64
}

impl Allergies {
    pub fn new(score: u64) -> Self {
        Allergies { score: score }
    }

    pub fn is_allergic_to(&self, a: &Allergen) -> bool {
        (self.score >> (a.clone() as u64)) % 2 == 1
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        [
            Allergen::Eggs, 
            Allergen::Peanuts, 
            Allergen::Shellfish,
            Allergen::Strawberries,
            Allergen::Tomatoes, 
            Allergen::Chocolate, 
            Allergen::Pollen, 
            Allergen::Cats
        ].iter()
         .filter(|a| self.is_allergic_to(&a))
         .cloned()
         .collect::<Vec<Allergen>>()
    }
}

