use gen_id_allocator::*;
use gen_id_component::Component;
use physics_types::*;

use crate::body::{Bodies, Body, BodyLinks};
use crate::region::{Region, Regions};
use crate::star::{Star, Stars};
use crate::state::State;
use crate::system::{SystemQueue, SystemState};

pub use setup::*;

mod setup;
mod state;
mod system;

mod body;
mod region;
mod star;
