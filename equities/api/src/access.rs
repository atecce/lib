//! Cloudflare Access, checked at the origin.
//!
//! Access lets a request through to the tunnel only after its login (a
//! person: email one-time code) or a service token, and attaches its verdict
//! as `Cf-Access-Jwt-Assertion`: a JWT signed (RS256) by the team's keys,
//! published at `https://<team>.cloudflareaccess.com/cdn-cgi/access/certs`.
//! Checking it here means a request that reaches the service some other way
//! (anything on this Mac can open 127.0.0.1:8787) is refused too.
//!
//! Valid means: signed by one of the team's current keys, issued by the team,
//! for this application's audience tag (its AUD), and not expired.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use jsonwebtoken::jwk::JwkSet;
use serde::Deserialize;
use tokio::sync::RwLock;

/// How long the team's keys are kept before fetching them again (Cloudflare
/// rotates them every six weeks, publishing the next before using it).
const KEYS_MAX_AGE: Duration = Duration::from_secs(3600);

pub struct Access {
    team: String,
    aud: String,
    http: reqwest::Client,
    keys: RwLock<Option<(Instant, HashMap<String, DecodingKey>)>>,
}

/// Who Access let through.
#[derive(Debug, Clone)]
pub enum Identity {
    Person(String),
    ServiceToken(String),
}

impl std::fmt::Display for Identity {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Identity::Person(email) => write!(f, "{}", email),
            Identity::ServiceToken(id) => write!(f, "service token {}", id),
        }
    }
}

#[derive(Deserialize)]
struct Claims {
    email: Option<String>,
    /// A service token's client ID.
    common_name: Option<String>,
}

impl Access {
    /// `team`: the team's name (`steep-snow-0aca`); `aud`: the Access
    /// application's audience tag.
    pub fn new(team: String, aud: String) -> Arc<Access> {
        Arc::new(Access { team, aud, http: reqwest::Client::new(), keys: RwLock::new(None) })
    }

    fn issuer(&self) -> String {
        format!("https://{}.cloudflareaccess.com", self.team)
    }

    async fn fetch_keys(&self) -> Result<HashMap<String, DecodingKey>, String> {
        let url = format!("{}/cdn-cgi/access/certs", self.issuer());
        let set: JwkSet = self.http.get(&url).timeout(Duration::from_secs(10)).send().await
            .and_then(|r| r.error_for_status()).map_err(|e| format!("{}: {}", url, e))?
            .json().await.map_err(|e| format!("{}: {}", url, e))?;
        let mut keys = HashMap::new();
        for jwk in &set.keys {
            if let (Some(kid), Ok(key)) = (&jwk.common.key_id, DecodingKey::from_jwk(jwk)) {
                keys.insert(kid.clone(), key);
            }
        }
        if keys.is_empty() {
            return Err(format!("{}: no usable keys", url));
        }
        Ok(keys)
    }

    /// The key for `kid`, fetching the team's keys when they are old or don't
    /// include it (a rotation).
    async fn key(&self, kid: &str) -> Result<DecodingKey, String> {
        if let Some((at, keys)) = self.keys.read().await.as_ref() {
            if at.elapsed() < KEYS_MAX_AGE {
                if let Some(k) = keys.get(kid) {
                    return Ok(k.clone());
                }
            }
        }
        let keys = self.fetch_keys().await?;
        let key = keys.get(kid).cloned();
        *self.keys.write().await = Some((Instant::now(), keys));
        key.ok_or_else(|| format!("no key {}", kid))
    }

    /// Check an assertion; `Err` says why it was refused.
    pub async fn verify(&self, assertion: &str) -> Result<Identity, String> {
        let header = decode_header(assertion).map_err(|e| format!("not a JWT: {}", e))?;
        if header.alg != Algorithm::RS256 {
            return Err(format!("algorithm {:?}", header.alg));
        }
        let kid = header.kid.ok_or("no key ID")?;
        let key = self.key(&kid).await?;
        let mut v = Validation::new(Algorithm::RS256);
        v.set_audience(&[&self.aud]);
        v.set_issuer(&[self.issuer()]);
        v.set_required_spec_claims(&["exp", "iss", "aud"]);
        let claims = decode::<Claims>(assertion, &key, &v).map_err(|e| e.to_string())?.claims;
        match (claims.email.filter(|e| !e.is_empty()), claims.common_name) {
            (Some(email), _) => Ok(Identity::Person(email)),
            (None, Some(id)) => Ok(Identity::ServiceToken(id)),
            (None, None) => Err("neither an email nor a service token".into()),
        }
    }
}
