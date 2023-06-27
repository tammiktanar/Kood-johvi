#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

pub fn fibonacci(n: u32) -> u32 {
	match n {
		0     => 0,
		1 | 2 => 1,
		3     => 2,

		_     => fibonacci(n - 1) + fibonacci(n - 2)
	}
}