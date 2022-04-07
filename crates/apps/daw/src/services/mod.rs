// = copyright ====================================================================
// DAW: Flutter UI for a DAW application
// Copyright (C) 2022  Pedro Tacla Yamada
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.
// = /copyright ===================================================================
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
