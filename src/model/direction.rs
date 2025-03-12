use serde::Deserialize;
use url::Url;

use crate::Client;

#[derive(Clone, Debug, Hash, PartialEq, Eq, Deserialize)]
pub struct Direction {
    pub route_direction_description: String,
    #[serde(rename = "direction_id")]
    pub id: usize,
    #[serde(rename = "direction_name")]
    pub name: String,
    pub route_id: usize,
    pub route_type: usize,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, Deserialize)]
pub struct DirectionsResponse {
    pub directions: Vec<Direction>,
    pub status: super::Status,
}

pub fn directions_for_route_url(route_id: usize) -> Url {
    let mut url = Url::parse("https://timetableapi.ptv.vic.gov.au/v3/directions/route").unwrap();
    url.path_segments_mut()
        .expect("HTTPS URL can always be a base")
        .push(&route_id.to_string());
    url
}

pub fn directions_url(direction_id: usize) -> Url {
    let mut url = Url::parse("https://timetableapi.ptv.vic.gov.au/v3/directions").unwrap();
    url.path_segments_mut()
        .expect("HTTPS URL can always be a base")
        .push(&direction_id.to_string());
    url
}

pub fn directions_for_route_type_url(direction_id: usize, route_type_id: usize) -> Url {
    let mut url = Url::parse("https://timetableapi.ptv.vic.gov.au/v3/directions").unwrap();
    url.path_segments_mut()
        .expect("HTTPS URL can always be a base")
        .push(&direction_id.to_string())
        .push("route_type")
        .push(&route_type_id.to_string());
    url
}

impl Client {
    pub async fn get_directions_for_route(
        &mut self,
        route_id: usize,
    ) -> Result<DirectionsResponse, reqwest::Error> {
        self.get(directions_for_route_url(route_id)).await
    }

    pub async fn get_directions(
        &mut self,
        direction_id: usize,
    ) -> Result<DirectionsResponse, reqwest::Error> {
        self.get(directions_url(direction_id)).await
    }

    pub async fn get_directions_for_route_type(
        &mut self,
        direction_id: usize,
        route_type_id: usize,
    ) -> Result<DirectionsResponse, reqwest::Error> {
        self.get(directions_for_route_type_url(direction_id, route_type_id))
            .await
    }
}
