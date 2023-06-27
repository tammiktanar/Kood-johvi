#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}


#[derive(Debug, PartialEq, Eq)]
pub struct Student (pub i32, pub String, pub String);

pub fn id(student: &Student) -> i32 {
    return student.0.clone();
}

pub fn first_name(student: &Student) -> String {
    return student.1.clone();
}

pub fn last_name(student: &Student) -> String {
    return student.2.clone();
}