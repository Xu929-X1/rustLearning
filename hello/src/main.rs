use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Deserialize;

#[derive(Deserialize)]
struct GcdcParameters {
    n: u64,
    m: u64,
}
// use std::env;
// use std::str::FromStr;

fn main() {
    // let mut numbers = Vec::new();
    // for arg in env::args().skip(1) {
    //     numbers.push(u64::from_str(&arg).expect("err parsing argument"));
    // }

    // if numbers.is_empty() {
    //     eprint!("Usage: gcd Number ...");
    //     std::process::exit(1);
    // }

    // let mut d = numbers[0];
    // for m in &numbers[1..] {
    //     d = gcd(d, *m);
    // }

    // println!("The greatest common divisor of {:?} is {}", numbers, d);
    let server = HttpServer::new(|| App::new().route("/", web::get().to(get_index)).route("/gcd", web::post().to(post_gcd)));
    println!("Listening on Port 3000");
    server
        .bind("::1:3000")
        .expect("Error binding the server to address")
        .run()
        .expect("Error when running the server");
}

fn get_index() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body(
        r#"
            <title>GCD Calculator</title>
            <form action = "gcd" method = "post">
            <input type = "text" placeholder="Please enter a number" name = "n"/>    
            <input type = "text" placeholder="Please enter a number" name = "m"/>
            <button type = "submit">Compute GCD</button>
            </form>
        "#,
    )
}
fn gcd(mut a: u64, mut b: u64) -> u64 {
    assert!(a != 0 && b != 0);
    while b != 0 {
        if a > b {
            std::mem::swap(&mut a, &mut b);
        }
        b %= a;
    }
    a
}
fn post_gcd(form: web::Form<GcdcParameters>) -> HttpResponse {
    if form.n == 0 || form.m == 0 {
        return HttpResponse::BadRequest().content_type("text/html").body(
            r#"
                <title>Error</title>
                <h1>It is useless to compute gcd with 0</h1>
                <a link="/">back<a>
            "#,
        );
    }

    let response = format!(
        "The gcd of number {} and number {} is <b>{}</b>\n",
        form.n,
        form.m,
        gcd(form.n, form.m)
    );
    HttpResponse::Ok().content_type("text/html").body(response)
}
