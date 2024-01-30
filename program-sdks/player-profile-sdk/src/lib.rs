pub use staratlas_player_profile::{accounts, instruction, program, state, typedefs};

use std::fmt;

pub mod utils;

pub struct Profile(state::Profile);

impl fmt::Debug for Profile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Profile")
            .field("vesion", &self.0.version)
            .field("auth_key_count", &self.0.auth_key_count)
            .field("auth_key_threshold", &self.0.key_threshold)
            .field("next_seq_id", &self.0.next_seq_id)
            .field("created_at", &self.0.created_at)
            .finish()
    }
}
