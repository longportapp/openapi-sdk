package com.longbridge.quote;

public interface QuotePushHandler {
    void onQuote(String symbol, PushQuote quote);

    void onDepth(String symbol, PushDepth depth);

    void onBrokers(String symbol, PushBrokers brokers);

    void onTrades(String symbol, PushTrades trades);
}
