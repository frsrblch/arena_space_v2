use crate::body::{Body, BodyLinks};
use crate::star::Star;
use crate::state::State;
use crate::system::{SystemQueue, SystemState};
use physics_types::DateTime;

#[derive(Debug)]
pub struct Setup {
    pub systems: Vec<StellarSystem>,
}

impl Setup {
    pub fn create(self) -> SystemState {
        let start = DateTime::parse_from_str("2050-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let mut state = State::default();

        let alloc = &mut state.allocators;
        let stars = &mut state.star;
        let bodies = &mut state.body;

        for stellar_system in self.systems {
            let star = alloc.star.create().value;
            stars.insert(star, stellar_system.star);

            for planetary_system in stellar_system.planets {
                let planet = alloc.body.create().value;
                let links = BodyLinks { star, parent: None };
                bodies.insert(planet, planetary_system.body, links);

                for moon in planetary_system.moons {
                    let id = alloc.body.create().value;
                    let parent = Some(planet);
                    let links = BodyLinks { star, parent };
                    bodies.insert(id, moon, links);
                }
            }
        }

        SystemState {
            state,
            queue: SystemQueue::new(start),
        }
    }
}

#[derive(Debug)]
pub struct StellarSystem {
    pub star: Star,
    pub planets: Vec<Planet>,
}

#[derive(Debug)]
pub struct Planet {
    pub body: Body,
    pub moons: Vec<Body>,
}

#[cfg(test)]
mod test {
    use super::*;
    use orbital_mechanics::EllipticalOrbit;
    use physics_types::*;

    #[test]
    fn setup() {
        let state = Setup {
            systems: vec![StellarSystem {
                star: Star {
                    name: "Rigel".to_string(),
                    mass: Default::default(),
                    radius: Default::default(),
                    temp: Default::default(),
                    position: Default::default(),
                },
                planets: vec![Planet {
                    body: Body {
                        name: "Planet".to_string(),
                        mass: Default::default(),
                        radius: Default::default(),
                        orbit: EllipticalOrbit::circular(
                            Duration::in_days(1.0),
                            Default::default(),
                            Default::default(),
                        ),
                    },
                    moons: vec![
                        Body {
                            name: "Moon A".to_string(),
                            mass: Default::default(),
                            radius: Default::default(),
                            orbit: EllipticalOrbit::circular(
                                Duration::in_days(1.0),
                                Default::default(),
                                Default::default(),
                            ),
                        },
                        Body {
                            name: "Moon B".to_string(),
                            mass: Default::default(),
                            radius: Default::default(),
                            orbit: EllipticalOrbit::circular(
                                Duration::in_days(2.0),
                                Default::default(),
                                Default::default(),
                            ),
                        },
                    ],
                }],
            }],
        }
        .create();

        // panic!();
    }
}
