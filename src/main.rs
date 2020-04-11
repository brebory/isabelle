use std::env;
use std::path::PathBuf;
use warp::Filter;

#[tokio::main]
async fn main() {
    let content_dir = env::var("CONTENT_DIR")
        .expect("Expected the $CONTENT_DIR to exist");

    let mut path = PathBuf::from(content_dir);
    path.push("index.html");

    println!("{:#?}", path.to_str());

    let root = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file(format!("{}", path.to_str().unwrap())));

    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a number");

    let end = root.with(warp::log("index"));
    
    println!("warp server running on 0.0.0.0:{}", &port);
    warp::serve(end).run(([127, 0, 0, 1], port)).await;
}
