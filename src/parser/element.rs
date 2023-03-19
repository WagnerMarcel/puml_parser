pub struct Element {
    name: String,
    type_: String,
    extensions: Vec<Element>,
    compositions: Vec<Element>,
    aggregations: Vec<Element>,
}

impl Default for Element {
    fn default() -> Self {
        Self {
            name: Default::default(),
            type_: Default::default(),
            extensions: Default::default(),
            compositions: Default::default(),
            aggregations: Default::default(),
        }
    }
}

impl Element {
    fn create() {}
}
