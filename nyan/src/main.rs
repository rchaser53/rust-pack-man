extern crate iron;
extern crate router;
extern crate staticfile;
extern crate mount;
extern crate time;

use std::path::Path;
use iron::prelude::*;
use iron::status;
use staticfile::Static;
use mount::Mount;

use iron::{BeforeMiddleware, AfterMiddleware, typemap};
use time::precise_time_ns;

use router::Router;

struct ResponseTime;

impl typemap::Key for ResponseTime { type Value = u64; }

impl BeforeMiddleware for ResponseTime {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        req.extensions.insert::<ResponseTime>(precise_time_ns());
        Ok(())
    }
}

impl AfterMiddleware for ResponseTime {
    fn after(&self, req: &mut Request, res: Response) -> IronResult<Response> {
        let delta = precise_time_ns() - *req.extensions.get::<ResponseTime>().unwrap();
        println!("Request took: {} ms", (delta as f64) / 1000000.0);
        Ok(res)
    }
}

fn hello_world(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((iron::status::Ok, "Hello World")))
}

fn main() {
    let mut mount = Mount::new();
    mount.mount("/abi", Static::new(Path::new("index.html")));

    let mut router = Router::new();
    router.get("/abi/:query", handler, "query");

    fn handler(req: &mut Request) -> IronResult<Response> {
        let ref query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("/");
        // Ok(Response::with((status::Ok, *query)))
        Ok(Response::with((status::Ok, "nyan")))
    }

    Iron::new(mount).http("localhost:3000").unwrap();
}