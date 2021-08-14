use crate::*;

type Comp<T> = Component<Star, T>;

#[derive(Debug, Clone)]
pub struct Star {
    pub name: String,
    pub mass: Mass,
    pub radius: Length,
    pub temp: Temperature,
    pub position: Position,
}

fixed_id!(Star);

#[derive(Debug, Default)]
pub struct Stars {
    pub name: Comp<String>,
    pub mass: Comp<Mass>,
    pub radius: Comp<Length>,
    pub temp: Comp<Temperature>,
    pub position: Comp<Position>,
}

impl Stars {
    pub(crate) fn insert(&mut self, id: Id<Star>, star: Star) {
        self.name.insert(id, star.name);
        self.mass.insert(id, star.mass);
        self.radius.insert(id, star.radius);
        self.temp.insert(id, star.temp);
        self.position.insert(id, star.position);
    }
}
