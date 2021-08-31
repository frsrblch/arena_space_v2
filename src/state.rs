use crate::*;

#[derive(Debug, Default)]
pub struct Allocators {
    pub star: RangeAllocator<Star>,
    pub body: RangeAllocator<Body>,
    pub region: RangeAllocator<Region>,
}

#[derive(Debug, Default)]
pub struct State {
    pub allocators: Allocators,
    pub star: Stars,
    pub body: Bodies,
    pub region: Regions,
}
