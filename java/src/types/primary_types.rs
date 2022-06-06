use std::borrow::Cow;

use jni::{errors::Result, objects::JValue, JNIEnv};

use crate::types::{FromJValue, IntoJValue, JSignature};

impl JSignature for i32 {
    fn signature() -> Cow<'static, str> {
        "I".into()
    }
}

impl FromJValue for i32 {
    #[inline]
    fn from_jvalue(_env: &JNIEnv, value: JValue) -> Result<Self> {
        value.i()
    }
}

impl IntoJValue for i32 {
    #[inline]
    fn into_jvalue<'a>(self, _env: &JNIEnv<'a>) -> Result<JValue<'a>> {
        Ok(JValue::from(self))
    }
}

impl JSignature for i64 {
    fn signature() -> Cow<'static, str> {
        "J".into()
    }
}

impl FromJValue for i64 {
    #[inline]
    fn from_jvalue(_env: &JNIEnv, value: JValue) -> Result<Self> {
        value.j()
    }
}

impl IntoJValue for i64 {
    #[inline]
    fn into_jvalue<'a>(self, _env: &JNIEnv<'a>) -> Result<JValue<'a>> {
        Ok(JValue::from(self))
    }
}

impl JSignature for bool {
    fn signature() -> Cow<'static, str> {
        "Z".into()
    }
}

impl FromJValue for bool {
    #[inline]
    fn from_jvalue(_env: &JNIEnv, value: JValue) -> Result<Self> {
        value.z()
    }
}

impl IntoJValue for bool {
    #[inline]
    fn into_jvalue<'a>(self, _env: &JNIEnv<'a>) -> Result<JValue<'a>> {
        Ok(JValue::from(self))
    }
}
