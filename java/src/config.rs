use jni::{
    objects::{JClass, JString},
    sys::jlong,
    JNIEnv,
};
use longbridge::Config;

use crate::error::jni_result;

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
        let mut config = Config::new(
            env.get_string(app_key)?,
            env.get_string(app_secret)?,
            env.get_string(access_token)?,
        );

        if !http_url.is_null() {
            config = config.http_url(env.get_string(http_url)?);
        }

        if !quote_ws_url.is_null() {
            config = config.quote_ws_url(env.get_string(quote_ws_url)?);
        }

        if !trade_ws_url.is_null() {
            config = config.trade_ws_url(env.get_string(trade_ws_url)?);
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
