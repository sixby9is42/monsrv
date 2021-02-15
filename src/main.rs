use futures::future;
use serde_derive::{Deserialize, Serialize};
use std::net::Ipv4Addr;
use std::net::Ipv6Addr;
use warp::Filter;

#[derive(Serialize, Deserialize)]
struct World {
    hello: String,
}

#[tokio::main]
async fn main() {
    // GET /hello
    let hello = warp::get()
        .and(warp::path!("hello"))
        .map(|| format!("Hello, World!"));

    // GET /hello/<string>
    let hello_with_param = warp::get()
        .and(warp::path!("hello" / String))
        .map(|name| format!("Hello, {}!", name));

    // POST /hello
    // body: {"hello": <string>}
    let hello_with_post = warp::post()
        .and(warp::path!("hello"))
        .and(warp::body::json())
        .map(|world: World| format!("Hello, {}!", world.hello));

    // GET /hello/json
    let hello_with_json = warp::get().and(warp::path!("hello" / "json")).map(|| {
        let world = World {
            hello: "World".to_string(),
        };
        warp::reply::json(&world)
    });

    let routes = hello_with_json
        .or(hello_with_param)
        .or(hello_with_post)
        .or(hello);

    future::join(
        warp::serve(routes).run((Ipv4Addr::UNSPECIFIED, 80)),
        warp::serve(routes).run((Ipv6Addr::UNSPECIFIED, 80)),
    )
    .await;
}
