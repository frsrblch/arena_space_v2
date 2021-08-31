use crate::*;

#[derive(Debug, Default)]
pub struct Allocators {
    pub star: Allocator<Star>,
    pub body: Allocator<Body>,
    pub region: RangeAllocator<Region>,
}

#[derive(Debug, Default)]
pub struct State {
    pub allocators: Allocators,
    pub star: Stars,
    pub body: Bodies,
    pub region: Regions,
}
