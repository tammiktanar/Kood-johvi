// use std::ops::{Deref, DerefMut, Mul};
//
// struct Sq<T> {
//     value: T,
//     square: Option<T>,
// }
//
// impl<T> Deref for Sq<T> {
//     type Target = T;
//
//     fn deref(&self) -> &Self::Target {
//         &self.value
//     }
// }
//
// impl<T> DerefMut for Sq<T> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.value
//     }
// }
//
// impl<T> Sq<T> {
//     fn new(value: T) -> Self {
//         Self {
//             value,
//             square: None,
//         }
//     }
//
//     fn take_value(self) -> T {
//         self.value
//     }
// }
//
// impl<'a, T> Sq<T>
// where &'a T: Mul<Output=T> + 'a
// {
//     fn square(&mut self) -> &T {
//         &self.square.get_or_insert_with(|| &self.value * &self.value)
//     }
//
//     fn take_square(self) -> T {
//         self.square.unwrap_or_else(|| &self.value * &self.value)
//     }
// }
//
// impl<T> From<T> for Sq<T> {
//     fn from(value: T) -> Self {
//         Self::new(value)
//     }
// }
//
