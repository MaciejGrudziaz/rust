extern crate iron;
#[macro_use] extern crate mime;
extern crate router;

use iron::prelude::*;
use iron::status;
use router::Router;

fn main() {
    let mut router = Router::new();

    router.get("/", get_form, "root");
    router.post("/gcd", post_gcd, "gcd");

    println!("Serwer dostępny pod adresem http://localhost:3000...");
    Iron::new(get_form).http("localhost:3000").unwrap();
}

fn get_form(_request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    response.set_mut(r#"
        <title>Kalkulator GCD</title>
        <form action="/gcd" method="POST">
            <input type="text" name="n"/>
            <input type="text" name="n"/>
            <button type="submit">Oblicz GCD</button>
        </form>
    "#);

    Ok(response)
}
