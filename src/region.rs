#![allow(dead_code)]

use super::*;

// Thinking:
// - Regions of a body defined by an `IdRange`
// - Lat/Lon, neighbors determined by the range and the position within the range

pub struct Region {
    pub area: Area,
}

pub enum RegionType {}
