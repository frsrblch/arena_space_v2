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

    pub fn orbit_position(&self, body: Id<Body>, time: TimeFloat) -> Position {
        match self.relation[body] {
            RangeRelation::ChildOf(parent) => {
                self.orbit[parent].distance(time) + self.orbit[body].distance(time)
            }
            RangeRelation::ParentOf(_) => self.orbit[body].distance(time),
        }
        .into()
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
                let planet = self.planets(star).position(|p| p.eq(&id)).unwrap();
                get_standard_name(star_name, planet, None)
            }
            RangeRelation::ChildOf(parent) => {
                let planet = self.planets(star).position(|p| p.eq(&parent)).unwrap();

                let moons = self.relation[parent].parent_of().unwrap();
                let moon = moons.position(id).unwrap();

                get_standard_name(star_name, planet, Some(moon))
            }
        }
    }
}

pub fn get_standard_name(star: &str, planet: usize, moon: Option<usize>) -> String {
    let planet = get_roman_numeral(planet);
    if let Some(moon) = moon {
        // E.g., Sol III-A
        let moon = get_abc_char(moon);
        format!("{} {}-{}", star, planet, moon)
    } else {
        // E.g., Sol III
        format!("{} {}", star, planet)
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
    let c = (i as u8 + b'A') as char;
    debug_assert!(c <= 'Z');
    c
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_standard_names() {
        assert_eq!("Rigel IV", get_standard_name("Rigel", 3, None));
        assert_eq!("Rigel IV-C", get_standard_name("Rigel", 3, Some(2)));
    }
}
