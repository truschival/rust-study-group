use std::cmp::Ordering::Equal;

pub struct MyStruct {
    value: i32,
}

impl MyStruct {
    pub fn create(value: i32) -> MyStruct {
        MyStruct { value }
    }

    pub fn get_value(&self) -> i32 {
        self.value
    }
}

// Ich versteh lifetime und referneces nicht!
// das kompiliert nicht wegen lifetime:
//pub fn return_err_for_15(arg: i32) -> Result<MyStruct, &str> {
// das kompiliert nicht wegen unknown
//pub fn return_err_for_15(arg: i32) -> Result<MyStruct, str> {
pub fn return_err_for_15(arg: &i32) -> Result<MyStruct, &str> {
    match arg.cmp(&15) {
        Equal => Err("This is bad!"),
        _ => Ok(MyStruct::create(arg * 2)),
    }
}

pub fn return_none_for_13(arg: &i32) -> Option<MyStruct> {
    match arg.cmp(&13) {
        Equal => None,
        _ => Some(MyStruct::create(arg * 3)),
    }
}

pub fn i_can_use_question_mark(arg: i32) -> Option<i32> {
    let val = return_none_for_13(&arg)?.get_value();
    Some(val)
}

pub fn i_can_use_question_mark_for_result(arg: &i32) -> Result<i32, &str> {
    let val = return_err_for_15(arg)?.get_value();
    Ok(val)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_panic_from_unwarp_err() {
        return_err_for_15(&15).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_panic_from_unwarp_result() {
        return_none_for_13(&13).unwrap();
    }

    #[test]
    fn test_question_mark_none() {
        assert_eq!(i_can_use_question_mark(13), None);
    }

    #[test]
    fn test_question_mark_some() {
        let res = i_can_use_question_mark(2);
        assert!(res.is_some());
        assert_eq!(res.expect("should be 2*3"), 6);
    }

    #[test]
    #[should_panic]
    fn test_expect_panics() {
        let res = i_can_use_question_mark(13);
        res.expect("will panic on 13");
    }
}
