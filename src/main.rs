#[macro_use]
extern crate clap;
extern crate bnt;
extern crate ansi_term;

use clap::{ App, Arg};
use ansi_term::Colour::{ Green, Purple, Red, Blue };


fn main() {
    let matches = App::new("bnt")
                  .version(crate_version!())
                  .author("ByteBuddha <shadowcynical@gmail.com>")
                  .about("Retrieve concert info from the bands in town api.")
                  .arg(Arg::with_name("artist")
                               .required(true)
                               .takes_value(true)
                               .help("The artist name to fetch concerts for.")
                  )
                  .arg(Arg::with_name("format")
                          .short("f")
                          .long("format")
                          .takes_value(true)
                          .possible_values(&["term", "json"])
                  ).get_matches();
        let output = if matches.is_present("format") {
            matches.value_of("format").unwrap()
        } else {
            "term"
        };
        match matches.value_of("artist") {
            None => unimplemented!(),
            Some(art) => {
                if &output == &"term" {
                    let artist = bnt::get_artist(art);
                    let name = format!("Name: {}",Purple.bold().paint(&artist.name[..]));
                    let num = &artist.upcoming_event_count.unwrap().to_string();
                    let upcoming_events = format!("Upcoming Events: {}", Purple.bold().paint(num.to_string()));
                    println!("{}    {}",name, upcoming_events);
                    let bnt_data = bnt::get_artist_events(art);
                    for (dex, date) in bnt_data.iter().enumerate() {
                        let status = match &date.ticket_status.clone() {
                            &Some(ref d) => {
                                match &d[..] {
                                    "available" => Green.paint("available").to_string(),
                                    "unavailable" => Red.paint("unavailable").to_string(),
                                    _ => "".to_string()
                                }
                            },
                            &None => {"".to_string()}
                        };
                        let date_p = Purple.paint(&date.formatted_datetime.clone().unwrap()[..]).to_string();
                        let title = Red.paint(&date.title[..]).to_string();
                        let venue = Blue.paint(&date.venue.name[..]).to_string();
                        let d = format!("{}: ", Purple.paint(dex.to_string()).to_string());
                        println!("{}{}    {}    {}    {}",d,title, date_p, venue, status);
                    }
                }
            }
        }
}
