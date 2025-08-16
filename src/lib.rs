pub mod arithmetic;
pub mod executable;
pub mod customio;

// a trivial wrapper for two elements of the same type
pub struct Duo<T> {
    pub first: T, pub second: T,
}

// a trivial wrapper for three elements of the same type
pub struct Trio<T> {
    pub first: T, pub second: T, pub third: T,
}


