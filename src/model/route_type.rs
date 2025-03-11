use serde::Deserialize;
use url::Url;

use crate::Client;

#[derive(Clone, Debug, Hash, PartialEq, Eq, Deserialize)]
pub struct RouteType {
    #[serde(rename = "route_type")]
    pub id: usize,
    #[serde(rename = "route_type_name")]
    pub name: String,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, Deserialize)]
pub struct RouteTypesResponse {
    pub route_types: Vec<RouteType>,
    pub status: super::Status,
}

impl Client {
    pub async fn get_route_types(&mut self) -> Result<RouteTypesResponse, reqwest::Error> {
        self.get(Url::parse("https://timetableapi.ptv.vic.gov.au/v3/route_types").unwrap())
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
                .get_route_types()
                .await
                .unwrap()
        );
        panic!();
    }
}
