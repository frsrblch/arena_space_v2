#![allow(dead_code)]

use super::*;
use crate::region::body_regions::BodyRegions;

// Thinking:
// - Regions of a body defined by an `IdRange`
// - Lat/Lon, neighbors determined by the range and the position within the range

#[derive(Debug, Copy, Clone)]
pub struct Region {
    pub area: Area,
}

fixed_id!(Region);

pub enum RegionType {}

#[derive(Debug, Default)]
pub struct Regions {
    pub area: Component<Region, Area>,
    pub body: Component<Region, Id<Body>>,
    pub body_regions: BodyRegions,
}

impl Regions {
    pub fn insert(
        &mut self,
        regions: Vec<Region>,
        body: Id<Body>,
        alloc: &mut RangeAllocator<Region>,
    ) -> IdRange<Region> {
        let id_range = alloc.create(regions.len());

        for (region, id) in regions.into_iter().zip(id_range) {
            self.area.insert(id, region.area);
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
