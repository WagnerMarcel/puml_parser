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
    extensions: Vec<Element>,
    // compositions: Vec<Element>,
    aggregations: Vec<Element>,
    dependencies: Vec<String>,
}

impl Element {
    pub fn create(&mut self, entity: Entity) {
        self.name = entity.get_name().unwrap_or("".to_string());
        self.kind = Self::map_kind(entity.get_kind());
        self.visibility = Self::get_accessibility_character(entity.get_accessibility());
        self.type_ = Self::extract_type(entity, self.kind.is_base());

        for child in entity.get_children() {
            let mut element = Element::default();
            element.create(child);

            if element.kind.is_value() {
                self.children.push(element);
            } else if element.kind.is_base() {
                self.extensions.push(element);
            } else if !element.kind.is_empty() {
                self.aggregations.push(element);
            } else {
                // Skip
            }

            if child.get_arguments().is_some() {
                let arguments = child.get_arguments().unwrap();
                for argument in arguments {
                    self.dependencies.push(
                        Self::extract_type(argument, false)
                            .trim_end_matches('&')
                            .trim()
                            .to_string(),
                    )
                }
                self.dependencies.sort();
                self.dependencies.dedup();
            }
        }
    }

    fn extract_type(entity: Entity, is_base: bool) -> String {
        match entity.get_type() {
            Some(type_) => {
                if !is_base {
                    type_
                        .get_display_name()
                        .split("::")
                        .collect::<Vec<&str>>()
                        .last()
                        .unwrap_or(&"")
                        .to_string()
                } else {
                    type_.get_display_name().replace("::", ".")
                }
            }
            None => String::new(),
        }
    }

    fn map_kind(kind: EntityKind) -> Kind {
        match kind {
            EntityKind::ClassDecl => Kind::Label(String::from("class")),
            EntityKind::StructDecl => Kind::Label(String::from("class")),
            EntityKind::EnumDecl => Kind::Label(String::from("enum")),
            EntityKind::Namespace => Kind::Namespace,
            EntityKind::FieldDecl => Kind::FieldDecl,
            EntityKind::Method => Kind::Method,
            EntityKind::EnumConstantDecl => Kind::EnumConstantDecl,
            EntityKind::BaseSpecifier => Kind::BaseSpecifier,
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

        let (mut own, mut end) = self.get_element();
        vec.append(&mut own);

        for aggregate in &self.aggregations {
            let mut other = aggregate.get();

            vec.append(&mut other);

            if !self.type_.is_empty() && !aggregate.type_.is_empty() {
                vec.push(format!("{} o-- {}", self.name, aggregate.name))
            }
        }

        for extension in &self.extensions {
            if !self.type_.is_empty() && !extension.type_.is_empty() {
                vec.push(format!("{} <|-- {}", self.name, extension.type_))
            }
        }

        for dependency in &self.dependencies {
            if !self.type_.is_empty() && !dependency.is_empty() {
                vec.push(format!("{} .. {}", self.name, dependency))
            }
        }

        vec.append(&mut end);

        vec
    }

    fn get_element(&self) -> (Vec<String>, Vec<String>) {
        let mut vec: Vec<String> = vec![];
        let mut end: Vec<String> = vec![];

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
                    Kind::EnumConstantDecl => vec.push(child.name.to_string()),
                    _ => {}
                }
            }
            vec.push(String::from("}"));
        } else if self.kind == Kind::Namespace {
            vec.push(format!("namespace {} {{", self.name));
            end.push(String::from("}"))
        }

        (vec, end)
    }
}
