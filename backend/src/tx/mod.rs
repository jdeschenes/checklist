mod error;
mod extension;
pub mod layer;
pub mod state;
pub mod tx;

pub fn setup(pool: sqlx::Pool<sqlx::Postgres>)-> (state::State, layer::Layer) 
{
    let state = state::State::new(pool);
    let layer = layer::Layer::new(state.clone());
    (state, layer)
}
