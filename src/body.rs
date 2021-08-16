use crate::*;
use gen_id_relations::{Relation, Relations};
use orbital_mechanics::EllipticalOrbit;

type Comp<T> = Component<Body, T>;

#[derive(Debug, Clone)]
pub struct Body {
    pub name: String,
    pub mass: Mass,
    pub radius: Length,
    pub orbit: EllipticalOrbit,
}

fixed_id!(Body);

#[derive(Debug, Copy, Clone)]
pub struct BodyLinks {
    pub star: Id<Star>,
    pub parent: Option<Id<Body>>,
}

#[derive(Debug, Default)]
pub struct Bodies {
    pub name: Comp<String>,
    pub mass: Comp<Mass>,
    pub radius: Comp<Length>,
    pub orbit: Comp<EllipticalOrbit>,

    pub star: Comp<Id<Star>>,
    pub relation: Relations<Body>,
}

impl Bodies {
    pub(crate) fn insert(&mut self, id: Id<Body>, body: Body, links: BodyLinks) {
        self.name.insert(id, body.name);
        self.mass.insert(id, body.mass);
        self.radius.insert(id, body.radius);
        self.orbit.insert(id, body.orbit);

        self.star.insert(id, links.star);

        match links.parent {
            Some(parent) => self.relation.insert_child(id, parent),
            None => self.relation.insert_parent(id),
        }
    }

    pub fn orbit_distance(&self, body: Id<Body>, time: TimeFloat) -> Distance {
        match self.relation[body] {
            Relation::ChildOf(parent) => {
                self.orbit[parent].distance(time) + self.orbit[body].distance(time)
            }
            Relation::ParentOf(_) => self.orbit[body].distance(time),
        }
    }

    pub fn position(&self, body: Id<Body>, time: TimeFloat, stars: &Stars) -> Position {
        stars.position[self.star[body]] + self.orbit_distance(body, time)
    }
}
