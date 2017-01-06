use serde_json::ser::to_string;


#[derive(Deserialize, Serialize, Debug)]
pub struct Artist {
    pub name: String,
    pub image_url: Option<String>,
    pub thumb_url: Option<String>,
    pub facebook_tour_dates_url: Option<String>,
    pub mbid: Option<String>,
    pub upcoming_event_count: Option<u32>,
    pub tracker_count: Option<u32>
}

impl Artist {
    pub fn to_json(&self) -> String {
        to_string(self).unwrap()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    pub id: u64,
    pub title:String,
    pub datetime: String,
    pub formatted_datetime: Option<String>,
    pub formatted_location: Option<String>,
    pub ticket_url: Option<String>,
    pub ticket_type: Option<String>,
    pub ticket_status: Option<String>,
    pub on_sale_datetime: Option<String>,
    pub facebook_rsvp_url: Option<String>,
    pub description: Option<String>,
    pub artists: Vec<Artist>,
    pub venue: Venue
}

impl Event {
    pub fn to_json(&self) -> String {
        to_string(self).unwrap()
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Venue {
    pub name: String,
    pub place: String,
    pub city: String,
    pub region: String,
    pub country: String,
    pub latitude: Option<f32>,
    pub longitude: Option<f32>
}

impl Venue {
    pub fn to_json(&self) -> String {
        to_string(self).unwrap()
    }
}
