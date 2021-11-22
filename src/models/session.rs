use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::User;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Sessions {
    pub map: HashMap<String, User>,
}
