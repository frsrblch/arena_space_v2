use crate::*;

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
        let regions = &mut state.region;

        for stellar_system in self.systems {
            let star = alloc.star.create().value;
            stars.insert(star, stellar_system.star);

            for planet in stellar_system.planets {
                let planet_id = alloc.body.create().value;
                let links = BodyLinks { star, parent: None };
                let Planet {
                    body,
                    moons,
                    regions: planet_regions,
                } = planet;

                bodies.insert(planet_id, body, links);
                regions.insert(planet_regions, planet_id, &mut alloc.region);

                for moon in moons {
                    let moon_id = alloc.body.create().value;
                    let parent = Some(planet_id);
                    let links = BodyLinks { star, parent };
                    let Moon {
                        body,
                        regions: moon_regions,
                    } = moon;

                    bodies.insert(moon_id, body, links);
                    regions.insert(moon_regions, moon_id, &mut alloc.region);
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
    pub moons: Vec<Moon>,
    pub regions: Vec<Region>,
}

#[derive(Debug)]
pub struct Moon {
    pub body: Body,
    pub regions: Vec<Region>,
}

#[cfg(test)]
mod test {
    use super::*;
    use orbital_mechanics::EllipticalOrbit;

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
                    regions: vec![Region {
                        area: Default::default(),
                    }],
                    moons: vec![
                        Moon {
                            body: Body {
                                name: "Moon A".to_string(),
                                mass: Default::default(),
                                radius: Default::default(),
                                orbit: EllipticalOrbit::circular(
                                    Duration::in_days(1.0),
                                    Default::default(),
                                    Default::default(),
                                ),
                            },
                            regions: vec![Region {
                                area: Default::default(),
                            }],
                        },
                        Moon {
                            body: Body {
                                name: "Moon B".to_string(),
                                mass: Default::default(),
                                radius: Default::default(),
                                orbit: EllipticalOrbit::circular(
                                    Duration::in_days(2.0),
                                    Default::default(),
                                    Default::default(),
                                ),
                            },
                            regions: vec![Region {
                                area: Default::default(),
                            }],
                        },
                    ],
                }],
            }],
        }
        .create();

        drop(state);
        // panic!();
    }
}
