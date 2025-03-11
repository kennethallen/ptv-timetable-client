mod route;
pub use route::*;
mod route_type;
pub use route_type::*;
mod status;
pub use status::*;

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
