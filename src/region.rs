#![allow(dead_code)]

use super::*;
use crate::region::body_regions::BodyRegions;
use planetary_dynamics::adjacency::Adjacency;
use planetary_dynamics::terrain::Terrain;

// Thinking:
// - DONE: Regions of a body defined by an `IdRange`
// - DONE: Lat/Lon, neighbors determined by the range and the position within the range

#[derive(Debug, Copy, Clone)]
pub struct Region {
    pub terrain: Terrain,
}

fixed_id!(Region);

#[derive(Debug, Default)]
pub struct Regions {
    pub terrain: Component<Region, Terrain>,

    pub body: Component<Region, Id<Body>>,
    pub body_regions: BodyRegions,

    pub adjacency: Adjacency,
}

impl Regions {
    pub fn insert(
        &mut self,
        regions: Vec<Region>,
        body: Id<Body>,
        alloc: &mut RangeAllocator<Region>,
    ) -> IdRange<Region> {
        let id_range = alloc.create(regions.len());

        self.adjacency.register(regions.len());

        for (id, _region) in id_range.into_iter().zip(regions) {
            self.body_regions.link(body, id);
        }

        id_range
    }
}

pub mod body_regions {
    use super::*;
    use gen_id_component::Component;
    use std::ops::Index;

    #[derive(Debug, Default, Clone)]
    pub struct BodyRegions {
        targets: Component<Body, IdRange<Region>>,
        source: Component<Region, Id<Body>>,
    }

    impl BodyRegions {
        pub fn link(&mut self, body: Id<Body>, region: Id<Region>) {
            self.source.insert(region, body);
            if let Some(range) = self.targets.get_mut(body) {
                range.extend(region);
            } else {
                self.targets.insert(body, region.into());
            }
        }
    }

    impl Index<Id<Body>> for BodyRegions {
        type Output = IdRange<Region>;

        fn index(&self, index: Id<Body>) -> &Self::Output {
            self.targets.index(index)
        }
    }

    impl Index<Id<Region>> for BodyRegions {
        type Output = Id<Body>;

        fn index(&self, index: Id<Region>) -> &Self::Output {
            self.source.index(index)
        }
    }
}
