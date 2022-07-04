use jni::{
    errors::Result,
    objects::{JString, JValue},
    JNIEnv,
};

use crate::{
    init::STRING_CLASS,
    types::{ClassLoader, FromJValue, IntoJValue, JSignature},
};

impl ClassLoader for String {
    fn init(_env: &JNIEnv) {}

    fn class_ref() -> jni::objects::GlobalRef {
        STRING_CLASS.get().cloned().unwrap()
    }
}

impl JSignature for String {
    fn signature() -> std::borrow::Cow<'static, str> {
        "Ljava/lang/String;".into()
    }
}

impl FromJValue for String {
    fn from_jvalue(env: &JNIEnv, value: JValue) -> Result<Self> {
        let obj = value.l()?;
        let str = env.get_string(JString::from(obj))?;
        Ok(str
            .to_str()
            .map(ToString::to_string)
            .expect("valid utf8 string"))
    }
}

impl IntoJValue for String {
    fn into_jvalue<'a>(self, env: &JNIEnv<'a>) -> Result<JValue<'a>> {
        env.new_string(self).map(JValue::from)
    }
}
