#[derive(Default, PartialEq, Debug)]
pub enum Kind {
    #[default]
    None,
    Label(String),
    Namespace,
    FieldDecl,
    Method,
    EnumConstantDecl,
    BaseSpecifier,
}

impl Kind {
    pub fn is_empty(&self) -> bool {
        &Self::None == self
    }

    pub fn is_label(&self) -> bool {
        matches!(self, Self::Label(_))
    }

    pub fn is_value(&self) -> bool {
        !self.is_empty() && !self.is_label() && !self.is_namespace() && !self.is_base()
    }

    pub fn is_namespace(&self) -> bool {
        matches!(self, Self::Namespace)
    }

    pub fn is_base(&self) -> bool {
        matches!(self, Self::BaseSpecifier)
    }

    pub fn value(&self) -> String {
        match self {
            Self::Label(value) => value.to_string(),
            _ => String::new(),
        }
    }
}
