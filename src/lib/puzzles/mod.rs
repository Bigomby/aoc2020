pub mod puzzle01;
pub mod puzzle02;
pub mod puzzle03;

pub trait Puzzle<T> {
    fn build(input: String) -> Self;
    fn solve(&self) -> T;
}
