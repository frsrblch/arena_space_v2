use crate::*;
use gen_id_relations::{Relation, Relations};

type Comp<T> = Component<Body, T>;

#[derive(Debug, Clone)]
pub struct Body {
    pub name: String,
    pub mass: Mass,
    pub radius: Length,
}

fixed_id!(Body);

#[derive(Debug, Copy, Clone)]
pub struct BodyLinks {
    pub parent: Option<Id<Body>>,
}

#[derive(Debug, Default)]
pub struct Bodies {
    pub name: Comp<String>,
    pub mass: Comp<Mass>,
    pub radius: Comp<Length>,
    pub relation: Relations<Body>,
}

impl Bodies {
    pub(crate) fn insert(&mut self, id: Id<Body>, body: Body, links: BodyLinks) {
        self.name.insert(id, body.name);
        self.mass.insert(id, body.mass);
        self.radius.insert(id, body.radius);

        match links.parent {
            Some(parent) => self.relation.insert_child(id, parent),
            None => self.relation.insert_parent(id),
        }
    }
}
