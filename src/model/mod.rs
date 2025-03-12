mod departure;
pub use departure::*;
mod direction;
pub use direction::*;
mod disruption;
pub use disruption::*;
mod fare_estimate;
pub use fare_estimate::*;
mod outlet;
pub use outlet::*;
mod pattern;
pub use pattern::*;
mod route;
pub use route::*;
mod route_type;
pub use route_type::*;
mod run;
pub use run::*;
mod search;
pub use search::*;
mod status;
pub use status::*;
mod stop;
pub use stop::*;

#[cfg(test)]
mod tests {
    use crate::Credential;

    pub fn env_cred() -> Credential {
        Credential::new(
            std::env::var("PTV_USER_ID").unwrap().to_owned(),
            std::env::var("PTV_API_KEY").unwrap().as_bytes(),
        )
    }
}
