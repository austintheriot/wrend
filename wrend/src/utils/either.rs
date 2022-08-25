#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Either<L: ?Sized, R: ?Sized> {
    Left(Box<L>),
    Right(Box<R>),
}

impl<L: ?Sized, R: ?Sized> Either<L, R> {
    pub fn new_left(left: Box<L>) -> Self {
        Either::Left(left)
    }

    pub fn new_right(right: Box<R>) -> Self {
        Either::Right(right)
    }

    pub fn is_left(&self) -> bool {
        match self {
            Either::Left(_) => true,
            Either::Right(_) => false,
        }
    }

    pub fn is_right(&self) -> bool {
        match self {
            Either::Left(_) => false,
            Either::Right(_) => true,
        }
    }

    pub fn left(self) -> Option<Box<L>> {
        match self {
            Either::Left(left) => Some(left),
            Either::Right(_) => None,
        }
    }

    pub fn right(self) -> Option<Box<R>> {
        match self {
            Either::Left(_) => None,
            Either::Right(right) => Some(right),
        }
    }

    pub fn unwrap_left(self) -> Box<L> {
        match self {
            Either::Left(left) => left,
            Either::Right(_) => panic!("called `Either::unwrap_left()` on a `Right` value"),
        }
    }

    pub fn unwrap_right(self) -> Box<R> {
        match self {
            Either::Left(_) => panic!("called `Either::unwrap_left()` on a `Left` value"),
            Either::Right(right) => right,
        }
    }
}
