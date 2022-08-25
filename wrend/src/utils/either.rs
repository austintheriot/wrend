#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Either<L, R> {
    A(L),
    B(R),
}

impl<L, R> Either<L, R> {
    pub fn new_a(a: L) -> Self {
        Either::A(a)
    }

    pub fn new_b(b: R) -> Self {
        Either::B(b)
    }

    pub fn is_a(&self) -> bool {
        match self {
            Either::A(_) => true,
            Either::B(_) => false,
        }
    }

    pub fn is_b(&self) -> bool {
        match self {
            Either::A(_) => false,
            Either::B(_) => true,
        }
    }

    pub fn a(self) -> Option<L> {
        match self {
            Either::A(a) => Some(a),
            Either::B(_) => None,
        }
    }

    pub fn b(self) -> Option<R> {
        match self {
            Either::A(_) => None,
            Either::B(b) => Some(b),
        }
    }

    pub fn unwrap_a(self) -> L {
        match self {
            Either::A(a) => a,
            Either::B(_) => panic!("called `Either::unwrap_a()` on a `B` value"),
        }
    }

    pub fn unwrap_b(self) -> R {
        match self {
            Either::A(_) => panic!("called `Either::unwrap_b()` on an `A` value"),
            Either::B(b) => b,
        }
    }
}
