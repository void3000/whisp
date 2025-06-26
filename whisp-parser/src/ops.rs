#[derive(Clone, Debug, PartialEq)]
pub enum Operation {
    Add, Sub, Mul, Div, Mod,
    Eq, Lt, Gt, Le, Ge,
    And, Or
}
