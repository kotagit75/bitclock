use crate::model::effect::Effect;

pub async fn run_effect(effect: Effect) {
    match effect {
        Effect::None => {}
        Effect::CreateStamp(pk, difficulty) => {
            // create_stamp and broadcast
        }
    }
}
