use std::fmt;

pub mod derive;
pub mod programs;

use programs::staratlas_player_profile::state;

pub struct Profile(pub state::Profile);

impl fmt::Debug for Profile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Profile")
            .field("version", &self.0.version)
            .field("auth_key_count", &self.0.auth_key_count)
            .field("key_threshold", &self.0.key_threshold)
            .field("next_seq_id", &self.0.next_seq_id)
            .field("created_at", &self.0.created_at)
            .finish()
    }
}
