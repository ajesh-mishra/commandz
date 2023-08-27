use command::get_response;

use crate::arguments::Verb;

mod command;

pub async fn request_router(route: String, verb: Verb) {
    println!("\n{} route is triggered with verb: {verb:?} \n", &route);

    match route.as_str() {
        "command" => get_response(verb).await,
        _ => panic!("Route: {route} not supported")
    }
}
