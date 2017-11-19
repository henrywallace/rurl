extern crate hyper;
extern crate clap;
extern crate tokio_core;
extern crate futures;

use clap::{Arg, App};
use std::io::{self, Write};
use hyper::Client;
use futures::Future;
use futures::stream::Stream;
use std::str::FromStr;

fn fetch(url: &str, only_headers: bool, method: hyper::Method) {
    let url = url.parse::<hyper::Uri>().unwrap();
    if url.scheme() != Some("http") {
        println!("non-http scheme passed");
        return;
    }
    let mut core = tokio_core::reactor::Core::new().unwrap();
    let handle = core.handle();
    let client = Client::configure().no_proto().build(&handle);
    let req = hyper::Request::new(method, url);

    let work = client
        .request(req)
        .and_then(|res| {
            if only_headers {
                println!("HTTP/1.1 {}", res.status());
                println!("{}", res.headers());
            }
            res.body().for_each(|chunk| if only_headers {
                io::sink().write_all(&chunk).map_err(From::from)
            } else {
                io::stdout().write_all(&chunk).map_err(From::from)
            })
        })
        .map(|_| {
            println!("");
        });

    core.run(work).unwrap();
}

fn main() {
    let matches = App::new("rurl")
        .arg(Arg::with_name("url").help("URL to fetch").required(true))
        .arg(Arg::with_name("head").short("I").long("head"))
        .arg(
            Arg::with_name("request")
                .short("X")
                .long("request")
                .takes_value(true),
        )
        .get_matches();
    let url = matches.value_of("url").unwrap();
    let req = matches.value_of("request").unwrap_or("GET");
    fetch(
        url,
        matches.is_present("head"),
        hyper::Method::from_str(req).unwrap(),
    );
}
