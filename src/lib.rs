pub mod model;

use hmac::{Hmac, Mac as _};
use serde::de::DeserializeOwned;
use sha1::Sha1;
use url::Url;

#[derive(Clone, Debug)]
pub struct Credential {
    user_id: String,
    hmac: Hmac<Sha1>,
}

impl Credential {
    pub fn new(user_id: String, api_key: impl AsRef<[u8]>) -> Self {
        Self {
            user_id,
            hmac: Hmac::<Sha1>::new_from_slice(api_key.as_ref())
                .expect("HMAC can take key of any size"),
        }
    }

    pub fn sign(&mut self, url: &mut Url) {
        url.query_pairs_mut().append_pair("devid", &self.user_id);
        self.hmac.update(url.path().as_bytes());
        self.hmac.update(b"?");
        self.hmac
            .update(url.query().expect("just set a query pair").as_bytes());
        let signature = self.hmac.finalize_reset();
        url.query_pairs_mut()
            .append_pair("signature", &hex::encode_upper(signature.into_bytes()));
    }
}

#[derive(Clone, Debug)]
pub struct Client {
    client: reqwest::Client,
    cred: Credential,
}

impl Client {
    pub fn new(client: reqwest::Client, cred: Credential) -> Self {
        Self { client, cred }
    }

    async fn get<T: DeserializeOwned>(&mut self, mut url: Url) -> Result<T, reqwest::Error> {
        self.cred.sign(&mut url);
        self.client.get(url).send().await?.json().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signing() {
        let mut cred = Credential::new(
            "2".to_owned(),
            "9c132d31-6a30-4cac-8d8b-8a1970834799".to_owned(),
        );
        let mut url = Url::parse("https://timetableapi.ptv.vic.gov.au/v3/route_types").unwrap();
        cred.sign(&mut url);
        assert_eq!(
            url,
            Url::parse("https://timetableapi.ptv.vic.gov.au/v3/route_types?devid=2&signature=9104F2DE9E883943F8627BF959914C2CDCD9FD10").unwrap(),
        );
    }
}
