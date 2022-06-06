use jni::{
    descriptors::Desc,
    objects::{GlobalRef, JClass, JValue},
    JNIEnv,
};
use once_cell::sync::OnceCell;

pub(crate) static DECIMAL_CLASS: OnceCell<GlobalRef> = OnceCell::new();
pub(crate) static TIME_ZONE_ID: OnceCell<GlobalRef> = OnceCell::new();

#[no_mangle]
pub extern "system" fn Java_com_longbridge_SdkNative_init(env: JNIEnv, _class: JClass) {
    {
        let cls: JClass = "java/math/BigDecimal"
            .lookup(&env)
            .expect("java/math/BigDecimal exists");
        let _ = DECIMAL_CLASS.set(env.new_global_ref(cls).unwrap());
    }

    {
        let utc = env.new_string("UTC").unwrap();
        let zone_id = env
            .call_static_method(
                "java/time/ZoneId",
                "of",
                "(Ljava/lang/String;)Ljava/time/ZoneId;",
                &[JValue::from(utc)],
            )
            .expect("create zone id");
        let _ = TIME_ZONE_ID.set(env.new_global_ref(zone_id.l().unwrap()).unwrap());
    }
}
