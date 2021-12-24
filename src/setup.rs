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

        for (StellarSystem { star, planets }, star_id) in self.systems.into_iter().zip(star_ids) {
            stars.insert(star_id, star);

            let body_count = planets.iter().map(|planet| 1 + planet.moons.len()).sum();
            let mut body_ids = alloc.body.create(body_count).into_iter();

            for Planet { body, moons, tiles } in planets {
                let planet_id = body_ids.next().expect("not enough body ids created");
                let links = BodyLinks {
                    star: star_id,
                    parent: None,
                };

                let tiles = tiles
                    .generate(body.radius, &regions.adjacency, rng)
                    .into_iter()
                    .map(|terrain| Region { terrain })
                    .collect();

                bodies.insert(planet_id, body, links);
                regions.insert(tiles, planet_id, &mut alloc.region);

                let links = BodyLinks {
                    star: star_id,
                    parent: Some(planet_id),
                };
                for Moon { body, tiles } in moons {
                    let moon_id = body_ids.next().expect("not enough body ids created");

                    let tiles = tiles
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
    pub tiles: TileGen,
}

#[derive(Debug)]
pub struct Moon {
    pub body: Body,
    pub tiles: TileGen,
}

#[cfg(test)]
mod test {
    use super::*;
    use orbital_mechanics::{EllipticalOrbit, Rotation};
    use rand::prelude::{Rng, SeedableRng, SmallRng};

    #[allow(dead_code)]
    fn setup() -> SystemState {
        let rng = &mut SmallRng::seed_from_u64(0);

        let m_p = 6e24 * KG;

        let m1 = 7e22 * KG;
        let m2 = 4e22 * KG;

        let r1 = 1700.0 * KM;
        let r2 = 1500.0 * KM;

        let or1 = 200e3 * KM;
        let or2 = 300e3 * KM;

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
                        mass: m_p,
                        radius: 6300.0 * KM,
                        orbit: EllipticalOrbit::circular(
                            Duration::in_d(1.0),
                            Default::default(),
                            Default::default(),
                        ),
                        rotation: Rotation::new(
                            Duration::in_d(1.0),
                            Angle::in_deg(20.0),
                            rng.gen(),
                        ),
                    },
                    tiles: TileGen {
                        water_fraction: 0.7,
                    },
                    moons: vec![
                        Moon {
                            body: Body {
                                name: "Moon A".to_string(),
                                mass: m1,
                                radius: r1,
                                orbit: EllipticalOrbit::circular_from_parent(m_p, or1, rng.gen()),
                                rotation: Rotation::new(
                                    Duration::of_orbit(or1, m_p),
                                    Angle::zero(),
                                    Angle::zero(),
                                ),
                            },
                            tiles: TileGen::default(),
                        },
                        Moon {
                            body: Body {
                                name: "Moon B".to_string(),
                                mass: m2,
                                radius: r2,
                                orbit: EllipticalOrbit::circular_from_parent(m_p, or2, rng.gen()),
                                rotation: Rotation::new(
                                    Duration::of_orbit(or2, m_p),
                                    Angle::zero(),
                                    Angle::zero(),
                                ),
                            },
                            tiles: TileGen::default(),
                        },
                    ],
                }],
            }],
        }
        .create()
    }
}
