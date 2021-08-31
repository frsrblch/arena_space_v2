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

        let star_count = self.systems.len();
        let star_ids = alloc.star.create(star_count);

        for (stellar_system, star_id) in self.systems.into_iter().zip(star_ids) {
            let StellarSystem { star, planets } = stellar_system;

            stars.insert(star_id, star);

            let body_count = planets.iter().map(|planet| 1 + planet.moons.len()).sum();
            let mut body_ids = alloc.body.create(body_count).into_iter();

            for planet in planets {
                let planet_id = body_ids.next().expect("not enough body ids created");
                let links = BodyLinks {
                    star: star_id,
                    parent: None,
                };
                let Planet {
                    body,
                    moons,
                    planet_regions,
                } = planet;

                bodies.insert(planet_id, body, links);
                regions.insert(planet_regions, planet_id, &mut alloc.region);

                for moon in moons {
                    let moon_id = body_ids.next().expect("not enough body ids created");
                    let parent = Some(planet_id);
                    let links = BodyLinks {
                        star: star_id,
                        parent,
                    };
                    let Moon { body, moon_regions } = moon;

                    bodies.insert(moon_id, body, links);
                    regions.insert(moon_regions, moon_id, &mut alloc.region);
                }
            }

            debug_assert_eq!(None, body_ids.next(), "too many body ids created");
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
    pub planet_regions: Vec<Region>,
}

#[derive(Debug)]
pub struct Moon {
    pub body: Body,
    pub moon_regions: Vec<Region>,
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
                    planet_regions: vec![Region {
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
                            moon_regions: vec![Region {
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
                            moon_regions: vec![Region {
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
