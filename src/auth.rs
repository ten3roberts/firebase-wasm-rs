mod user;
use std::{error::Error, fmt};

use crate::FirebaseError;
pub use user::*;
use wasm_bindgen::{prelude::*, JsCast};

#[derive(Clone, Debug, derive_more::Deref)]
pub struct AuthError {
    pub kind: AuthErrorKind,
    #[deref]
    pub source: FirebaseError,
}

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.source.fmt(f)
    }
}

impl Error for AuthError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}

impl From<FirebaseError> for AuthError {
    fn from(err: FirebaseError) -> Self {
        Self {
            kind: err.code().parse().unwrap(),
            source: err,
        }
    }
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, strum_macros::EnumString)]
#[non_exhaustive]
pub enum AuthErrorKind {
    #[strum(serialize = "auth/app-deleted")]
    AppDeleted,
    #[strum(serialize = "auth/app-not-authorized")]
    AppNotAuthorized,
    #[strum(serialize = "auth/argument-error")]
    ArgumentError,
    #[strum(serialize = "auth/invalid-api-key")]
    InvalidApiKey,
    #[strum(serialize = "auth/invalid-user-token")]
    InvalidUserToken,
    #[strum(serialize = "auth/invalid-tenant-id")]
    InvalidTenantId,
    #[strum(serialize = "auth/network-request-failed")]
    NetworkRequestFailed,
    #[strum(serialize = "auth/operation-not-allowed")]
    OperationNotAllowed,
    #[strum(serialize = "auth/requires-recent-login")]
    RequiresRecentLogin,
    #[strum(serialize = "auth/too-many-requests")]
    TooManyRequests,
    #[strum(serialize = "auth/unauthorized-domain")]
    UnauthorizedDomain,
    #[strum(serialize = "auth/user-disabled")]
    UserDisabled,
    #[strum(serialize = "auth/user-token-expired")]
    UserTokenExpired,
    #[strum(serialize = "auth/web-storage-unsupported")]
    WebStorageUnsupported,
    #[strum(serialize = "auth/invalid-email")]
    InvalidEmail,
    #[strum(serialize = "auth/user-not-found")]
    UserNotFound,
    #[strum(serialize = "auth/wrong-password")]
    WrongPassword,
    #[strum(serialize = "auth/email-already-in-use")]
    EmailAlreadyInUse,
    #[strum(serialize = "auth/weak-password")]
    WeakPassword,
    #[strum(serialize = "auth/missing-android-pkg-name")]
    MissingAndroidPackageName,
    #[strum(serialize = "auth/missing-continue-uri")]
    MissingContinueUri,
    #[strum(serialize = "auth/missing-ios-bundle-id")]
    MissingIOSBundleId,
    #[strum(serialize = "auth/invalid-continue-uri")]
    InvalidContinueUri,
    #[strum(serialize = "auth/unauthorized-continue-uri")]
    UnauthorizedContinueUri,
    #[strum(serialize = "auth/expired-action-code")]
    ExpiredActionCode,
    #[strum(default)]
    Other(String),
}

#[derive(Debug, Clone, PartialEq, Eq, TypedBuilder, serde::Serialize)]
#[builder(field_defaults(default))]
pub struct ActionCodeSettings {
    pub android: Option<AndroidActionCodeSettings>,
    #[builder(setter(strip_option))]
    pub handle_code_in_app: Option<bool>,
    pub ios: Option<IOSActionCodeSettings>,
    #[builder(!default)]
    pub url: String,
    pub dynamic_link_domain: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, TypedBuilder, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AndroidActionCodeSettings {
    #[builder(default, setter(strip_option))]
    pub install_app: Option<bool>,
    #[builder(default)]
    pub minimum_version: Option<String>,
    pub package_name: String,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, TypedBuilder, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IOSActionCodeSettings {
    pub bundle_id: String,
}

pub async fn create_user_with_email_and_password(
    auth: Auth,
    email: &str,
    password: &str,
) -> Result<UserCredential, AuthError> {
    create_user_with_email_and_password_js(auth, email, password)
        .await
        .map(|cred| cred.unchecked_into::<UserCredential>())
        .map_err(|err| err.unchecked_into::<FirebaseError>().into())
}

pub async fn sign_in_with_email_and_password(
    auth: Auth,
    email: &str,
    password: &str,
) -> Result<UserCredential, AuthError> {
    sign_in_with_email_and_password_js(auth, email, password)
        .await
        .map(|cred| cred.unchecked_into::<UserCredential>())
        .map_err(|err| err.unchecked_into::<FirebaseError>().into())
}

pub async fn send_sign_in_link_to_email(
    auth: Auth,
    email: &str,
    action_code_settings: &ActionCodeSettings,
) -> Result<(), AuthError> {
    let action_code_settings = serde_wasm_bindgen::to_value(action_code_settings).unwrap();

    send_sign_in_link_to_email_js(auth, email, action_code_settings)
        .await
        .map_err(|err| err.unchecked_into::<FirebaseError>().into())
}

pub async fn sign_in_with_email_link(
    auth: Auth,
    email: &str,
    email_link: &str,
) -> Result<UserCredential, AuthError> {
    sign_in_with_email_link_js(auth, email, email_link)
        .await
        .map(|u| u.unchecked_into::<UserCredential>())
        .map_err(|err| err.unchecked_into::<FirebaseError>().into())
}

#[wasm_bindgen(module = "firebase/auth")]
extern "C" {
    #[derive(Clone, Debug)]
    pub type Auth;
    #[derive(Clone, Debug)]
    pub type UserCredential;

    #[wasm_bindgen(js_name = getAuth)]
    pub fn get_auth() -> Auth;

    #[wasm_bindgen(js_name = onAuthStateChanged)]
    pub fn on_auth_state_changed(auth: Auth, callback: &Closure<dyn FnMut(Option<User>)>);

    #[wasm_bindgen(js_name = createUserWithEmailAndPassword, catch)]
    async fn create_user_with_email_and_password_js(
        auth: Auth,
        email: &str,
        password: &str,
    ) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_name = signInWithEmailAndPassword, catch)]
    async fn sign_in_with_email_and_password_js(
        auth: Auth,
        email: &str,
        password: &str,
    ) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_name = signInWithEmailLink, catch)]
    async fn sign_in_with_email_link_js(
        auth: Auth,
        email: &str,
        email_link: &str,
    ) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_name = isSignInWithEmailLink, )]
    pub fn is_sign_in_with_email_link(auth: Auth, email_link: &str) -> bool;

    #[wasm_bindgen(js_name = sendSignInLinkToEmail, catch)]
    async fn send_sign_in_link_to_email_js(
        auth: Auth,
        email: &str,
        action_code_settings: JsValue,
    ) -> Result<(), JsValue>;

    #[wasm_bindgen(js_name = signOut)]
    pub async fn sign_out(auth: Auth);

    // =======================================================
    //                  UserCredential
    // =======================================================

    #[wasm_bindgen(method, getter)]
    pub fn user(this: &UserCredential) -> user::User;

    #[wasm_bindgen(method, getter, js_name = providerId)]
    pub fn provider_id(this: &UserCredential) -> String;

    #[wasm_bindgen(method, getter, js_name = operationType)]
    pub fn operation_type(this: &UserCredential) -> String;
}
