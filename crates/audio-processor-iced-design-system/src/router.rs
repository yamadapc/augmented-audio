use std::collections::HashMap;

use iced::{Element, Text};

use crate::updatable::Updatable;

pub type RouteIdRef<'a> = &'a str;
pub type RouteId = String;

#[derive(Debug, Clone)]
pub enum Message<InnerMessage> {
    RouteChanged(RouteId),
    Inner(InnerMessage),
}

pub fn route_changed_message<InnerMessage>(route_id: RouteIdRef) -> Message<InnerMessage> {
    Message::RouteChanged(String::from(route_id))
}

pub struct RouterState<RouteState: Updatable> {
    current_route: RouteId,
    routes: HashMap<RouteId, RouteState>,
}

impl<RouteState: Updatable> RouterState<RouteState> {
    pub fn new(initial_route: RouteId, initial_routes: HashMap<RouteId, RouteState>) -> Self {
        RouterState {
            current_route: initial_route,
            routes: initial_routes,
        }
    }

    pub fn update(&mut self, message: Message<RouteState::Message>) {
        match message {
            Message::RouteChanged(route_id) => {
                self.set_current_route(route_id);
            }
            Message::Inner(_) => {}
        }
    }

    fn set_current_route(&mut self, route_id: String) {
        self.current_route = route_id;
    }

    pub fn add_route(&mut self, route_id: RouteId, state: RouteState) {
        self.routes.insert(route_id, state);
    }

    pub fn route<Message, F>(&mut self, route_id: RouteIdRef, renderer: F) -> Element<Message>
    where
        F: Fn(&mut RouteState) -> Element<Message>,
    {
        if self.current_route != route_id {
            return empty();
        }

        if let Some(route) = self.routes.get_mut(route_id) {
            return renderer(route);
        }

        empty()
    }
}

fn empty<'a, Message>() -> Element<'a, Message> {
    Text::new("").into()
}
