use crate::state::State;

#[derive(Debug)]
pub struct SystemState {
    pub state: State,
    pub queue: SystemQueue,
}

#[derive(Debug)]
pub struct SystemQueue;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Systems {}

impl Systems {
    const LEN: usize = 0;

    pub fn index(self) -> usize {
        todo!()
    }
}
