macro_rules! check_ctx_exists {
    ($ctx:expr) => {
        if $ctx.is_some() {
            return Err(napi::Error::from_reason("context is already open"));
        }
    };
}

macro_rules! get_ctx {
    ($ctx:expr) => {{
        match &$ctx {
            Some(ctx) => ctx,
            None => return Err(napi::Error::from_reason("context has not been opened")),
        }
    }};
}
