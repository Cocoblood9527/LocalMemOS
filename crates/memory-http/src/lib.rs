pub mod routes;
pub mod state;

pub fn app_with_state(state: state::AppState) -> axum::Router {
    axum::Router::new()
        .route("/facts/:id/history", axum::routing::get(routes::history))
        .route("/*path", axum::routing::post(routes::dispatch_post))
        .with_state(state)
}

pub async fn app_for_test(path: &str) -> axum::Router {
    let state = state::AppState::new(path).expect("failed to create app state");
    app_with_state(state)
}
