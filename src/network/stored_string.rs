use std::collections::HashMap;

use crate::{
    network::{
        account_info::{EditorAccountInfo, GameAccountInfo},
        password::ManagedItem,
    },
    save::Save,
};

pub trait StringStorable {
    fn store(&self, ident: &str, existing: &mut Vec<String>);
    fn read(ident: &str, data: &Vec<String>) -> Option<Self>
    where
        Self: Sized;
}

impl StringStorable for String {
    fn store(&self, ident: &str, existing: &mut Vec<String>) {
        existing.push(format!("{ident}:{self}"));
    }
    fn read(ident: &str, data: &Vec<String>) -> Option<Self> {
        for line in data {
            if line.starts_with(ident) {
                let val = line.split_at(ident.len() + 1).1;
                return Some(val.to_string());
            }
        }
        return None;
    }
}
impl StringStorable for str {
    fn store(&self, ident: &str, existing: &mut Vec<String>) {
        existing.push(format!("{ident}:{self}"));
    }
}

impl StringStorable for Vec<String> {
    fn store(&self, ident: &str, existing: &mut Vec<String>) {
        for val in self {
            val.store(ident, existing);
        }
    }
    fn read(ident: &str, data: &Vec<String>) -> Option<Self>
    where
        Self: Sized,
    {
        let mut res = Vec::new();
        for line in data {
            if line.starts_with(ident) {
                let val = line.split_at(ident.len() + 1).1;
                res.push(val.to_string());
            }
        }
        Some(res)
    }
}

impl StringStorable for HashMap<String, String> {
    fn store(&self, ident: &str, existing: &mut Vec<String>) {
        let to_write: Vec<String> = self.iter().map(|(k, v)| format!("{k}:{v}")).collect();

        to_write.store(ident, existing);
    }
    fn read(ident: &str, data: &Vec<String>) -> Option<Self>
    where
        Self: Sized,
    {
        let res = <Vec<String>>::read(ident, data)?;

        Some(HashMap::from_iter(res.into_iter().flat_map(|v| {
            v.split_once(":")
                .map(|(k, v2)| (k.to_string(), v2.to_string()))
        })))
    }
}

pub fn format_identifier(ident: &str) -> String {
    format!("_bcsfe:{ident}")
}

pub fn store_to_save<T: StringStorable + ?Sized>(ident: &str, val: &T, save: &mut Save) {
    val.store(&format_identifier(ident), &mut save.order_ids);
}
pub fn remove_and_store_to_save<T: StringStorable + ?Sized>(ident: &str, val: &T, save: &mut Save) {
    remove_from_save(ident, save);
    store_to_save(ident, val, save);
}

pub fn read_from_save<T: StringStorable>(ident: &str, save: &Save) -> Option<T> {
    T::read(&format_identifier(ident), &save.order_ids)
}

pub fn remove_from_save(ident: &str, save: &mut Save) {
    let ident = format_identifier(ident);

    let mut new_ids = Vec::new();
    for id in save.order_ids.drain(..) {
        if !id.starts_with(&ident) {
            new_ids.push(id);
        }
    }

    save.order_ids = new_ids;
}

const PASSWORD_IDENT: &str = "password";
const AUTH_TOKEN_IDENT: &str = "auth_token";
const SAVE_KEY_IDENT: &str = "save_key";
const MANAGED_ITEMS_IDENT: &str = "managed_items";

pub fn save_password(password: &str, save: &mut Save) {
    remove_and_store_to_save(PASSWORD_IDENT, password, save);
}
pub fn read_password(save: &Save) -> Option<String> {
    read_from_save(PASSWORD_IDENT, save)
}
pub fn remove_password(save: &mut Save) {
    remove_from_save(PASSWORD_IDENT, save);
}

pub fn save_auth_token(auth_token: &str, save: &mut Save) {
    remove_and_store_to_save(AUTH_TOKEN_IDENT, auth_token, save);
}
pub fn read_auth_token(save: &Save) -> Option<String> {
    read_from_save(AUTH_TOKEN_IDENT, save)
}
pub fn remove_auth_token(save: &mut Save) {
    remove_from_save(AUTH_TOKEN_IDENT, save);
}

pub fn save_save_key(save_key: &HashMap<String, String>, save: &mut Save) {
    remove_and_store_to_save(SAVE_KEY_IDENT, save_key, save);
}
pub fn read_save_key(save: &Save) -> Option<HashMap<String, String>> {
    read_from_save(SAVE_KEY_IDENT, save)
}
pub fn remove_save_key(save: &mut Save) {
    remove_from_save(SAVE_KEY_IDENT, save);
}

pub fn save_managed_items(items: &Vec<ManagedItem>, save: &mut Save) {
    remove_and_store_to_save(
        MANAGED_ITEMS_IDENT,
        &items.iter().map(|v| v.to_short_form()).collect::<Vec<_>>(),
        save,
    )
}

pub fn read_managed_items(save: &Save) -> Option<Vec<ManagedItem>> {
    let val: Vec<String> = read_from_save(MANAGED_ITEMS_IDENT, save)?;

    Some(
        val.into_iter()
            .map(|v| ManagedItem::from_short_form(&v))
            .collect(),
    )
}

pub fn remove_managed_items(save: &mut Save) {
    remove_from_save(MANAGED_ITEMS_IDENT, save);
}

pub fn add_managed_item(item: ManagedItem, save: &mut Save) {
    if item.unchanged() {
        return;
    }
    let mut current = read_managed_items(save).unwrap_or_default();

    current.push(item);

    save_managed_items(&current, save);
}

pub fn read_account_info(save: &Save) -> EditorAccountInfo {
    EditorAccountInfo {
        account_info: read_game_account_info(save),
        managed_items: read_managed_items(save).unwrap_or_default(),
    }
}

pub fn read_game_account_info(save: &Save) -> GameAccountInfo {
    GameAccountInfo {
        password: read_password(save),
        auth_token: read_auth_token(save),
    }
}

pub fn save_game_account_info(info: &GameAccountInfo, save: &mut Save) {
    if let Some(ref pw) = info.password {
        save_password(pw, save);
    }
    if let Some(ref tok) = info.auth_token {
        save_auth_token(tok, save);
    }
}

pub fn save_account_info(info: &EditorAccountInfo, save: &mut Save) {
    save_game_account_info(&info.account_info, save);
    save_managed_items(&info.managed_items, save);
}
