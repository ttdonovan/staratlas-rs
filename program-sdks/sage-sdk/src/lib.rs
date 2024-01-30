use staratlas_sage::state;

use std::fmt;

pub mod utils;

pub struct Fleet(state::Fleet);

impl fmt::Debug for Fleet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Fleet").finish()
    }
}

pub struct Game(state::Game);

impl fmt::Debug for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Game").finish()
    }
}
