use longbridge::quote::{
    DerivativeType, OptionDirection, OptionType, TradeDirection, TradeSession, TradeStatus,
    WarrantType,
};

enum_type!(
    TradeStatus,
    "com/longbridge/quote/TradeStatus",
    [
        Normal,
        Halted,
        Delisted,
        Fuse,
        PrepareList,
        CodeMoved,
        ToBeOpened,
        SplitStockHalts,
        Expired,
        WarrantPrepareList,
        SuspendTrade
    ]
);

enum_type!(
    TradeSession,
    "com/longbridge/quote/TradeSession",
    [NormalTrade, PreTrade, PostTrade]
);

enum_type!(
    TradeDirection,
    "com/longbridge/quote/TradeDirection",
    [Neutral, Down, Up]
);

enum_type!(
    DerivativeType,
    "com/longbridge/quote/DerivativeType",
    [Option, Warrant]
);

enum_type!(
    OptionType,
    "com/longbridge/quote/OptionType",
    [Unknown, American, Europe]
);

enum_type!(
    OptionDirection,
    "com/longbridge/quote/OptionDirection",
    [Unknown, Put, Call]
);

enum_type!(
    WarrantType,
    "com/longbridge/quote/WarrantType",
    [Unknown, Call, Put, Bull, Bear, Inline]
);
