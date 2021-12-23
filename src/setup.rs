use crate::*;
use planetary_dynamics::tile_gen::TileGen;
use rand::thread_rng;

#[derive(Debug)]
pub struct Setup {
    pub systems: Vec<StellarSystem>,
}

impl Setup {
    pub fn create(self) -> SystemState {
        let rng = &mut thread_rng();

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

                let tiles = planet_regions
                    .generate(body.radius, &regions.adjacency, rng)
                    .into_iter()
                    .map(|terrain| Region { terrain })
                    .collect();

                bodies.insert(planet_id, body, links);
                regions.insert(tiles, planet_id, &mut alloc.region);

                for moon in moons {
                    let moon_id = body_ids.next().expect("not enough body ids created");
                    let links = BodyLinks {
                        star: star_id,
                        parent: Some(planet_id),
                    };
                    let Moon { body, moon_regions } = moon;

                    let tiles = moon_regions
                        .generate(body.radius, &regions.adjacency, rng)
                        .into_iter()
                        .map(|terrain| Region { terrain })
                        .collect();

                    bodies.insert(moon_id, body, links);
                    regions.insert(tiles, moon_id, &mut alloc.region);
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
    pub planet_regions: TileGen,
}

#[derive(Debug)]
pub struct Moon {
    pub body: Body,
    pub moon_regions: TileGen,
}

#[cfg(test)]
mod test {
    use super::*;
    use orbital_mechanics::EllipticalOrbit;

    #[allow(dead_code)]
    fn setup() -> SystemState {
        let r1 = 6300.0 * KM;

        Setup {
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
                        mass: 5.972e24 * KG,
                        radius: r1,
                        orbit: EllipticalOrbit::circular(
                            Duration::in_d(1.0),
                            Default::default(),
                            Default::default(),
                        ),
                    },
                    planet_regions: TileGen {
                        water_fraction: 0.7,
                    },
                    moons: vec![
                        Moon {
                            body: Body {
                                name: "Moon A".to_string(),
                                mass: Default::default(),
                                radius: Default::default(),
                                orbit: EllipticalOrbit::circular(
                                    Duration::in_d(1.0),
                                    Default::default(),
                                    Default::default(),
                                ),
                            },
                            moon_regions: TileGen {
                                water_fraction: 0.0,
                            },
                        },
                        Moon {
                            body: Body {
                                name: "Moon B".to_string(),
                                mass: Default::default(),
                                radius: Default::default(),
                                orbit: EllipticalOrbit::circular(
                                    Duration::in_d(2.0),
                                    Default::default(),
                                    Default::default(),
                                ),
                            },
                            moon_regions: TileGen {
                                water_fraction: 0.0,
                            },
                        },
                    ],
                }],
            }],
        }
        .create()
    }
}
