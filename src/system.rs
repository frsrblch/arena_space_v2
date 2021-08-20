#![allow(dead_code)]

use crate::state::State;
use physics_types::DateTime;

#[derive(Debug)]
pub struct SystemState {
    pub state: State,
    pub queue: SystemQueue,
}

#[derive(Debug)]
pub struct SystemQueue;

impl SystemQueue {
    pub(crate) fn new(_start: DateTime) -> Self {
        Self
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Systems {}

impl Systems {
    const LEN: usize = 0;

    pub fn index(self) -> usize {
        todo!()
    }
}
