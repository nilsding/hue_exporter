use anyhow::{bail, Result};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Sensor {
    pub r#type: String,
    pub name: String,
    pub state: SensorState,
    pub config: SensorConfig,
}

#[derive(Debug, Deserialize)]
pub struct SensorState {
    pub lastupdated: String,
    pub daylight: Option<bool>,
    pub flag: Option<bool>,
    pub presence: Option<bool>,
    pub dark: Option<bool>,
    pub status: Option<i64>,
    pub buttonevent: Option<i64>,
    pub temperature: Option<i64>,
    pub lightlevel: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct SensorConfig {
    pub on: bool,
    pub configured: Option<bool>,
    pub sunriseoffset: Option<i64>,
    pub sunsetoffset: Option<i64>,
    pub reachable: Option<bool>,
    pub battery: Option<i64>,
    pub alert: Option<String>,
    pub ledindication: Option<bool>,
    pub usertest: Option<bool>,
    pub sensitivity: Option<i64>,
    pub sensitivitymax: Option<i64>,
    pub tholddark: Option<i64>,
    pub tholdoffset: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct Light {
    pub r#type: String,
    pub name: String,
    pub state: LightState,
    // The config of a light only contains string values ... omitted it (for now)
}

#[derive(Debug, Deserialize)]
pub struct LightState {
    pub reachable: bool,
    pub on: bool,
    pub bri: Option<i64>,
    pub hue: Option<i64>,
    pub sat: Option<i64>,
    pub ct: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct AuthResponse {
    pub error: Option<AuthResponseError>,
    pub success: Option<AuthResponseSuccess>,
}

#[derive(Debug, Deserialize)]
pub struct AuthResponseError {
    pub r#type: i64,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct AuthResponseSuccess {
    pub username: String,
}

#[derive(Clone, Debug)]
pub struct HueClient {
    token: String,
    hue_url: String,
}

impl HueClient {
    pub fn new(token: String, hue_url: String) -> Self {
        HueClient { token, hue_url }
    }

    pub async fn sensors(&self) -> Result<HashMap<String, Sensor>> {
        let result = reqwest::get(format!("{}/api/{}/sensors", self.hue_url, self.token))
            .await?
            .json::<HashMap<String, Sensor>>()
            .await?;

        Ok(result)
    }

    pub async fn lights(&self) -> Result<HashMap<String, Light>> {
        let result = reqwest::get(format!("{}/api/{}/lights", self.hue_url, self.token))
            .await?
            .json::<HashMap<String, Light>>()
            .await?;

        Ok(result)
    }

    pub fn authorize(hue_url: &str) -> Result<String> {
        let mut params = HashMap::new();
        params.insert("devicetype", "hue_exporter");

        let client = reqwest::blocking::Client::new();
        let result = client
            .post(format!("{}/api", hue_url))
            .json(&params)
            .send()?
            .json::<Vec<AuthResponse>>()?;

        if result.len() != 1 {
            bail!("expected response with one element, got {}", result.len());
        }

        if let Some(err) = &result[0].error {
            if err.r#type == 101 {
                bail!("You now need to push the link button on your Hue bridge.")
            } else {
                bail!("received response {} ({})", err.r#type, err.description);
            }
        }

        match &result[0].success {
            Some(succ) => Ok(succ.username.clone()),
            None => bail!("somehow received no response????"),
        }
    }
}
