use std::sync::Arc;

use jni::{
    descriptors::Desc,
    errors::Result,
    objects::{GlobalRef, JClass, JObject, JString, JValue},
    sys::{jboolean, jobjectArray},
    JNIEnv, JavaVM,
};
use longbridge::{
    quote::{PushEvent, PushEventDetail, SubFlags},
    Config, QuoteContext,
};

use crate::{
    async_util,
    error::jni_result,
    types::{set_field, FromJValue, IntoJValue, ObjectArray},
};

fn send_push_event(jvm: &JavaVM, push_handler: &GlobalRef, event: PushEvent) -> Result<()> {
    let env = jvm.attach_current_thread().unwrap();

    match event.detail {
        PushEventDetail::Quote(push_quote) => {
            env.call_method(
                push_handler,
                "onQuote",
                "(Ljava/lang/String;Lcom/longbridge/quote/PushQuote;)V",
                &[
                    event.symbol.into_jvalue(&env)?,
                    push_quote.into_jvalue(&env)?,
                ],
            )?;
        }
        PushEventDetail::Depth(push_depth) => {
            env.call_method(
                push_handler,
                "onDepth",
                "(Ljava/lang/String;Lcom/longbridge/quote/PushDepth;)V",
                &[
                    event.symbol.into_jvalue(&env)?,
                    push_depth.into_jvalue(&env)?,
                ],
            )?;
        }
        PushEventDetail::Brokers(push_brokers) => {
            env.call_method(
                push_handler,
                "onBrokers",
                "(Ljava/lang/String;Lcom/longbridge/quote/PushBrokers;)V",
                &[
                    event.symbol.into_jvalue(&env)?,
                    push_brokers.into_jvalue(&env)?,
                ],
            )?;
        }
        PushEventDetail::Trade(push_trades) => {
            env.call_method(
                push_handler,
                "onTrades",
                "(Ljava/lang/String;Lcom/longbridge/quote/PushTrades;)V",
                &[
                    event.symbol.into_jvalue(&env)?,
                    push_trades.into_jvalue(&env)?,
                ],
            )?;
        }
    }

    Ok(())
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_newQuoteContext(
    env: JNIEnv,
    _class: JClass,
    config: i64,
    push_handler: JObject,
    callback: JObject,
) {
    struct ContextObjRef(i64);

    impl IntoJValue for ContextObjRef {
        fn into_jvalue<'a>(self, env: &JNIEnv<'a>) -> Result<JValue<'a>> {
            let ctx_cls: JClass = "com/longbridge/quote/QuoteContext".lookup(env)?;
            let ctx_obj = env.new_object(ctx_cls, "()V", &[])?;
            set_field(env, ctx_obj, "raw", self.0)?;
            Ok(JValue::from(ctx_obj))
        }
    }

    jni_result(&env, (), || {
        let config = Arc::new((&*(config as *const Config)).clone());
        let push_handler = env.new_global_ref(push_handler)?;
        let jvm = env.get_java_vm()?;

        async_util::execute(&env, callback, async move {
            let (ctx, mut receiver) = QuoteContext::try_new(config).await?;
            let ctx_ref = ContextObjRef(Box::into_raw(Box::new(ctx)) as i64);

            tokio::spawn(async move {
                while let Some(event) = receiver.recv().await {
                    let _ = send_push_event(&jvm, &push_handler, event);
                }
            });

            Ok(ctx_ref)
        })?;

        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_freeQuoteContext(
    _env: JNIEnv,
    _class: JClass,
    ctx: i64,
) {
    let _ = Box::from_raw(ctx as *mut QuoteContext);
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_quoteContextSubscribe(
    env: JNIEnv,
    _class: JClass,
    context: i64,
    symbols: jobjectArray,
    flags: i32,
    is_first_push: jboolean,
    callback: JObject,
) {
    jni_result(&env, (), || {
        let context = &*(context as *const QuoteContext);
        let symbols: ObjectArray<String> = FromJValue::from_jvalue(&env, symbols.into())?;
        let sub_flags = SubFlags::from_bits(flags as u8).unwrap_or(SubFlags::empty());
        async_util::execute(&env, callback, async move {
            Ok(context
                .subscribe(symbols.0, sub_flags, is_first_push > 0)
                .await?)
        })?;
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_quoteContextUnsubscribe(
    env: JNIEnv,
    _class: JClass,
    context: i64,
    symbols: jobjectArray,
    flags: i32,
    callback: JObject,
) {
    jni_result(&env, (), || {
        let context = &*(context as *const QuoteContext);
        let symbols: ObjectArray<String> = FromJValue::from_jvalue(&env, symbols.into())?;
        let sub_flags = SubFlags::from_bits(flags as u8).unwrap_or(SubFlags::empty());
        async_util::execute(&env, callback, async move {
            Ok(context.unsubscribe(symbols.0, sub_flags).await?)
        })?;
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_quoteContextSubscriptions(
    env: JNIEnv,
    _class: JClass,
    context: i64,
    callback: JObject,
) {
    jni_result(&env, (), || {
        let context = &*(context as *const QuoteContext);
        async_util::execute(&env, callback, async move {
            let list = context.subscriptions().await?;
            Ok(ObjectArray(list))
        })?;
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_quoteContextStaticInfo(
    env: JNIEnv,
    _class: JClass,
    context: i64,
    symbols: jobjectArray,
    callback: JObject,
) {
    jni_result(&env, (), || {
        let context = &*(context as *const QuoteContext);
        let symbols: ObjectArray<String> = FromJValue::from_jvalue(&env, symbols.into())?;
        async_util::execute(&env, callback, async move {
            let list = context.static_info(symbols.0).await?;
            Ok(ObjectArray(list))
        })?;
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_quoteContextQuote(
    env: JNIEnv,
    _class: JClass,
    context: i64,
    symbols: jobjectArray,
    callback: JObject,
) {
    jni_result(&env, (), || {
        let context = &*(context as *const QuoteContext);
        let symbols: ObjectArray<String> = FromJValue::from_jvalue(&env, symbols.into())?;
        async_util::execute(&env, callback, async move {
            let list = context.quote(symbols.0).await?;
            Ok(ObjectArray(list))
        })?;
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_quoteContextOptionQuote(
    env: JNIEnv,
    _class: JClass,
    context: i64,
    symbols: jobjectArray,
    callback: JObject,
) {
    jni_result(&env, (), || {
        let context = &*(context as *const QuoteContext);
        let symbols: ObjectArray<String> = FromJValue::from_jvalue(&env, symbols.into())?;
        async_util::execute(&env, callback, async move {
            let list = context.option_quote(symbols.0).await?;
            Ok(ObjectArray(list))
        })?;
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_quoteContextWarrantQuote(
    env: JNIEnv,
    _class: JClass,
    context: i64,
    symbols: jobjectArray,
    callback: JObject,
) {
    jni_result(&env, (), || {
        let context = &*(context as *const QuoteContext);
        let symbols: ObjectArray<String> = FromJValue::from_jvalue(&env, symbols.into())?;
        async_util::execute(&env, callback, async move {
            let list = context.warrant_quote(symbols.0).await?;
            Ok(ObjectArray(list))
        })?;
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_quoteContextDepth(
    env: JNIEnv,
    _class: JClass,
    context: i64,
    symbol: JString,
    callback: JObject,
) {
    jni_result(&env, (), || {
        let context = &*(context as *const QuoteContext);
        let symbol: String = FromJValue::from_jvalue(&env, symbol.into())?;
        async_util::execute(
            &env,
            callback,
            async move { Ok(context.depth(symbol).await?) },
        )?;
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_quoteContextBrokers(
    env: JNIEnv,
    _class: JClass,
    context: i64,
    symbol: JString,
    callback: JObject,
) {
    jni_result(&env, (), || {
        let context = &*(context as *const QuoteContext);
        let symbol: String = FromJValue::from_jvalue(&env, symbol.into())?;
        async_util::execute(
            &env,
            callback,
            async move { Ok(context.brokers(symbol).await?) },
        )?;
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_quoteContextParticipants(
    env: JNIEnv,
    _class: JClass,
    context: i64,
    callback: JObject,
) {
    jni_result(&env, (), || {
        let context = &*(context as *const QuoteContext);
        async_util::execute(&env, callback, async move {
            Ok(ObjectArray(context.participants().await?))
        })?;
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_quoteContextTrades(
    env: JNIEnv,
    _class: JClass,
    context: i64,
    symbol: JString,
    count: i32,
    callback: JObject,
) {
    jni_result(&env, (), || {
        let context = &*(context as *const QuoteContext);
        let symbol: String = FromJValue::from_jvalue(&env, symbol.into())?;
        async_util::execute(&env, callback, async move {
            Ok(ObjectArray(
                context.trades(symbol, count.max(0) as usize).await?,
            ))
        })?;
        Ok(())
    })
}
