# ly-rustlib/lyrlib
Bunch of utils that I use.

## builder proc_macro
this macro automatically produces a "builder" for structs.
- Field Customization: The fields of the builder can be configured using attribtues.
    - `#[builder(skip)]`: skips this field in the builder
    - `#[builder(ty = Type)]`: sets the type of the field in the builder.
    - `#[builder(pass = /* attr */)]`: passes the `attr` into the builder field. 
    - `#[builder(skip_table)]`: passed the field to the builder and not the object.
    - `#[builder(skip_setter)]`: does not create a builder for a field.
    - `#[builder(init = /* code */)]`: it sets the default value of the field to `code`.
- Builder Customization: 
    - `#[builder(name = /* name */)]`: Sets the builder name to `name`
    - `#[builder(pass = /* outer attr */]`: passes the `outer attr` as an outer argument.

### Example
```Rust
// original:
#[builder(name = Builder, pass = derive(Debug, Default, Serialize, Deserialize))]
#[derive(Debug, Default, Clone)]
struct Table {
    #[builder(skip)]
    id: usize,
    #[builder(skip_table)]
    ident: String,
    #[builder(ty = String, pass = serde(skip_serializing_if = "String::is_empty"))]
    parent: Option<usize>,
    #[builder(init = String::from("data"))]
    data: String,
    #[builder(skip_setter)]
    complicated: String,
}
// generated:
#[derive(Debug, Default, Clone)]
struct Table {
    id: usize,
    parent: Option<usize>,
    data: String,
    complicated: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Builder {
    pub ident: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub parent: String,
    pub data: String,
    pub complicated: String,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            ident: Default::default(),
            parent: Default::default(),
            data: String::from("data"),
            complicated: Default::default(),
        }
    }
    pub fn ident(mut self, ident: String) -> Self {
        self.ident = ident;
        return self;
    }
    pub fn parent(mut self, parent: String) -> Self {
        self.parent = parent;
        return self;
    }
    pub fn data(mut self, data: String) -> Self {
        self.data = data;
        return self;
    }
}
```



## log module
this module contains a runtime logging system. It records the logs at 3 different levels, log, warning and error. using a global logger that can be modified at runtime.

### Features
- Supports 3 logging level, `log`, `warn`, `error`, that can be changed at runtime.
- Using the trait `Logger` allows the creation of a custom loggers
- Provides a global thread safe logger that can be changed into any custom logger
