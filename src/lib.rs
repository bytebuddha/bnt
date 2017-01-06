#![feature(proc_macro,custom_derive)]

extern crate serde;
#[macro_use]
extern crate serde_derive;

extern crate serde_json;

extern crate hyper;

mod ser;
pub use ser::{ Event, Artist, Venue };

use hyper::Client;
use hyper::header::Connection;
use std::io::Read;

pub fn get_artist_events(artist: &str) -> Vec<Event> {
    let mut request = String::from("http://api.bandsintown.com/artists/");
    request.push_str(artist);
    request.push_str("/events.json?api_version=2.0&app_id=RustScraper");
    let client = Client::new();
    let mut response = client.get(&request).
        header(Connection::close()).send().unwrap();
    let mut body = String::new();
    response.read_to_string(&mut body).unwrap();
    serde_json::from_str(&body).unwrap()
}


pub fn get_artist(artist: &str) -> Artist {
    let mut request = String::from("http://api.bandsintown.com/artists/");
    request.push_str(artist);
    request.push_str(".json?api_version=2.0&app_id=RustScraper");
    let client = Client::new();
    let mut response = client.get(&request).
        header(Connection::close()).send().unwrap();
    let mut body = String::new();
    response.read_to_string(&mut body).unwrap();
    serde_json::from_str(&body).unwrap()
    //body
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        println!("{:#?}",get_artist_events("Revocation"));
    }
}
