mod environ;
mod eval;

use std::collections::HashMap;
use crate::frontend::Node;

struct Scope {
    var: HashMap<String, Node>
}