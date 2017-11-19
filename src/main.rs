extern crate hyper;
extern crate clap;
extern crate tokio_core;
extern crate futures;

use clap::{Arg, App};
use std::io::{self, Write};
use hyper::Client;
use futures::Future;
use futures::stream::Stream;

fn fetch(url: &str, only_headers: bool) {
    let url = url.parse::<hyper::Uri>().unwrap();
    if url.scheme() != Some("http") {
        println!("non-http scheme passed");
        return;
    }
    let mut core = tokio_core::reactor::Core::new().unwrap();
    let handle = core.handle();
    let client = Client::configure().no_proto().build(&handle);

    let work = client
        .get(url)
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
        .get_matches();
    let url = matches.value_of("url").unwrap();
    fetch(url, matches.is_present("head"));
}
