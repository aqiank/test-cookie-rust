extern crate iron;
extern crate cookie;
extern crate mount;
extern crate router;
extern crate staticfile;
extern crate time;

// Std
use std::path::Path;

// Iron
use iron::prelude::*;
use iron::headers;
use iron::status;

// Cookie
use cookie::Cookie;

// Mount
use mount::Mount;

// Router
use router::Router;

// Staticfile
use staticfile::Static;

fn cookie_handler(req: &mut Request) -> IronResult<Response> {
    if let Some(cookie) = req.headers.get::<headers::Cookie>() {
        println!("{:?}", cookie);
    }

    Ok(Response::with((status::Ok)))
}

fn set_cookie_handler(_: &mut Request) -> IronResult<Response> {
    let mut response = Response::with((status::Ok));

    let now = time::now() + time::Duration::minutes(1);
    
    let mut a = Cookie::new("SessionID".to_string(), "loremipsumdolorsitamet".to_string());
    a.expires = Some(now);

    let b = Cookie::new("UserID".to_string(), "1337".to_string());

    let c = Cookie::new("Expires".to_string(), format!("{}", time::strftime("%a, %d-%b-%Y %T %Z", &now).unwrap()));

    let setcookie = headers::SetCookie(vec![a, b, c]);
    response.headers.set(setcookie);

    Ok(response)
}

fn main() {
    let mut router = Router::new();
    router.get("/cookie", cookie_handler);
    router.get("/setcookie", set_cookie_handler);

    let mut mount = Mount::new();
    mount.mount("/", Static::new(Path::new(".")));
    mount.mount("/api", router);

    Iron::new(mount).http("localhost:8080").unwrap();
}
