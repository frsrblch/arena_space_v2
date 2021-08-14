use crate::*;

#[derive(Debug, Default)]
pub struct Allocators {
    pub star: Allocator<Star>,
    pub body: Allocator<Body>,
}

#[derive(Debug, Default)]
pub struct State {
    pub allocators: Allocators,
    pub star: Stars,
    pub bodies: Stars,
}
