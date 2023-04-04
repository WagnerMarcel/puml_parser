use clang::*;

use super::super::utils::macros::*;

#[derive(Default)]
pub struct Element {
    name: String,
    kind: String,
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

            if element.kind == "field" || element.kind == "method" {
                self.children.push(element);
            } else if !element.kind.is_empty() {
                self.aggregations.push(element);
            } else {
                // Skip
            }
        }
    }

    fn map_kind(kind: EntityKind) -> String {
        match kind {
            EntityKind::ClassDecl => "class".to_string(),
            EntityKind::StructDecl => "struct".to_string(),
            EntityKind::FieldDecl => "field".to_string(),
            EntityKind::Method => "method".to_string(),
            kind => {
                warn_unimplemented!(format!("{:?}", kind));
                String::new()
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

        if !self.kind.is_empty() {
            vec.push(format!("{} {} {{", self.kind, self.name));
            for child in &self.children {
                if child.kind == "field" {
                    match child.visibility {
                        Some(visibility) => {
                            vec.push(format!("{}{} : {}", visibility, child.name, child.type_))
                        }
                        None => vec.push(format!("{} : {}", child.name, child.type_)),
                    }
                } else if child.kind == "method" {
                    match child.visibility {
                        Some(visibility) => vec.push(format!("{}{}()", visibility, child.name)),
                        None => vec.push(format!("{}()", child.name)),
                    }
                }
            }
            vec.push("}".to_string());
        }

        vec
    }
}
