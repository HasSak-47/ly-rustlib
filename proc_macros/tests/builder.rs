use util_macros::builder;

fn function() -> String{
    "test".to_string()
}

#[builder(Test)]
#[derive(Debug, Default)]
struct TestStruct{
    a: Option<i32>,
    #[builder(default = function)]
    b: String,

    c: (u32, u32),

    #[builder(skip)]
    id: usize,
}


#[test]
fn test_builder(){
    #[builder(A)]
    #[derive(Debug)]
    struct TestStructA{
        pub a: (u32, u32),
    }
}
