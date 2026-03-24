use crate::model::{effect::Effect, state::State};

mod run;

pub async fn loop_effect_run(state: State, effect: Effect) {
    let mut effect_opt = Some(effect);
    while let Some(effect) = effect_opt {
        debug!("Running effect: {:?}", effect);
        effect_opt = run::run_effect(state.clone(), effect).await;
    }
}
