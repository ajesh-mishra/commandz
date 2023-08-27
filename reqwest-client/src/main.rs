use std::error::Error;

use arguments::get_args;

mod arguments;
mod routes;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (route, verb) = get_args();
    routes::request_router(route, verb).await;

    Ok(())
}
