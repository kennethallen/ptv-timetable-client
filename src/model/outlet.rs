use serde::Deserialize;
use url::Url;

use crate::Client;

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Outlet {
    #[serde(rename = "outlet_slid_spid")]
    slid_spid: String,
    #[serde(rename = "outlet_name")]
    name: String,
    #[serde(rename = "outlet_business")]
    business: String,
    #[serde(rename = "outlet_latitude")]
    latitude: f64,
    #[serde(rename = "outlet_longitude")]
    longitude: f64,
    #[serde(rename = "outlet_suburb")]
    suburb: String,
    #[serde(rename = "outlet_postcode")]
    postcode: u16,
    #[serde(rename = "outlet_business_hour_mon")]
    business_hour_mon: Option<String>,
    #[serde(rename = "outlet_business_hour_tue")]
    business_hour_tue: Option<String>,
    #[serde(rename = "outlet_business_hour_wed")]
    business_hour_wed: Option<String>,
    #[serde(rename = "outlet_business_hour_thu")]
    business_hour_thu: Option<String>,
    #[serde(rename = "outlet_business_hour_fri")]
    business_hour_fri: Option<String>,
    #[serde(rename = "outlet_business_hour_sat")]
    business_hour_sat: Option<String>,
    #[serde(rename = "outlet_business_hour_sun")]
    business_hour_sun: Option<String>,
    #[serde(rename = "outlet_notes")]
    notes: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct NearbyOutlet {
    #[serde(rename = "outlet_distance")]
    distance: f64,
    #[serde(rename = "outlet_slid_spid")]
    slid_spid: String,
    #[serde(rename = "outlet_name")]
    name: String,
    #[serde(rename = "outlet_business")]
    business: String,
    #[serde(rename = "outlet_latitude")]
    latitude: f64,
    #[serde(rename = "outlet_longitude")]
    longitude: f64,
    #[serde(rename = "outlet_suburb")]
    suburb: String,
    #[serde(rename = "outlet_postcode")]
    postcode: u16,
    #[serde(rename = "outlet_business_hour_mon")]
    business_hour_mon: String,
    #[serde(rename = "outlet_business_hour_tue")]
    business_hour_tue: String,
    #[serde(rename = "outlet_business_hour_wed")]
    business_hour_wed: String,
    #[serde(rename = "outlet_business_hour_thu")]
    business_hour_thu: String,
    #[serde(rename = "outlet_business_hour_fri")]
    business_hour_fri: String,
    #[serde(rename = "outlet_business_hour_sat")]
    business_hour_sat: String,
    #[serde(rename = "outlet_business_hour_sun")]
    business_hour_sun: String,
    #[serde(rename = "outlet_notes")]
    notes: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct OutletsResponse {
    pub outlets: Vec<Outlet>,
    pub status: super::Status,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct NearbyOutletsResponse {
    pub outlets: Vec<NearbyOutlet>,
    pub status: super::Status,
}

pub fn outlets_url(max_results: Option<usize>) -> Url {
    let mut url = Url::parse("https://timetableapi.ptv.vic.gov.au/v3/outlets").unwrap();
    if let Some(max_results) = max_results {
        url.query_pairs_mut()
            .append_pair("max_results", &max_results.to_string());
    }
    url
}

pub fn outlets_near_location_url(
    latitude: f64,
    longitude: f64,
    max_distance: Option<f64>,
    max_results: Option<usize>,
) -> Url {
    let mut url = Url::parse("https://timetableapi.ptv.vic.gov.au/v3/outlets/location").unwrap();
    url.path_segments_mut()
        .expect("HTTPS URL can always be a base")
        .push(&format!("{},{}", latitude, longitude));
    {
        let mut query = url.query_pairs_mut();
        if let Some(max_distance) = max_distance {
            query.append_pair("max_distance", &max_distance.to_string());
        }
        if let Some(max_results) = max_results {
            query.append_pair("max_results", &max_results.to_string());
        }
    }
    url
}

impl Client {
    pub async fn get_outlets(
        &mut self,
        max_results: Option<usize>,
    ) -> Result<OutletsResponse, reqwest::Error> {
        self.get(outlets_url(max_results)).await
    }

    pub async fn get_outlets_near_location(
        &mut self,
        latitude: f64,
        longitude: f64,
        max_distance: Option<f64>,
        max_results: Option<usize>,
    ) -> Result<NearbyOutletsResponse, reqwest::Error> {
        self.get(outlets_near_location_url(
            latitude,
            longitude,
            max_distance,
            max_results,
        ))
        .await
    }
}
