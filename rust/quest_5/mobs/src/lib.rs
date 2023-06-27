#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

mod mobs;
pub use mobs::Mob;
pub use mobs::member;
pub use mobs::boss;
