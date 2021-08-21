use crate::body::star_bodies::StarBodies;
use crate::*;
use gen_id_relations::{RangeRelation, RangeRelations};
use orbital_mechanics::EllipticalOrbit;
use std::ops::Index;

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

    pub star_bodies: StarBodies,
    pub relation: RangeRelations<Body>,
}

impl Bodies {
    pub(crate) fn insert(&mut self, id: Id<Body>, body: Body, links: BodyLinks) {
        self.name.insert(id, body.name);
        self.mass.insert(id, body.mass);
        self.radius.insert(id, body.radius);
        self.orbit.insert(id, body.orbit);

        self.star_bodies.link(links.star, id);

        match links.parent {
            Some(parent) => self.relation.insert_child(id, parent),
            None => self.relation.insert_parent(id),
        }
    }

    pub fn orbit_distance(&self, body: Id<Body>, time: TimeFloat) -> Distance {
        match self.relation[body] {
            RangeRelation::ChildOf(parent) => {
                self.orbit[parent].distance(time) + self.orbit[body].distance(time)
            }
            RangeRelation::ParentOf(_) => self.orbit[body].distance(time),
        }
    }

    pub fn position(&self, body: Id<Body>, time: TimeFloat, stars: &Stars) -> Position {
        let star = self.star_bodies[body];
        stars.position[star] + self.orbit_distance(body, time)
    }

    pub fn planets(&self, star: Id<Star>) -> impl Iterator<Item = Id<Body>> + '_ {
        let bodies = self.star_bodies[star];
        self.relation.parents(bodies)
    }

    pub fn get_standard_name(&self, id: Id<Body>, stars: &Stars) -> String {
        let star = self.star_bodies[id];
        let star_name = &stars.name[star];

        match self.relation[id] {
            RangeRelation::ParentOf(_) => {
                // E.g., Sol III
                let p = self.planets(star).position(|p| p.eq(&id)).unwrap();
                let p = get_roman_numeral(p);
                format!("{} {}", star_name, p)
            }
            RangeRelation::ChildOf(parent) => {
                // E.g., Sol III-A
                let p = self.planets(star).position(|p| p.eq(&parent)).unwrap();
                let p = get_roman_numeral(p);

                let moons = self.relation[parent].parent_of().unwrap();
                let m = moons.position(id).unwrap();
                let m = get_abc_char(m);

                format!("{} {}-{}", star_name, p, m)
            }
        }
    }
}

fn get_roman_numeral(i: usize) -> &'static str {
    match i {
        0 => "I",
        1 => "II",
        2 => "III",
        3 => "IV",
        4 => "V",
        5 => "VI",
        6 => "VII",
        7 => "VIII",
        8 => "IX",
        9 => "X",
        10 => "XI",
        11 => "XII",
        _ => panic!("number > 11"),
    }
}

fn get_abc_char(i: usize) -> char {
    match i {
        0 => 'A',
        1 => 'B',
        2 => 'C',
        3 => 'D',
        4 => 'E',
        5 => 'F',
        6 => 'G',
        7 => 'H',
        8 => 'I',
        9 => 'J',
        _ => panic!("number > 9"),
    }
}

pub mod star_bodies {
    use super::*;
    use gen_id_component::Component;

    #[derive(Debug, Default, Clone)]
    pub struct StarBodies {
        targets: Component<Star, IdRange<Body>>,
        source: Component<Body, Id<Star>>,
    }

    impl StarBodies {
        pub fn link(&mut self, star: Id<Star>, body: Id<Body>) {
            self.source.insert(body, star);
            if let Some(range) = self.targets.get_mut(star) {
                range.extend(body);
            } else {
                self.targets.insert(star, body.into());
            }
        }
    }

    impl Index<Id<Star>> for StarBodies {
        type Output = IdRange<Body>;

        fn index(&self, index: Id<Star>) -> &Self::Output {
            self.targets.index(index)
        }
    }

    impl Index<Id<Body>> for StarBodies {
        type Output = Id<Star>;

        fn index(&self, index: Id<Body>) -> &Self::Output {
            self.source.index(index)
        }
    }
}
