#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Matrix (pub (i32, i32), pub (i32, i32));

pub fn transpose(m: Matrix) -> Matrix {
    let res = Matrix ((m.0.0, m.1.0), (m.0.1, m.1.1));
    return res
}