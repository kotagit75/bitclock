use crate::model::{effect::Effect, event::Event, state::State};

pub fn update(state: State, event: Event) -> (State, Effect) {
    match event {
        Event::None => (state, Effect::None),
    }
}
