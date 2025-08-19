use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[derive(PartialEq)]
pub enum PlayMode {
    Repeat,
    Random,
    Single,
    // Sequential there is no need to add, use Repeat instead.
}
