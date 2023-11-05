use derive_more::Constructor;
use serde::{Deserialize, Serialize};

// constructor macro makes the new method on the struct and returns self with data
#[derive(Clone, Constructor, Debug, Deserialize, Serialize)]
pub struct Hits(u64);

impl Hits {
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
