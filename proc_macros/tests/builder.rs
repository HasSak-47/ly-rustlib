use util_macros::Builder;

#[test]
fn test_setter(){
    #[derive(Builder)]
    struct TestStruct{
        a: Option<i32>,
        b: String,
    }
}
