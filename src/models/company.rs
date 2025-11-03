#![allow(unused)] // TODO: remove me

use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct Company {
    id: Uuid,
    name: String,
    fields: Vec<Uuid>, // TODO
}

impl Company {
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            fields: Vec::new(),
        }
    }

    pub fn builder() -> CompanyBuilder {
        CompanyBuilder::default()
    }
}

#[derive(Default)]
pub struct CompanyBuilder {
    name: Option<String>,
    fields: Vec<Uuid>,
}

impl CompanyBuilder {
    pub fn name(&mut self, name: &str) -> &mut Self {
        self.name = Some(name.to_owned());
        self
    }

    pub fn field(&mut self, field_id: Uuid) -> &mut Self {
        self.fields.push(field_id);
        self
    }

    pub fn fields(&mut self, field_id: Vec<Uuid>) -> &mut Self {
        self.fields.extend(field_id);
        self
    }

    pub fn build(self) -> Company {
        Company {
            id: Uuid::new_v4(),
            name: self.name.expect("name should be specified"),
            fields: self.fields,
        }
    }
}
