use jni::{
    errors::Result,
    objects::{JObject, JThrowable, JValue},
    JNIEnv,
};

use crate::init::{LONG_CLASS, OPENAPI_EXCEPTION_CLASS};

#[derive(Debug, thiserror::Error)]
pub(crate) enum JniError {
    #[error(transparent)]
    Jni(#[from] jni::errors::Error),
    #[error(transparent)]
    OpenApi(#[from] longbridge::Error),
}

impl JniError {
    fn into_jni_error_object<'a>(env: &'a JNIEnv, err: jni::errors::Error) -> Result<JObject<'a>> {
        let jmsg: JObject = env.new_string(err.to_string())?.into();
        env.new_object(
            "java/lang/RuntimeException",
            "(Ljava/lang/String;)V",
            &[JValue::from(jmsg)],
        )
    }

    fn throw_jni_error(env: &JNIEnv, err: jni::errors::Error) -> Result<()> {
        env.throw(JThrowable::from(Self::into_jni_error_object(env, err)?))?;
        Ok(())
    }

    fn into_openapi_error_object<'a>(
        env: &'a JNIEnv,
        err: longbridge::Error,
    ) -> Result<JObject<'a>> {
        let exception_cls = OPENAPI_EXCEPTION_CLASS.get().unwrap();
        let err = err.into_simple_error();

        let code = match err.code() {
            Some(code) => {
                env.new_object(LONG_CLASS.get().unwrap(), "(J)V", &[JValue::from(code)])?
            }
            None => JObject::null(),
        };
        let message: JObject = env.new_string(err.message())?.into();

        env.new_object(
            exception_cls,
            "(Ljava/lang/Long;Ljava/lang/String;)V",
            &[JValue::from(code), JValue::from(message)],
        )
    }

    fn throw_openapi_error(env: &JNIEnv, err: longbridge::Error) -> Result<()> {
        env.throw(JThrowable::from(Self::into_openapi_error_object(env, err)?))?;
        Ok(())
    }

    pub(crate) fn into_error_object<'a>(self, env: &'a JNIEnv) -> JObject<'a> {
        match self {
            JniError::Jni(err) => Self::into_jni_error_object(env, err),
            JniError::OpenApi(err) => Self::into_openapi_error_object(env, err),
        }
        .expect("to error object")
    }

    fn throw(self, env: &JNIEnv) {
        let res = match self {
            JniError::Jni(err) => Self::throw_jni_error(env, err),
            JniError::OpenApi(err) => Self::throw_openapi_error(env, err),
        };
        if let Err(err) = res {
            env.fatal_error(err.to_string());
        }
    }
}

pub(crate) fn jni_result<F, T>(env: &JNIEnv, err_value: T, f: F) -> T
where
    F: FnOnce() -> std::result::Result<T, JniError>,
{
    match f() {
        Ok(value) => value,
        Err(err) => {
            err.throw(env);
            err_value
        }
    }
}
