use std::borrow::Cow;

use jni::{
    errors::Result,
    objects::{JValue, ReleaseMode},
    JNIEnv,
};

use crate::types::{FromJValue, IntoJValue, JSignature};

pub(crate) struct PrimaryArray<T>(pub(crate) Vec<T>);

impl<T: JSignature> JSignature for PrimaryArray<T> {
    fn signature() -> Cow<'static, str> {
        format!("[{}", T::signature()).into()
    }
}

impl FromJValue for PrimaryArray<i32> {
    fn from_jvalue(env: &JNIEnv, value: JValue) -> Result<Self> {
        let array = env.get_int_array_elements(value.l()?.into_inner(), ReleaseMode::CopyBack)?;
        unsafe {
            Ok(PrimaryArray(
                std::slice::from_raw_parts(array.as_ptr(), array.size()? as usize).to_vec(),
            ))
        }
    }
}

impl IntoJValue for PrimaryArray<i32> {
    fn into_jvalue<'a>(self, env: &JNIEnv<'a>) -> Result<JValue<'a>> {
        let array = env.new_int_array(self.0.len() as i32)?;
        env.set_int_array_region(array, 0, &self.0)?;
        Ok(array.into())
    }
}

impl FromJValue for PrimaryArray<i64> {
    fn from_jvalue(env: &JNIEnv, value: JValue) -> Result<Self> {
        let array = env.get_long_array_elements(value.l()?.into_inner(), ReleaseMode::CopyBack)?;
        unsafe {
            Ok(PrimaryArray(
                std::slice::from_raw_parts(array.as_ptr(), array.size()? as usize).to_vec(),
            ))
        }
    }
}

impl IntoJValue for PrimaryArray<i64> {
    fn into_jvalue<'a>(self, env: &JNIEnv<'a>) -> Result<JValue<'a>> {
        let array = env.new_long_array(self.0.len() as i32)?;
        env.set_long_array_region(array, 0, &self.0)?;
        Ok(array.into())
    }
}

impl FromJValue for PrimaryArray<bool> {
    fn from_jvalue(env: &JNIEnv, value: JValue) -> Result<Self> {
        let array =
            env.get_boolean_array_elements(value.l()?.into_inner(), ReleaseMode::CopyBack)?;
        unsafe {
            Ok(PrimaryArray(
                std::slice::from_raw_parts(array.as_ptr(), array.size()? as usize)
                    .iter()
                    .copied()
                    .map(|value| value > 0)
                    .collect(),
            ))
        }
    }
}

impl IntoJValue for PrimaryArray<bool> {
    fn into_jvalue<'a>(self, env: &JNIEnv<'a>) -> Result<JValue<'a>> {
        let array = env.new_boolean_array(self.0.len() as i32)?;
        let buf = self
            .0
            .into_iter()
            .map(|value| if value { 1u8 } else { 0 })
            .collect::<Vec<_>>();
        env.set_boolean_array_region(array, 0, &buf)?;
        Ok(array.into())
    }
}
