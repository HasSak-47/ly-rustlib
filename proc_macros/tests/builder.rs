use util_macros::Builder;

fn function() -> String{
    "test".to_string()
}

#[derive(Builder)]
struct TestStruct{
    a: Option<i32>,
    #[builder(default = function)]
    b: String,

    #[builder(fake)]
    c: (u32, u32),

    #[builder(skip)]
    id: usize,
}

#[derive(Debug)]
struct TestStructA{
    a: (u32, u32),
}

fn test_setter(){
}
