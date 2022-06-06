macro_rules! enum_type {
    ($ident:ident, $classname:literal, [$($item:ident),*]) => {
        impl crate::types::JClassName for $ident {
            const CLASSNAME: &'static str = $classname;
        }

        impl crate::types::JSignature for $ident {
            fn signature() -> std::borrow::Cow<'static, str> {
                concat!("L", $classname, ";").into()
            }
        }

        impl crate::types::FromJValue for $ident {
            fn from_jvalue(
                env: &jni::JNIEnv,
                value: jni::objects::JValue,
            ) -> jni::errors::Result<Self> {
                use $ident::*;
                use jni::descriptors::Desc;

                let cls: jni::objects::JClass = $classname.lookup(env).expect(concat!($classname, " exists"));
                let value = value.l()?;

                $(
                    let r = env.get_static_field(cls, stringify!($item), concat!("L", $classname, ";"))?.l()?;
                    if env.is_same_object(value, r)? {
                        return Ok($item);
                    }
                )*

                panic!("invalid enum value")
            }
        }

        impl crate::types::IntoJValue for $ident {
            fn into_jvalue<'a>(
                self,
                env: &jni::JNIEnv<'a>,
            ) -> jni::errors::Result<jni::objects::JValue<'a>> {
                use jni::descriptors::Desc;
                use $ident::*;

                let cls: jni::objects::JClass = $classname.lookup(env).expect(concat!($classname, " exists"));

                match self {
                    $(
                    $item => env.get_static_field(cls, stringify!($item), concat!("L", $classname, ";")),
                    )*
                }
            }
        }
    };
}
