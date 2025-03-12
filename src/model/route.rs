use serde::Deserialize;
use time::{Date, OffsetDateTime, format_description::well_known::Iso8601};
use url::Url;

use crate::Client;

#[derive(Clone, Debug, Hash, PartialEq, Eq, Deserialize)]
pub struct Route {
    #[serde(rename = "route_service_status")]
    pub service_status: ServiceStatus,
    #[serde(rename = "route_type")]
    pub type_id: usize,
    #[serde(rename = "route_name")]
    pub name: String,
    #[serde(rename = "route_number")]
    pub number: String,
    #[serde(rename = "route_gtfs_id")]
    pub gtfs_id: String,
    pub geopath: Vec<Geopath>,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, Deserialize)]
pub struct ServiceStatus {
    description: String,
    #[serde(deserialize_with = "time::serde::iso8601::deserialize")]
    timestamp: OffsetDateTime,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, Deserialize)]
pub struct Geopath {
    direction_id: usize,
    valid_from: String, // TODO deserialize date
    valid_to: String,   // TODO deserialize date
    paths: Vec<String>, // TODO deserialize data
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, Deserialize)]
pub struct RouteResponse {
    pub route: Route,
    pub status: super::Status,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, Deserialize)]
pub struct RoutesResponse {
    pub routes: Vec<Route>,
    pub status: super::Status,
}

pub fn routes_url(
    route_types: impl IntoIterator<Item = usize>,
    route_name: Option<impl AsRef<str>>,
) -> Url {
    let mut url = Url::parse("https://timetableapi.ptv.vic.gov.au/v3/routes").unwrap();
    {
        let mut query = url.query_pairs_mut();
        for route_type in route_types {
            query.append_pair("route_types", &route_type.to_string());
        }
        if let Some(route_name) = route_name {
            query.append_pair("route_name", route_name.as_ref());
        }
    }
    url
}

pub fn route_url(route_id: usize, include_geopath: bool, geopath_utc: Option<Date>) -> Url {
    let mut url = Url::parse("https://timetableapi.ptv.vic.gov.au/v3/routes/{}").unwrap();
    url.path_segments_mut()
        .expect("HTTPS URL can always be a base")
        .push(&route_id.to_string());
    {
        let mut query = url.query_pairs_mut();
        query.append_pair("include_geopath", &include_geopath.to_string());
        if let Some(geopath_utc) = geopath_utc {
            query.append_pair(
                "geopath_utc",
                &geopath_utc.format(&Iso8601::DEFAULT).unwrap(),
            );
        }
    }
    url
}

impl Client {
    pub async fn get_routes(
        &mut self,
        route_types: impl IntoIterator<Item = usize>,
        route_name: Option<impl AsRef<str>>,
    ) -> Result<RoutesResponse, reqwest::Error> {
        self.get(routes_url(route_types, route_name)).await
    }

    pub async fn get_route(
        &mut self,
        route_id: usize,
        include_geopath: bool,
        geopath_utc: Option<Date>,
    ) -> Result<RouteResponse, reqwest::Error> {
        self.get(route_url(route_id, include_geopath, geopath_utc))
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn get() {
        dbg!(
            Client::new(reqwest::Client::new(), crate::model::tests::env_cred())
                .get_routes([0, 2], Some("Pak"))
                .await
                .unwrap()
        );
        panic!();
    }
}
