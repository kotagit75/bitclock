use crate::model::{effect::Effect, state::State};

pub async fn run_effect(state: State, effect: Effect) {
    match effect {
        Effect::None => {}
        Effect::CreateStamp(pk, difficulty) => {
            // create_stamp and broadcast
        }
        Effect::Broadcast(message) => {}
    }
}
