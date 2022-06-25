use jni::{
    objects::{JClass, JString},
    sys::jlong,
    JNIEnv,
};
use longbridge::Config;

use crate::{error::jni_result, types::FromJValue};

#[no_mangle]
pub extern "system" fn Java_com_longbridge_SdkNative_newConfig(
    env: JNIEnv,
    _class: JClass,
    app_key: JString,
    app_secret: JString,
    access_token: JString,
    http_url: JString,
    quote_ws_url: JString,
    trade_ws_url: JString,
) -> jlong {
    jni_result(&env, 0, || {
        let app_key = String::from_jvalue(&env, app_key.into())?;
        let app_secret = String::from_jvalue(&env, app_secret.into())?;
        let access_token = String::from_jvalue(&env, access_token.into())?;
        let http_url = <Option<String>>::from_jvalue(&env, http_url.into())?;
        let quote_ws_url = <Option<String>>::from_jvalue(&env, quote_ws_url.into())?;
        let trade_ws_url = <Option<String>>::from_jvalue(&env, trade_ws_url.into())?;

        let mut config = Config::new(app_key, app_secret, access_token);

        if let Some(http_url) = http_url {
            config = config.http_url(http_url);
        }
        if let Some(quote_ws_url) = quote_ws_url {
            config = config.quote_ws_url(quote_ws_url);
        }
        if let Some(trade_ws_url) = trade_ws_url {
            config = config.trade_ws_url(trade_ws_url);
        }

        Ok(Box::into_raw(Box::new(config)) as jlong)
    })
}

#[no_mangle]
pub extern "system" fn Java_com_longbridge_SdkNative_newConfigFromEnv(
    env: JNIEnv,
    _class: JClass,
) -> jlong {
    jni_result(&env, 0, || {
        let config = Config::from_env()?;
        Ok(Box::into_raw(Box::new(config)) as jlong)
    })
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_freeConfig(
    _env: JNIEnv,
    _class: JClass,
    config: jlong,
) {
    let _ = Box::from_raw(config as *mut Config);
}
