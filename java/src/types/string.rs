use jni::{
    errors::Result,
    objects::{JString, JValue},
    JNIEnv,
};

use crate::types::{FromJValue, IntoJValue, JClassName, JSignature};

impl JClassName for String {
    const CLASSNAME: &'static str = "java/lang/String";
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
