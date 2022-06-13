use longbridge_java_macros::impl_java_class;

impl_java_class!(
    "com/longbridge/quote/Trade",
    longbridge::quote::Trade,
    [
        price,
        volume,
        timestamp,
        trade_type,
        direction,
        trade_session
    ]
);

impl_java_class!(
    "com/longbridge/quote/Brokers",
    longbridge::quote::Brokers,
    [
        position,
        #[java(priarray)]
        broker_ids
    ]
);

impl_java_class!(
    "com/longbridge/quote/Depth",
    longbridge::quote::Depth,
    [position, price, volume, order_num]
);

impl_java_class!(
    "com/longbridge/quote/Subscription",
    longbridge::quote::Subscription,
    [symbol, sub_types]
);

impl_java_class!(
    "com/longbridge/quote/PushQuote",
    longbridge::quote::PushQuote,
    [
        last_done,
        open,
        high,
        low,
        timestamp,
        volume,
        turnover,
        trade_status,
        trade_session
    ]
);

impl_java_class!(
    "com/longbridge/quote/PushDepth",
    longbridge::quote::PushDepth,
    [
        #[java(objarray)]
        asks,
        #[java(objarray)]
        bids
    ]
);

impl_java_class!(
    "com/longbridge/quote/PushBrokers",
    longbridge::quote::PushBrokers,
    [
        #[java(objarray)]
        ask_brokers,
        #[java(objarray)]
        bid_brokers
    ]
);

impl_java_class!(
    "com/longbridge/quote/PushTrades",
    longbridge::quote::PushTrades,
    [
        #[java(objarray)]
        trades,
    ]
);

impl_java_class!(
    "com/longbridge/quote/SecurityStaticInfo",
    longbridge::quote::SecurityStaticInfo,
    [
        symbol,
        name_cn,
        name_en,
        name_hk,
        exchange,
        currency,
        lot_size,
        total_shares,
        circulating_shares,
        hk_shares,
        eps,
        eps_ttm,
        bps,
        dividend_yield,
        #[java(objarray)]
        stock_derivatives
    ]
);

impl_java_class!(
    "com/longbridge/quote/PrePostQuote",
    longbridge::quote::PrePostQuote,
    [last_done, timestamp, volume, turnover, high, low, prev_close]
);

impl_java_class!(
    "com/longbridge/quote/SecurityQuote",
    longbridge::quote::SecurityQuote,
    [
        symbol,
        last_done,
        prev_close,
        open,
        high,
        low,
        timestamp,
        volume,
        turnover,
        trade_status,
        pre_market_quote,
        post_market_quote
    ]
);

impl_java_class!(
    "com/longbridge/quote/OptionQuote",
    longbridge::quote::OptionQuote,
    [
        symbol,
        last_done,
        prev_close,
        open,
        high,
        low,
        timestamp,
        volume,
        turnover,
        trade_status,
        implied_volatility,
        open_interest,
        expiry_date,
        strike_price,
        contract_multiplier,
        contract_type,
        contract_size,
        direction,
        historical_volatility,
        underlying_symbol,
    ]
);

impl_java_class!(
    "com/longbridge/quote/WarrantQuote",
    longbridge::quote::WarrantQuote,
    [
        symbol,
        last_done,
        prev_close,
        open,
        high,
        low,
        timestamp,
        volume,
        turnover,
        trade_status,
        implied_volatility,
        expiry_date,
        last_trade_date,
        outstanding_ratio,
        outstanding_quantity,
        conversion_ratio,
        category,
        strike_price,
        upper_strike_price,
        lower_strike_price,
        call_price,
        underlying_symbol
    ]
);

impl_java_class!(
    "com/longbridge/quote/SecurityDepth",
    longbridge::quote::SecurityDepth,
    [
        #[java(objarray)]
        asks,
        #[java(objarray)]
        bids
    ]
);

impl_java_class!(
    "com/longbridge/quote/SecurityBrokers",
    longbridge::quote::SecurityBrokers,
    [
        #[java(objarray)]
        ask_brokers,
        #[java(objarray)]
        bid_brokers
    ]
);

impl_java_class!(
    "com/longbridge/quote/ParticipantInfo",
    longbridge::quote::ParticipantInfo,
    [
        #[java(priarray)]
        broker_ids,
        name_cn,
        name_en,
        name_hk
    ]
);

impl_java_class!(
    "com/longbridge/quote/IntradayLine",
    longbridge::quote::IntradayLine,
    [price, timestamp, volume, turnover, avg_price]
);

impl_java_class!(
    "com/longbridge/quote/Candlestick",
    longbridge::quote::Candlestick,
    [close, open, low, high, volume, turnover, timestamp]
);

impl_java_class!(
    "com/longbridge/quote/StrikePriceInfo",
    longbridge::quote::StrikePriceInfo,
    [price, call_symbol, put_symbol, standard]
);

impl_java_class!(
    "com/longbridge/quote/IssuerInfo",
    longbridge::quote::IssuerInfo,
    [issuer_id, name_cn, name_en, name_hk]
);

impl_java_class!(
    "com/longbridge/quote/MarketTradingSession",
    longbridge::quote::MarketTradingSession,
    [
        market,
        #[java(objarray)]
        trade_session
    ]
);

impl_java_class!(
    "com/longbridge/quote/MarketTradingSession",
    longbridge::quote::TradingSessionInfo,
    [begin_time, end_time, trade_session]
);

impl_java_class!(
    "com/longbridge/quote/MarketTradingDays",
    longbridge::quote::MarketTradingDays,
    [
        #[java(objarray)]
        trading_days,
        #[java(objarray)]
        half_trading_days
    ]
);

impl_java_class!(
    "com/longbridge/quote/RealtimeQuote",
    longbridge::quote::RealtimeQuote,
    [
        symbol,
        last_done,
        open,
        high,
        low,
        timestamp,
        volume,
        turnover,
        trade_status
    ]
);
