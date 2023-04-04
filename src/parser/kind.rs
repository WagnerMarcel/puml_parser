#[derive(Default, PartialEq)]
pub enum Kind {
    #[default]
    None,
    Label(String),
    FieldDecl,
    Method,
    EnumConstantDecl,
}

impl Kind {
    pub fn is_empty(&self) -> bool {
        &Self::None == self
    }

    pub fn is_label(&self) -> bool {
        matches!(self, Self::Label(_))
    }

    pub fn is_value(&self) -> bool {
        !self.is_empty() && !self.is_label()
    }

    pub fn value(&self) -> String {
        match self {
            Self::Label(value) => value.to_string(),
            _ => String::new(),
        }
    }
}
