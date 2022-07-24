#[derive(Default, Debug, PartialEq, Clone, Copy, Eq, PartialOrd, Ord)]
pub struct KeydownMap {
    pub w: bool,
    pub a: bool,
    pub s: bool,
    pub d: bool,
    pub space: bool,
    pub shift: bool,
}

impl KeydownMap {
    pub fn all_false(&self) -> bool {
        !self.w && !self.a && !self.s && !self.d && !self.space && !self.shift
    }
}
