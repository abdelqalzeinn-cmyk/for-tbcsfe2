use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, HeaderValue};

use crate::{
    network::{
        account_info::{EditorAccountInfo, GameAccountInfo, SaveFileAccount},
        get_unix_timestamp,
        signature::{sign_v1, sign_v2},
        transfer::{ClientInfo, gen_nonce, new_client},
    },
    save::{GVCC, SaveFile},
    stream::StreamError,
};

pub fn v1_headers(
    inquiry_code: &str,
    data: &[u8],
) -> Result<reqwest::header::HeaderMap, std::io::Error> {
    let mut headers = reqwest::header::HeaderMap::new();

    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert("nyanko-signature-version", HeaderValue::from_static("1"));
    headers.insert(
        "nyanko-signature-algorithm",
        HeaderValue::from_static("HMACSHA256"),
    );
    headers.insert(
        "nyanko-timestamp",
        HeaderValue::from_str(&get_unix_timestamp().to_string())
            .expect("timestamp string was a valid ascii string"),
    );
    headers.insert(
        "nyanko-signature",
        HeaderValue::from_str(&sign_v2(inquiry_code, data)?)
            .expect("signature was a valid ascii string"),
    );

    Ok(headers)
}
pub fn v2_headers_empty(auth_token: &str) -> Result<reqwest::header::HeaderMap, std::io::Error> {
    let mut headers = reqwest::header::HeaderMap::new();

    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(
        "nyanko-timestamp",
        HeaderValue::from_str(&get_unix_timestamp().to_string())
            .expect("timestamp string was a valid ascii string"),
    );
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {auth_token}")).map_err(std::io::Error::other)?,
    );

    Ok(headers)
}

pub fn v2_headers(
    inquiry_code: &str,
    auth_token: &str,
    data: &[u8],
) -> Result<reqwest::header::HeaderMap, std::io::Error> {
    let mut headers = reqwest::header::HeaderMap::new();

    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert("nyanko-signature-version", HeaderValue::from_static("1"));
    headers.insert(
        "nyanko-signature-algorithm",
        HeaderValue::from_static("HMACSHA256"),
    );
    headers.insert(
        "nyanko-timestamp",
        HeaderValue::from_str(&get_unix_timestamp().to_string())
            .expect("timestamp string was a valid ascii string"),
    );
    headers.insert(
        "nyanko-signature",
        HeaderValue::from_str(&sign_v2(inquiry_code, data)?)
            .expect("signature was a valid ascii string"),
    );
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {auth_token}")).map_err(std::io::Error::other)?,
    );

    Ok(headers)
}

#[derive(Debug, thiserror::Error)]
pub enum PasswordError {
    #[error("failed to create a new client: {0}")]
    NewClient(reqwest::Error),
    #[error("failed to create headers: {0}")]
    Headers(std::io::Error),
    #[error("failed to send request: {0}")]
    SendReq(reqwest::Error),
    #[error("failed to get response json data: {0}")]
    JsonResp(reqwest::Error),
    #[error("failed to generate nonce: {0}")]
    GenNonce(std::io::Error),
    #[error("failed to generate json string: {0}")]
    ToJsonStr(serde_json::Error),
    #[error("password payload was null")]
    NullPayload,
    #[error("failed to get response text: {0}")]
    RespText(reqwest::Error),
    #[error("failed to get upload save data: {0}")]
    RespUpload(String),
    #[error("failed to generate signature v1: {0}")]
    SigV1(std::io::Error),
    #[error("failed to serialize save data: {0}")]
    SerializeSave(StreamError),
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct RequestJsonResponseV1<P> {
    pub status_code: i32,
    pub timestamp: i32,
    pub payload: Option<P>,
    pub nonce: String,
}
impl<P> RequestJsonResponseV1<P> {
    pub fn into_payload(self) -> Result<P, PasswordError> {
        self.payload.ok_or(PasswordError::NullPayload)
    }
}
#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct RequestJsonResponseV2<P> {
    pub status_code: Option<i32>,
    pub timestamp: Option<i32>,
    pub payload: Option<P>,
    pub nonce: Option<String>,
    pub message: Option<String>,
    pub code: Option<String>,
}

impl<P> RequestJsonResponseV2<P> {
    pub fn into_payload(self) -> Result<P, PasswordError> {
        self.payload.ok_or(PasswordError::NullPayload)
    }
}
type RequestJsonResponseV2Blank = RequestJsonResponseV2<String>;

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct PasswordJsonResponse {
    pub password: String,
    pub password_refresh_token: String,
    pub account_code: Option<String>,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct RequestPayloadV1<'a, S> {
    pub account_code: &'a str,
    #[serde(flatten)]
    pub payload: &'a S,
    pub nonce: String,
}
impl<'a, S> RequestPayloadV1<'a, S> {
    fn new(inquiry_code: &'a str, payload: &'a S) -> Result<Self, std::io::Error> {
        Ok(Self {
            account_code: inquiry_code,
            payload,
            nonce: gen_nonce()?,
        })
    }
}
#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct RequestPayloadV2<'a, S> {
    #[serde(flatten)]
    pub payload: &'a S,
    pub nonce: String,
}
impl<'a, S> RequestPayloadV2<'a, S> {
    fn new(payload: &'a S) -> Result<Self, std::io::Error> {
        Ok(Self {
            payload,
            nonce: gen_nonce()?,
        })
    }
}

async fn v1_request<S: serde::Serialize, D: for<'a> serde::Deserialize<'a>>(
    url: &str,
    inquiry_code: &str,
    payload: &S,
) -> Result<RequestJsonResponseV1<D>, PasswordError> {
    let payload = RequestPayloadV1::new(inquiry_code, payload).map_err(PasswordError::GenNonce)?;
    let data = serde_json::to_vec(&payload).map_err(PasswordError::ToJsonStr)?;
    let headers =
        v1_headers(&payload.account_code, &data).map_err(|e| PasswordError::Headers(e))?;

    let client = new_client().map_err(|e| PasswordError::NewClient(e))?;

    let resp = client
        .post(url)
        .headers(headers)
        .body(data)
        .send()
        .await
        .map_err(|e| PasswordError::SendReq(e))?;

    let json_data: RequestJsonResponseV1<D> =
        resp.json().await.map_err(|e| PasswordError::JsonResp(e))?;

    Ok(json_data)
}
async fn v2_request_empty<D: for<'a> serde::Deserialize<'a>>(
    url: &str,
    auth_token: &str,
) -> Result<RequestJsonResponseV2<D>, PasswordError> {
    let headers = v2_headers_empty(auth_token).map_err(|e| PasswordError::Headers(e))?;

    let client = new_client().map_err(|e| PasswordError::NewClient(e))?;

    let resp = client
        .get(url)
        .headers(headers)
        // .body(data)
        .send()
        .await
        .map_err(|e| PasswordError::SendReq(e))?;

    let json_data: RequestJsonResponseV2<D> =
        resp.json().await.map_err(|e| PasswordError::JsonResp(e))?;

    Ok(json_data)
}
async fn v2_request<S: serde::Serialize, D: for<'a> serde::Deserialize<'a>>(
    url: &str,
    auth_token: &str,
    inquiry_code: &str,
    payload: &S,
) -> Result<RequestJsonResponseV2<D>, PasswordError> {
    let payload = RequestPayloadV2::new(payload).map_err(PasswordError::GenNonce)?;
    let data = serde_json::to_vec(&payload).map_err(PasswordError::ToJsonStr)?;
    let headers =
        v2_headers(inquiry_code, auth_token, &data).map_err(|e| PasswordError::Headers(e))?;

    let client = new_client().map_err(|e| PasswordError::NewClient(e))?;

    let resp = client
        .post(url)
        .headers(headers)
        .body(data)
        .send()
        .await
        .map_err(|e| PasswordError::SendReq(e))?;

    let json_data: RequestJsonResponseV2<D> =
        resp.json().await.map_err(|e| PasswordError::JsonResp(e))?;

    Ok(json_data)
}

const AUTH_URL: &str = "https://nyanko-auth.ponosgames.com";
const BACKUPS_URL: &str = "https://nyanko-backups.ponosgames.com";
const SAVE_URL: &str = "https://nyanko-save.ponosgames.com";
const MANAGED_ITEM_URL: &str = "https://nyanko-managed-item.ponosgames.com";

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewAccountPayload {
    pub account_created_at: u64,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct NewAccountJsonResponse {
    pub success: bool,
    pub account_id: String,
}

pub async fn create_new_account() -> Result<NewAccountJsonResponse, PasswordError> {
    let url = format!("{BACKUPS_URL}/?action=createAccount&referenceId=");

    let client = new_client().map_err(PasswordError::NewClient)?;

    let resp = client
        .get(url)
        .send()
        .await
        .map_err(PasswordError::SendReq)?;

    let json_data: NewAccountJsonResponse = resp.json().await.map_err(PasswordError::JsonResp)?;
    Ok(json_data)
}

pub async fn register_new_account(
    inquiry_code: &str,
    account_created_at: u64,
) -> Result<RequestJsonResponseV1<PasswordJsonResponse>, PasswordError> {
    let payload = NewAccountPayload { account_created_at };
    v1_request(&format!("{AUTH_URL}/v1/users"), inquiry_code, &payload).await
}

#[derive(Debug)]
pub struct NewAccountData {
    pub password: String,
    pub password_refresh_token: String,
    pub inquiry_code: String,
}

#[derive(Debug, Clone)]
pub struct UploadInfo {
    pub inquiry_code: Option<String>,
    pub password_refresh_token: Option<String>,
    pub gvcc: GVCC,
    pub catfood: i32,
    pub rare_tickets: i32,
    pub platinum_tickets: i32,
    pub legend_tickets: i32,
    pub playtime: i32,
    pub user_rank: i32,
}

impl UploadInfo {
    pub fn from_save(save: &SaveFile) -> Self {
        Self {
            gvcc: save.gvcc,
            catfood: save.save.catfood,
            rare_tickets: save.save.rare_tickets,
            platinum_tickets: save.save.get_platinum_tickets().unwrap_or_default(),
            legend_tickets: save.save.get_legend_tickets().unwrap_or_default(),
            playtime: save.save.get_play_time().unwrap_or_default(),
            user_rank: save.save.calculate_user_rank(),
            inquiry_code: save.save.get_inquiry_code().map(String::from),
            password_refresh_token: save.save.get_password_refresh_token().map(String::from),
        }
    }
}

#[derive(Debug, Clone)]
pub struct NewAccountInfo {
    pub account_info: GameAccountInfo,
    pub password_refresh_token: String,
    pub inquiry_code: String,
}

#[derive(Debug, Clone)]
pub struct AccountState {
    pub inquiry_code: Option<String>,
    pub account_info: GameAccountInfo,
    pub password_refresh_token: Option<String>,
    pub gvcc: GVCC,
    pub items: ManagedItemsUpdate,
    pub account_created_at: u64,
}

impl AccountState {
    pub async fn try_get_inquiry_code(&mut self) -> Result<String, PasswordError> {
        if let Some(ref iq) = self.inquiry_code {
            return Ok(iq.to_string());
        }
        let iq = create_new_account().await?.account_id;

        self.inquiry_code = Some(iq.clone());

        Ok(iq)
    }
    pub async fn init_new_account(&mut self) -> Result<(), PasswordError> {
        let token = Box::pin(self.try_get_auth_token()).await?;

        let iq = self.try_get_inquiry_code().await?;

        update_managed_items(&token, &iq, &self.items).await?;

        Ok(())
    }

    pub async fn try_get_password_refresh_token(&mut self) -> Result<String, PasswordError> {
        if let Some(ref prt) = self.password_refresh_token {
            return Ok(prt.to_string());
        }

        let iq = self.try_get_inquiry_code().await?;

        let payload = register_new_account(&iq, self.account_created_at)
            .await?
            .into_payload()?;

        self.password_refresh_token = Some(payload.password_refresh_token.clone());
        self.account_info.password = Some(payload.password);

        Ok(payload.password_refresh_token)
    }

    pub async fn try_get_password(&mut self) -> Result<String, PasswordError> {
        if let Some(ref pw) = self.account_info.password {
            return Ok(pw.to_string());
        }

        let prt = self.try_get_password_refresh_token().await?;
        let iq = self.try_get_inquiry_code().await?;

        let mut payload = refresh_password(&iq, &prt).await?.into_payload();

        if payload.is_err() {
            payload = register_new_account(&iq, self.account_created_at)
                .await?
                .into_payload();
        }

        let payload = payload?;

        self.password_refresh_token = Some(payload.password_refresh_token);
        self.account_info.password = Some(payload.password.clone());
        if let Some(iq) = payload.account_code {
            self.inquiry_code = Some(iq);
            self.init_new_account().await?;
        }

        return Ok(payload.password);
    }

    pub async fn try_get_auth_token(&mut self) -> Result<String, PasswordError> {
        if let Some(ref token) = self.account_info.auth_token {
            return Ok(token.to_string());
        }
        let password = self.try_get_password().await?;
        let iq = self.try_get_inquiry_code().await?;
        let token = get_auth_token(&iq, &password, self.gvcc.cc, self.gvcc.gv)
            .await?
            .into_payload()?
            .token;

        self.account_info.auth_token = Some(token.clone());

        Ok(token)
    }
}

pub async fn upload_save(
    mut save_file: SaveFileAccount,
    info: UploadInfo,
    account_info: EditorAccountInfo,
) -> Result<(TransferCodes, SaveFileAccount), PasswordError> {
    let mut state = AccountState {
        inquiry_code: info.inquiry_code,
        account_info: account_info.account_info,
        password_refresh_token: info.password_refresh_token,
        gvcc: info.gvcc,
        items: ManagedItemsUpdate {
            catfood_amount: info.catfood,
            is_paid: false, // TODO
            legend_ticket_amount: info.legend_tickets,
            platinum_ticket_amount: info.platinum_tickets,
            rare_ticket_amount: info.rare_tickets,
        },
        account_created_at: 0, // TODO
    };

    let auth_token = state.try_get_auth_token().await?;
    let inquiry_code = state.try_get_inquiry_code().await?;

    let save_key = get_save_key(&auth_token).await?.into_payload()?;

    let prt = state.try_get_password_refresh_token().await?;
    let password = state.try_get_password().await?;

    save_file
        .save_file
        .save
        .set_inquiry_code(inquiry_code.clone());
    save_file.save_file.save.set_password_refresh_token(prt);
    save_file.account_info.account_info.password = Some(password);
    save_file.account_info.account_info.auth_token = Some(auth_token.clone());

    let save_data = save_file
        .save_file
        .write_with_hash()
        .map_err(PasswordError::SerializeSave)?;

    let codes = upload_save_data(
        &auth_token,
        save_key,
        &inquiry_code,
        save_data,
        account_info.managed_items,
        info.playtime,
        info.user_rank,
        Vec::new(),
    )
    .await?
    .into_payload()?;

    Ok((codes, save_file))
}

pub async fn create_and_upload(
    save_data: SaveFileAccount,
    info: UploadInfo,
) -> Result<(TransferCodes, SaveFileAccount), PasswordError> {
    upload_save(save_data, info, EditorAccountInfo::default()).await
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct RefreshPasswordPayload<'a> {
    password_refresh_token: &'a str,
}

pub async fn refresh_password(
    inquiry_code: &str,
    password_refresh_token: &str,
) -> Result<RequestJsonResponseV1<PasswordJsonResponse>, PasswordError> {
    let url = format!("{AUTH_URL}/v1/user/password");
    let data = RefreshPasswordPayload {
        password_refresh_token,
    };
    v1_request(&url, inquiry_code, &data).await
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuthTokenJsonResponse {
    pub token: String,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct AuthTokenRequest<'a> {
    client_info: ClientInfo,
    password: &'a str,
}

pub async fn get_auth_token(
    inquiry_code: &str,
    password: &str,
    cc: crate::country_code::CountryCode,
    gv: crate::game_version::GameVersion,
) -> Result<RequestJsonResponseV1<AuthTokenJsonResponse>, PasswordError> {
    let url = format!("{AUTH_URL}/v1/tokens");
    v1_request(
        &url,
        inquiry_code,
        &AuthTokenRequest {
            password,
            client_info: ClientInfo::new(cc, gv),
        },
    )
    .await
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SaveKeyJsonResponse {
    pub url: String,
    pub key: String,
    pub policy: String,
    #[serde(rename = "x-amz-algorithm")]
    pub x_amz_algorithm: String,
    #[serde(rename = "x-amz-credential")]
    pub x_amz_credential: String,
    #[serde(rename = "x-amz-date")]
    pub x_amz_date: String,
    #[serde(rename = "x-amz-security-token")]
    pub x_amz_security_token: String,
    #[serde(rename = "x-amz-signature")]
    pub x_amz_signature: String,
}

pub async fn get_save_key(
    auth_token: &str,
) -> Result<RequestJsonResponseV2<SaveKeyJsonResponse>, PasswordError> {
    let url = format!(
        "{SAVE_URL}/v2/save/key?nonce={}",
        gen_nonce().map_err(PasswordError::GenNonce)?
    );
    v2_request_empty(&url, auth_token).await
}

fn generate_save_form_data(
    save_key: SaveKeyJsonResponse,
    save_data: Vec<u8>,
) -> reqwest::multipart::Form {
    let formdata = reqwest::multipart::Form::new();

    formdata
        .text("key", save_key.key)
        .text("policy", save_key.policy)
        .text("x-amz-signature", save_key.x_amz_signature)
        .text("x-amz-credential", save_key.x_amz_credential)
        .text("x-amz-algorithm", save_key.x_amz_algorithm)
        .text("x-amz-date", save_key.x_amz_date)
        .text("x-amz-security-token", save_key.x_amz_security_token)
        .part(
            "file",
            reqwest::multipart::Part::bytes(save_data).file_name("file.sav"),
        )
}

async fn post_save_data(
    save_key: SaveKeyJsonResponse,
    save_data: Vec<u8>,
) -> Result<(), PasswordError> {
    let client = new_client().map_err(PasswordError::NewClient)?;

    let resp = client
        .post(&save_key.url)
        .multipart(generate_save_form_data(save_key, save_data))
        .send()
        .await
        .map_err(PasswordError::SendReq)?;

    match resp.status().is_success() {
        true => Ok(()),
        false => {
            let text = resp.text().await.map_err(|e| PasswordError::RespText(e))?;

            return Err(PasswordError::RespUpload(text));
        }
    }
}

#[derive(Debug, Copy, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ManagedItemDetailType {
    Get,
    Use,
}
#[derive(Debug, Copy, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ManagedItemType {
    Catfood,
    RareTicket,
    PlatinumTicket,
    LegendTicket,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ManagedItem {
    amount: i32,
    detail_code: String,
    detail_created_at: u64,
    detail_type: ManagedItemDetailType,
    managed_item_type: ManagedItemType,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct UploadRequestPayload<'a> {
    managed_item_details: Vec<ManagedItem>,
    play_time: i32,
    rank: i32,
    reciept_log_ids: Vec<String>,
    #[serde(rename = "signature_v1")]
    signature_v1: String,
    save_key: &'a str,
}

impl<'a> UploadRequestPayload<'a> {
    fn new(
        managed_items: Vec<ManagedItem>,
        play_time: i32,
        user_rank: i32,
        reciept_log_ids: Vec<String>,
        save_key: &'a str,
        inquiry_code: &str,
    ) -> Result<Self, PasswordError> {
        let managed_item_data =
            serde_json::to_vec(&managed_items).map_err(PasswordError::ToJsonStr)?;
        let sig_v1 =
            sign_v1(inquiry_code, &managed_item_data).map_err(|e| PasswordError::SigV1(e))?;
        Ok(Self {
            managed_item_details: managed_items,
            play_time,
            rank: user_rank,
            reciept_log_ids,
            signature_v1: sig_v1,
            save_key,
        })
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct TransferCodes {
    pub transfer_code: String,
    #[serde(rename = "pin")]
    pub confirmation_code: String,
}

pub async fn upload_save_data(
    auth_token: &str,
    save_key: SaveKeyJsonResponse,
    inquiry_code: &str,
    save_data: Vec<u8>,
    managed_items: Vec<ManagedItem>,
    play_time: i32,
    user_rank: i32,
    reciept_log_ids: Vec<String>,
) -> Result<RequestJsonResponseV2<TransferCodes>, PasswordError> {
    let key = save_key.key.clone();
    post_save_data(save_key, save_data).await?;

    let url = format!("{SAVE_URL}/v2/transfers");

    let payload = UploadRequestPayload::new(
        managed_items,
        play_time,
        user_rank,
        reciept_log_ids,
        &key,
        inquiry_code,
    )?;

    v2_request(&url, auth_token, inquiry_code, &payload).await
}
pub async fn upload_save_metadata(
    auth_token: &str,
    save_key: SaveKeyJsonResponse,
    inquiry_code: &str,
    save_data: Vec<u8>,
    managed_items: Vec<ManagedItem>,
    play_time: i32,
    user_rank: i32,
    reciept_log_ids: Vec<String>,
) -> Result<RequestJsonResponseV2Blank, PasswordError> {
    let key = save_key.key.clone();
    post_save_data(save_key, save_data).await?;

    let url = format!("{SAVE_URL}/v2/backups");

    let payload = UploadRequestPayload::new(
        managed_items,
        play_time,
        user_rank,
        reciept_log_ids,
        &key,
        inquiry_code,
    )?;

    v2_request(&url, auth_token, inquiry_code, &payload).await
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ManagedItemsUpdate {
    catfood_amount: i32,
    is_paid: bool,
    legend_ticket_amount: i32,
    platinum_ticket_amount: i32,
    rare_ticket_amount: i32,
}

pub async fn update_managed_items(
    auth_token: &str,
    inquiry_code: &str,
    managed_items: &ManagedItemsUpdate,
) -> Result<RequestJsonResponseV2Blank, PasswordError> {
    let url = format!("{MANAGED_ITEM_URL}/v1/managed-items");
    v2_request(&url, auth_token, inquiry_code, managed_items).await
}
