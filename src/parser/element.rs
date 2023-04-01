#[derive(Default)]
pub struct Element {
    name: String,
    type_: String,
    extensions: Vec<Element>,
    compositions: Vec<Element>,
    aggregations: Vec<Element>,
}

impl Element {
    fn create() {}
}
