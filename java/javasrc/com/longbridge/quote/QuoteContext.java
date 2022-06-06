package com.longbridge.quote;

import java.util.concurrent.CompletableFuture;
import com.longbridge.*;

public class QuoteContext implements AutoCloseable {
    private long raw;

    public static CompletableFuture<QuoteContext> create(Config config, QuotePushHandler handler)
            throws OpenApiException {
        CompletableFuture<QuoteContext> fut = new CompletableFuture<QuoteContext>();

        SdkNative.newQuoteContext(config.getRaw(), handler, (err, obj) -> {
            if (err == null) {
                fut.complete((QuoteContext) obj);
            } else {
                fut.completeExceptionally((Throwable) err);
            }
        });

        return fut;
    }

    @Override
    public void close() throws Exception {
        SdkNative.freeQuoteContext(raw);
    }

    public CompletableFuture<Void> subscribe(String[] symbols, int flags, boolean isFirstPush) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextSubscribe(this.raw, symbols, flags, isFirstPush, callback);
        });
    }

    public CompletableFuture<Void> unsubscribe(String[] symbols, int flags) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextUnsubscribe(this.raw, symbols, flags, callback);
        });
    }

    public CompletableFuture<Subscription[]> subscrptions(String[] symbols, int flags) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextSubscriptions(this.raw, callback);
        });
    }

    public CompletableFuture<SecurityStaticInfo[]> staticInfo(String[] symbols) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextStaticInfo(this.raw, symbols, callback);
        });
    }

    public CompletableFuture<SecurityQuote[]> quote(String[] symbols) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextQuote(this.raw, symbols, callback);
        });
    }

    public CompletableFuture<OptionQuote[]> optionQuote(String[] symbols) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextOptionQuote(this.raw, symbols, callback);
        });
    }

    public CompletableFuture<SecurityDepth> depth(String symbol) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextDepth(this.raw, symbol, callback);
        });
    }

    public CompletableFuture<SecurityBrokers> brokers(String symbol) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextBrokers(this.raw, symbol, callback);
        });
    }

    public CompletableFuture<SecurityBrokers> participants() throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextParticipants(this.raw, callback);
        });
    }

    public CompletableFuture<SecurityBrokers> trades(String symbol, int count) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextTrades(this.raw, symbol, count, callback);
        });
    }
}
