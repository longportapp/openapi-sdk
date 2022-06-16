package com.longbridge.quote;

import java.util.Arrays;

import com.longbridge.Market;

public class MarketTradingSession {
    private Market market;
    private TradingSessionInfo[] tradeSession;

    public Market getMarket() {
        return market;
    }

    public TradingSessionInfo[] getTradeSession() {
        return tradeSession;
    }

    @Override
    public String toString() {
        return "MarketTradingSession [market=" + market + ", tradeSession=" + Arrays.toString(tradeSession) + "]";
    }
}
