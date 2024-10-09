use util_macros::builder;

// original:
#[derive(Debug, Default, Clone)]
struct TestStruct {
    #[builder(skip)]
    id1: usize,
    #[builder(init = String::from("test"))]
    data: String,
    #[builder(ty = Option<i32>, init = Some(10))]
    id2: usize,
    #[builder(pass = serde(skip_serializing_if = "String::is_empty"))]
    string: String,
}

// parsed:
#[derive(Debug, Default, Clone)]
struct TestStruct {
    id1: usize,
    data: String,
    id2: usize,
    string: String,
}
#[derive(Debug, Default)]
struct Test {
    pub data: String,
    pub id2: Option<i32>,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub string: String,
}
impl Test {
    pub fn new() -> Self {
        Self {
            data: String::from("test"),
            id2: Some(10),
            string: Default::default(),
        }
    }
    pub fn data(mut self, data: String) -> Self {
        self.data = data;
        return self;
    }
    pub fn id2(mut self, id2: Option<i32>) -> Self {
        self.id2 = id2;
        return self;
    }
    pub fn string(mut self, string: String) -> Self {
        self.string = string;
        return self;
    }
}
