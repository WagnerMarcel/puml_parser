use clang::*;

use super::kind::*;

use super::super::utils::macros::*;

#[derive(Default)]
pub struct Element {
    name: String,
    kind: Kind,
    type_: String,
    visibility: Option<char>,
    children: Vec<Element>,
    // extensions: Vec<Element>,
    // compositions: Vec<Element>,
    aggregations: Vec<Element>,
}

impl Element {
    pub fn create(&mut self, entity: Entity) {
        self.name = entity.get_name().unwrap_or("".to_string());
        self.kind = Self::map_kind(entity.get_kind());
        self.visibility = Self::get_accessibility_character(entity.get_accessibility());
        self.type_ = match entity.get_type() {
            Some(type_) => type_.get_display_name(),
            None => String::new(),
        };

        for child in entity.get_children() {
            let mut element = Element::default();
            element.create(child);

            if element.kind.is_value() {
                self.children.push(element);
            } else if !element.kind.is_empty() {
                self.aggregations.push(element);
            } else {
                // Skip
            }
        }
    }

    fn map_kind(kind: EntityKind) -> Kind {
        match kind {
            EntityKind::ClassDecl => Kind::Label(String::from("class")),
            EntityKind::StructDecl => Kind::Label(String::from("struct")),
            EntityKind::EnumDecl => Kind::Label(String::from("enum")),
            EntityKind::FieldDecl => Kind::FieldDecl,
            EntityKind::Method => Kind::Method,
            EntityKind::EnumConstantDecl => Kind::EnumConstantDecl,
            kind => {
                warn_unimplemented!(format!("{:?}", kind));
                Kind::None
            }
        }
    }

    fn get_accessibility_character(accessibility: Option<Accessibility>) -> Option<char> {
        match accessibility {
            Some(Accessibility::Private) => Some('-'),
            Some(Accessibility::Protected) => Some('#'),
            Some(Accessibility::Public) => Some('+'),
            None => None,
        }
    }
}

impl Element {
    pub fn get(&self) -> Vec<String> {
        let mut vec: Vec<String> = vec![];

        let mut own = self.get_element();
        vec.append(&mut own);

        for aggregate in &self.aggregations {
            let mut other = aggregate.get();

            vec.append(&mut other);

            if !self.type_.is_empty() && !aggregate.type_.is_empty() {
                vec.push(format!("{} o-- {}", self.name, aggregate.name))
            }
        }

        vec
    }

    fn get_element(&self) -> Vec<String> {
        let mut vec: Vec<String> = vec![];

        if self.kind.is_label() {
            vec.push(format!("{} {} {{", self.kind.value(), self.name));
            for child in &self.children {
                let visibility = match child.visibility {
                    Some(visibility) => visibility.to_string(),
                    None => String::new(),
                };

                match child.kind {
                    Kind::FieldDecl => {
                        vec.push(format!("{}{} : {}", visibility, child.name, child.type_))
                    }
                    Kind::Method => vec.push(format!("{}{}()", visibility, child.name)),
                    Kind::EnumConstantDecl => vec.push(format!("{}", child.name)),
                    _ => {}
                }
            }
            vec.push("}".to_string());
        }

        vec
    }
}
