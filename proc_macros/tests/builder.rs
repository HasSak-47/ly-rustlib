use util_macros::Builder;

#[test]
fn test_builder(){
    fn function() -> String{
        "test".to_string()
    }
    
    #[derive(Builder)]
    #[derive(Debug, Default)]
    struct TestStruct{
        a: Option<i32>,
        #[builder(default = function)]
        b: String,
    
        c: (u32, u32),
    
        #[builder(skip)]
        id: usize,
    }
    
    #[derive(Debug)]
    struct TestStructA{
        a: (u32, u32),
    }
}
