#[derive(Clone, Debug)]
pub enum Operation {
    Add, Sub, Mul, Div,
    Eq, Lt, Gt, Le, Ge,
    And, Or
}
