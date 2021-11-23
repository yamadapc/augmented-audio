use actix::{Actor, Handler, Message, Supervised, SystemService};
use std::collections::HashMap;

pub struct EntityEvent {
    /// CHANGE
    pub event_type: String,
    /// TRACK:1234/...
    pub entity_id: String,
    pub old_value: String,
    pub new_value: String,
}

pub enum EntityFieldValue {
    List(Vec<EntityFieldValue>),
    Dict(HashMap<String, EntityFieldValue>),
    String(String),
    Int(i32),
    Float(f32),
    Null,
}

pub struct EntityState {
    pub id: String,
    pub type_name: String,
    pub fields: HashMap<String, EntityFieldValue>,
}

#[derive(Default)]
pub struct EntityService {
    entities: HashMap<String, EntityState>,
}

impl Actor for EntityService {
    type Context = actix::Context<Self>;
}

impl Supervised for EntityService {}
impl SystemService for EntityService {}

#[derive(Message)]
#[rtype(result = "()")]
pub struct CreateEntityMessage {
    pub id: String,
    pub type_name: String,
    pub fields: HashMap<String, EntityFieldValue>,
}

impl Handler<CreateEntityMessage> for EntityService {
    type Result = ();

    fn handle(&mut self, msg: CreateEntityMessage, _ctx: &mut Self::Context) -> Self::Result {
        self.entities.insert(
            msg.id.clone(),
            EntityState {
                id: msg.id,
                type_name: msg.type_name,
                fields: msg.fields,
            },
        );
    }
}
