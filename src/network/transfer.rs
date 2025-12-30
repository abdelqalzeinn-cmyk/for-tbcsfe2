use reqwest::header::{CONTENT_TYPE, HeaderValue};

use crate::{
    country_code::CountryCode, game_version::GameVersion, network::account_info::GameAccountInfo,
};

const SAVE_URL: &str = "https://nyanko-save.ponosgames.com";

#[derive(Debug, serde::Serialize)]
pub struct Client {
    #[serde(rename = "countryCode")]
    pub country_code: ClientCountryCode,
    pub version: GameVersion,
}

#[derive(Debug, Copy, Clone, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ClientCountryCode {
    Ja,
    En,
    Kr,
    Tw,
}

impl From<CountryCode> for ClientCountryCode {
    fn from(value: CountryCode) -> Self {
        match value {
            CountryCode::Jp => Self::Ja,
            CountryCode::En => Self::En,
            CountryCode::Kr => Self::Kr,
            CountryCode::Tw => Self::Tw,
        }
    }
}

#[derive(Debug, serde::Serialize)]
pub struct Device {
    pub model: String,
}

#[derive(Debug, serde::Serialize)]
pub struct Os {
    #[serde(rename = "type")]
    pub os_type: String,
    pub version: String,
}

#[derive(Debug, serde::Serialize)]
pub struct ClientInfo {
    pub client: Client,
    pub device: Device,
    pub os: Os,
}

#[derive(Debug, serde::Serialize)]
pub struct ReceptionPayload {
    #[serde(rename = "clientInfo")]
    pub client_info: ClientInfo,
    pub nonce: String,
    pub pin: String,
}

impl ReceptionPayload {
    pub fn new(client_info: ClientInfo, confirmation_code: String) -> Result<Self, std::io::Error> {
        Ok(Self {
            client_info,
            nonce: gen_nonce()?,
            pin: confirmation_code,
        })
    }
}

impl Default for ClientInfo {
    fn default() -> Self {
        Self::new(CountryCode::default(), GameVersion::default())
    }
}

pub fn gen_nonce() -> Result<String, std::io::Error> {
    let mut nonce_bytes: [u8; 16] = [0; 16];
    getrandom::fill(&mut nonce_bytes).map_err(|e| std::io::Error::other(e.to_string()))?;
    let nonce_val: u128 = u128::from_ne_bytes(nonce_bytes);
    Ok(format!("{nonce_val:x}"))
}

impl ClientInfo {
    pub fn new(cc: CountryCode, gv: GameVersion) -> Self {
        Self {
            client: Client {
                country_code: cc.into(),
                version: gv,
            },
            device: Device {
                model: "SM-G955F".to_string(),
            },
            os: Os {
                os_type: "android".to_string(),
                version: "9".to_string(),
            },
        }
    }

    pub fn with_cc(mut self, cc: CountryCode) -> Self {
        self.client.country_code = cc.into();

        self
    }

    pub fn with_gv(mut self, gv: GameVersion) -> Self {
        self.client.version = gv;

        self
    }

    pub fn with_device(mut self, device: Device) -> Self {
        self.device = device;

        self
    }

    pub fn with_device_maybe(self, device: Option<Device>) -> Self {
        if let Some(device) = device {
            self.with_device(device)
        } else {
            self
        }
    }
    pub fn with_os(mut self, os: Os) -> Self {
        self.os = os;

        self
    }

    pub fn with_os_maybe(self, os: Option<Os>) -> Self {
        if let Some(os) = os {
            self.with_os(os)
        } else {
            self
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum FromCodesError {
    #[error("failed to serialize payload: {0}")]
    SerdeJson(serde_json::error::Error),
    #[error("failed to initialize new client: {0}")]
    NewClient(reqwest::Error),
    #[error("failed to send reqwest: {0}")]
    SendReq(reqwest::Error),
    #[error("invalid transfer/confirmation codes")]
    InvalidCodes,
    #[error("failed to fetch result body: {0}")]
    Body(reqwest::Error),
    #[error("failed to generate nonce: {0}")]
    Random(std::io::Error),
}

pub fn new_client() -> Result<reqwest::Client, reqwest::Error> {
    reqwest::Client::builder()
        .user_agent("Dalvik/2.1.0 (Linux; U; Android 9; SM-G955F Build/N2G48B)")
        .build()
}

#[derive(Debug, Clone)]
pub struct FromCodesResponse {
    pub save_data: Vec<u8>,
    pub account_info: GameAccountInfo,
    pub password_refresh_token: Option<String>,
}

pub async fn from_codes(
    transfer_code: String,
    confirmation_code: String,
    cc: CountryCode,
    gv: GameVersion,
) -> Result<FromCodesResponse, FromCodesError> {
    from_codes_opt_device(transfer_code, confirmation_code, cc, gv, None, None).await
}

pub async fn from_codes_with_device(
    transfer_code: String,
    confirmation_code: String,
    cc: CountryCode,
    gv: GameVersion,
    device: Device,
    os: Os,
) -> Result<FromCodesResponse, FromCodesError> {
    from_codes_opt_device(
        transfer_code,
        confirmation_code,
        cc,
        gv,
        Some(device),
        Some(os),
    )
    .await
}
async fn from_codes_opt_device(
    transfer_code: String,
    confirmation_code: String,
    cc: CountryCode,
    gv: GameVersion,
    device: Option<Device>,
    os: Option<Os>,
) -> Result<FromCodesResponse, FromCodesError> {
    let url = format!("{SAVE_URL}/v2/transfers/{transfer_code}/reception");

    let client_info = ClientInfo::new(cc, gv)
        .with_device_maybe(device)
        .with_os_maybe(os);

    let payload =
        ReceptionPayload::new(client_info, confirmation_code).map_err(FromCodesError::Random)?;

    let payload_str = serde_json::to_string(&payload).map_err(FromCodesError::SerdeJson)?;

    let client = new_client().map_err(FromCodesError::NewClient)?;

    let resp = client
        .post(url)
        .header(CONTENT_TYPE, "application/json")
        .body(payload_str)
        .send()
        .await
        .map_err(FromCodesError::SendReq)?;

    let resp_content_type = resp.headers().get(CONTENT_TYPE);
    if resp_content_type != Some(&HeaderValue::from_static("application/octet-stream")) {
        return Err(FromCodesError::InvalidCodes);
    }

    let password_refresh_token =
        header_value_to_string(resp.headers().get("Nyanko-Password-Refresh-Token"));
    let password = header_value_to_string(resp.headers().get("Nyanko-Password"));

    let body = resp.bytes().await.map_err(FromCodesError::Body)?;
    Ok(FromCodesResponse {
        save_data: body.into(),
        account_info: GameAccountInfo {
            password,
            auth_token: None,
        },
        password_refresh_token,
    })
}

fn header_value_to_string(header: Option<&HeaderValue>) -> Option<String> {
    header.and_then(|v| v.to_str().ok().map(|s| s.to_string()))
}
