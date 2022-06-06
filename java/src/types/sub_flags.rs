use std::borrow::Cow;

use jni::{errors::Result, objects::JValue, JNIEnv};
use longbridge::quote::SubFlags;

use crate::types::{IntoJValue, JSignature};

impl JSignature for SubFlags {
    fn signature() -> Cow<'static, str> {
        "I".into()
    }
}

impl IntoJValue for SubFlags {
    fn into_jvalue<'a>(self, _env: &JNIEnv<'a>) -> Result<JValue<'a>> {
        Ok(JValue::from(self.bits() as i32))
    }
}
