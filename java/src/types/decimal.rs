use std::borrow::Cow;

use jni::{errors::Result, objects::JValue, JNIEnv};
use longbridge::Decimal;

use crate::{
    init::DECIMAL_CLASS,
    types::{FromJValue, IntoJValue, JClassName, JSignature},
};

impl JClassName for Decimal {
    const CLASSNAME: &'static str = "java/math/BigDecimal";
}

impl JSignature for Decimal {
    fn signature() -> Cow<'static, str> {
        "Ljava/math/BigDecimal;".into()
    }
}

impl FromJValue for Decimal {
    fn from_jvalue(env: &JNIEnv, value: JValue) -> Result<Self> {
        let obj = value.l()?;
        let value = env.call_method(obj, "toString", "()Ljava/lang/String;", &[])?;
        let value = String::from_jvalue(env, value)?;
        Ok(value.parse().expect("valid decimal"))
    }
}

impl IntoJValue for Decimal {
    fn into_jvalue<'a>(self, env: &JNIEnv<'a>) -> Result<JValue<'a>> {
        let str = env.new_string(self.to_string())?;
        let obj = env.new_object(
            DECIMAL_CLASS.get().unwrap(),
            "(Ljava/lang/String;)V",
            &[JValue::from(str)],
        )?;
        Ok(JValue::from(obj))
    }
}
