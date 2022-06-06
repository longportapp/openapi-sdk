package com.longbridge;

import com.longbridge.quote.*;

public class SdkNative {
        static native void init();

        public static native long newConfig(String appKey, String appSecret, String accessToken, String httpUrl,
                        String quoteWsUrl, String tradeWsUrl);

        public static native long newConfigFromEnv();

        public static native void freeConfig(long config);

        public static native void newQuoteContext(long config, QuotePushHandler pushHandler, AsyncCallback callback);

        public static native void freeQuoteContext(long config);

        public static native void quoteContextSubscribe(long context, String[] symbols, int flags, boolean isFirstPush,
                        AsyncCallback callback);

        public static native void quoteContextUnsubscribe(long context, String[] symbols, int flags,
                        AsyncCallback callback);

        public static native void quoteContextSubscriptions(long context, AsyncCallback callback);

        public static native void quoteContextStaticInfo(long context, String[] symbols, AsyncCallback callback);

        public static native void quoteContextQuote(long context, String[] symbols, AsyncCallback callback);

        public static native void quoteContextOptionQuote(long context, String[] symbols, AsyncCallback callback);

        public static native void quoteContextWarrantQuote(long context, String[] symbols, AsyncCallback callback);

        public static native void quoteContextDepth(long context, String symbol, AsyncCallback callback);

        public static native void quoteContextBrokers(long context, String symbol, AsyncCallback callback);

        public static native void quoteContextParticipants(long context, AsyncCallback callback);

        public static native void quoteContextTrades(long context, String symbol, int count, AsyncCallback callback);

        static {
                System.loadLibrary("longbridge_java");
                SdkNative.init();
        }
}
