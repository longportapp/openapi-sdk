use time::{macros::time, Duration};
use time_tz::timezones::db;

use crate::{
    market::Market, TradeSession, TRADE_SESSION_NORMAL, TRADE_SESSION_OVERNIGHT,
    TRADE_SESSION_POST, TRADE_SESSION_PRE,
};

pub const HK: Market = Market {
    timezone: db::asia::HONG_KONG,
    session_types: &[TRADE_SESSION_NORMAL],
    trade_sessions: &[
        // Normal
        &[
            TradeSession::new(time!(9:30:00), time!(12:00:00)),
            TradeSession::new(time!(13:00:00), time!(16:00:00)).with_timeout(Duration::minutes(10)),
        ],
    ],
    half_trade_sessions: &[
        // Normal
        &[TradeSession::new(time!(9:30:00), time!(12:00:00)).with_timeout(Duration::minutes(10))],
    ],
    lot_size: 1,
};

pub const SG: Market = Market {
    timezone: db::asia::SINGAPORE,
    session_types: &[TRADE_SESSION_NORMAL],
    trade_sessions: &[
        // Normal
        &[
            TradeSession::new(time!(9:00:00), time!(12:00:00)),
            TradeSession::new(time!(13:00:00), time!(17:15:00)).with_timeout(Duration::minutes(5)),
        ],
    ],
    half_trade_sessions: &[
        // Normal
        &[TradeSession::new(time!(9:00:00), time!(12:15:00)).with_timeout(Duration::minutes(5))],
    ],
    lot_size: 1,
};

pub const CN: Market = Market {
    timezone: db::asia::SHANGHAI,
    session_types: &[TRADE_SESSION_NORMAL],
    trade_sessions: &[
        // Normal
        &[
            TradeSession::new(time!(9:30:00), time!(11:30:00)),
            TradeSession::new(time!(13:00:00), time!(15:00:00)).with_timeout(Duration::minutes(10)),
        ],
    ],
    half_trade_sessions: &[],
    lot_size: 100,
};

pub const US: Market = Market {
    timezone: db::america::NEW_YORK,
    session_types: &[
        TRADE_SESSION_NORMAL,
        TRADE_SESSION_PRE,
        TRADE_SESSION_POST,
        TRADE_SESSION_OVERNIGHT,
    ],
    trade_sessions: &[
        // Normal
        &[TradeSession::new(time!(9:30:00), time!(16:00:00))],
        // Pre
        &[TradeSession::new(time!(4:00:00), time!(9:30:00))],
        // Post
        &[TradeSession::new(time!(16:00:00), time!(20:00:00))],
        // Overnight
        &[
            TradeSession::new(time!(0:00:00), time!(4:00:00)),
            TradeSession::new(time!(20:00:00), time!(23:59:59)).with_inclusive(),
        ],
    ],
    half_trade_sessions: &[
        // Normal
        &[TradeSession::new(time!(9:30:00), time!(13:00:00))],
        // Pre
        &[TradeSession::new(time!(4:00:00), time!(9:30:00))],
        // Post
        &[TradeSession::new(time!(13:00:00), time!(17:00:00))],
        // Overnight
        &[
            TradeSession::new(time!(0:00:00), time!(4:00:00)),
            TradeSession::new(time!(20:00:00), time!(23:59:59)).with_inclusive(),
        ],
    ],
    lot_size: 1,
};

pub const US_OPTION: Market = Market {
    timezone: db::america::NEW_YORK,
    session_types: &[TRADE_SESSION_NORMAL],
    trade_sessions: &[
        // Normal
        &[TradeSession::new(time!(9:30:00), time!(16:15:00))],
    ],
    half_trade_sessions: &[
        // Normal
        &[TradeSession::new(time!(9:30:00), time!(13:15:00))],
    ],
    lot_size: 1,
};
