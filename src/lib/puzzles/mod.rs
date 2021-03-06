pub mod puzzle01;
pub mod puzzle02;
pub mod puzzle03;
pub mod puzzle04;
pub mod puzzle05;
pub mod puzzle06;
pub mod puzzle07;
pub mod puzzle08;
pub mod puzzle09;

pub trait Puzzle<T> {
    fn build(input: String) -> Self;
    fn solve(&self) -> T;
}
