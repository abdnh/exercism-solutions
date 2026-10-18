pub struct Allergies {
    allergies: Vec<Allergen>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

impl TryFrom<u32> for Allergen {
    type Error = String;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(match value {
            1 => Allergen::Eggs,
            2 => Allergen::Peanuts,
            4 => Allergen::Shellfish,
            8 => Allergen::Strawberries,
            16 => Allergen::Tomatoes,
            32 => Allergen::Chocolate,
            64 => Allergen::Pollen,
            128 => Allergen::Cats,
            _ => return Err(format!("Invalid allergen score: {}", value)),
        })
    }
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        let allergies: Vec<Allergen> = (0..32)
            .filter_map(|i: u32| {
                let value = 1 << i;
                let allergen: Option<Allergen> = value.try_into().ok();
                allergen.and_then(|a| if value & score != 0 { Some(a) } else { None })
            })
            .collect();
        Self { allergies }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.allergies.contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        return self.allergies.clone();
    }
}
