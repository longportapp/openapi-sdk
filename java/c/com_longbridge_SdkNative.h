/* DO NOT EDIT THIS FILE - it is machine generated */
#include <jni.h>
/* Header for class com_longbridge_SdkNative */

#ifndef _Included_com_longbridge_SdkNative
#define _Included_com_longbridge_SdkNative
#ifdef __cplusplus
extern "C" {
#endif
/*
 * Class:     com_longbridge_SdkNative
 * Method:    init
 * Signature: ()V
 */
JNIEXPORT void JNICALL Java_com_longbridge_SdkNative_init
  (JNIEnv *, jclass);

/*
 * Class:     com_longbridge_SdkNative
 * Method:    newConfig
 * Signature: (Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)J
 */
JNIEXPORT jlong JNICALL Java_com_longbridge_SdkNative_newConfig
  (JNIEnv *, jclass, jstring, jstring, jstring, jstring, jstring, jstring);

/*
 * Class:     com_longbridge_SdkNative
 * Method:    newConfigFromEnv
 * Signature: ()J
 */
JNIEXPORT jlong JNICALL Java_com_longbridge_SdkNative_newConfigFromEnv
  (JNIEnv *, jclass);

/*
 * Class:     com_longbridge_SdkNative
 * Method:    freeConfig
 * Signature: (J)V
 */
JNIEXPORT void JNICALL Java_com_longbridge_SdkNative_freeConfig
  (JNIEnv *, jclass, jlong);

/*
 * Class:     com_longbridge_SdkNative
 * Method:    newQuoteContext
 * Signature: (JLcom/longbridge/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longbridge_SdkNative_newQuoteContext
  (JNIEnv *, jclass, jlong, jobject);

/*
 * Class:     com_longbridge_SdkNative
 * Method:    freeQuoteContext
 * Signature: (J)V
 */
JNIEXPORT void JNICALL Java_com_longbridge_SdkNative_freeQuoteContext
  (JNIEnv *, jclass, jlong);

/*
 * Class:     com_longbridge_SdkNative
 * Method:    quoteContextSetOnQuote
 * Signature: (JLcom/longbridge/quote/QuoteHandler;)V
 */
JNIEXPORT void JNICALL Java_com_longbridge_SdkNative_quoteContextSetOnQuote
  (JNIEnv *, jclass, jlong, jobject);

/*
 * Class:     com_longbridge_SdkNative
 * Method:    quoteContextSetOnDepth
 * Signature: (JLcom/longbridge/quote/DepthHandler;)V
 */
JNIEXPORT void JNICALL Java_com_longbridge_SdkNative_quoteContextSetOnDepth
  (JNIEnv *, jclass, jlong, jobject);

/*
 * Class:     com_longbridge_SdkNative
 * Method:    quoteContextSetOnBrokers
 * Signature: (JLcom/longbridge/quote/BrokersHandler;)V
 */
JNIEXPORT void JNICALL Java_com_longbridge_SdkNative_quoteContextSetOnBrokers
  (JNIEnv *, jclass, jlong, jobject);

/*
 * Class:     com_longbridge_SdkNative
 * Method:    quoteContextSetOnTrades
 * Signature: (JLcom/longbridge/quote/TradesHandler;)V
 */
JNIEXPORT void JNICALL Java_com_longbridge_SdkNative_quoteContextSetOnTrades
  (JNIEnv *, jclass, jlong, jobject);

/*
 * Class:     com_longbridge_SdkNative
 * Method:    quoteContextSetOnCandlestick
 * Signature: (JLcom/longbridge/quote/CandlestickHandler;)V
 */
JNIEXPORT void JNICALL Java_com_longbridge_SdkNative_quoteContextSetOnCandlestick
  (JNIEnv *, jclass, jlong, jobject);

/*
 * Class:     com_longbridge_SdkNative
 * Method:    quoteContextSubscribe
 * Signature: (J[Ljava/lang/String;IZLcom/longbridge/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longbridge_SdkNative_quoteContextSubscribe
  (JNIEnv *, jclass, jlong, jobjectArray, jint, jboolean, jobject);

/*
 * Class:     com_longbridge_SdkNative
 * Method:    quoteContextUnsubscribe
 * Signature: (J[Ljava/lang/String;ILcom/longbridge/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longbridge_SdkNative_quoteContextUnsubscribe
  (JNIEnv *, jclass, jlong, jobjectArray, jint, jobject);

/*
 * Class:     com_longbridge_SdkNative
 * Method:    quoteContextSubscribeCandlesticks
 * Signature: (JLjava/lang/String;Lcom/longbridge/quote/Period;Lcom/longbridge/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longbridge_SdkNative_quoteContextSubscribeCandlesticks
  (JNIEnv *, jclass, jlong, jstring, jobject, jobject);

/*
 * Class:     com_longbridge_SdkNative
 * Method:    quoteContextUnsubscribeCandlesticks
 * Signature: (JLjava/lang/String;Lcom/longbridge/quote/Period;Lcom/longbridge/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longbridge_SdkNative_quoteContextUnsubscribeCandlesticks
  (JNIEnv *, jclass, jlong, jstring, jobject, jobject);

/*
 * Class:     com_longbridge_SdkNative
 * Method:    quoteContextSubscriptions
 * Signature: (JLcom/longbridge/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longbridge_SdkNative_quoteContextSubscriptions
  (JNIEnv *, jclass, jlong, jobject);

/*
 * Class:     com_longbridge_SdkNative
 * Method:    quoteContextStaticInfo
 * Signature: (J[Ljava/lang/String;Lcom/longbridge/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longbridge_SdkNative_quoteContextStaticInfo
  (JNIEnv *, jclass, jlong, jobjectArray, jobject);

/*
 * Class:     com_longbridge_SdkNative
 * Method:    quoteContextQuote
 * Signature: (J[Ljava/lang/String;Lcom/longbridge/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longbridge_SdkNative_quoteContextQuote
  (JNIEnv *, jclass, jlong, jobjectArray, jobject);

/*
 * Class:     com_longbridge_SdkNative
 * Method:    quoteContextOptionQuote
 * Signature: (J[Ljava/lang/String;Lcom/longbridge/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longbridge_SdkNative_quoteContextOptionQuote
  (JNIEnv *, jclass, jlong, jobjectArray, jobject);

/*
 * Class:     com_longbridge_SdkNative
 * Method:    quoteContextWarrantQuote
 * Signature: (J[Ljava/lang/String;Lcom/longbridge/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longbridge_SdkNative_quoteContextWarrantQuote
  (JNIEnv *, jclass, jlong, jobjectArray, jobject);

/*
 * Class:     com_longbridge_SdkNative
 * Method:    quoteContextDepth
 * Signature: (JLjava/lang/String;Lcom/longbridge/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longbridge_SdkNative_quoteContextDepth
  (JNIEnv *, jclass, jlong, jstring, jobject);

/*
 * Class:     com_longbridge_SdkNative
 * Method:    quoteContextBrokers
 * Signature: (JLjava/lang/String;Lcom/longbridge/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longbridge_SdkNative_quoteContextBrokers
  (JNIEnv *, jclass, jlong, jstring, jobject);

/*
 * Class:     com_longbridge_SdkNative
 * Method:    quoteContextParticipants
 * Signature: (JLcom/longbridge/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longbridge_SdkNative_quoteContextParticipants
  (JNIEnv *, jclass, jlong, jobject);

/*
 * Class:     com_longbridge_SdkNative
 * Method:    quoteContextTrades
 * Signature: (JLjava/lang/String;ILcom/longbridge/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longbridge_SdkNative_quoteContextTrades
  (JNIEnv *, jclass, jlong, jstring, jint, jobject);

/*
 * Class:     com_longbridge_SdkNative
 * Method:    quoteContextIntraday
 * Signature: (JLjava/lang/String;Lcom/longbridge/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longbridge_SdkNative_quoteContextIntraday
  (JNIEnv *, jclass, jlong, jstring, jobject);

/*
 * Class:     com_longbridge_SdkNative
 * Method:    quoteContextCandlesticks
 * Signature: (JLjava/lang/String;Lcom/longbridge/quote/Period;ILcom/longbridge/quote/AdjustType;Lcom/longbridge/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longbridge_SdkNative_quoteContextCandlesticks
  (JNIEnv *, jclass, jlong, jstring, jobject, jint, jobject, jobject);

/*
 * Class:     com_longbridge_SdkNative
 * Method:    quoteContextOptionChainExpiryDateList
 * Signature: (JLjava/lang/String;Lcom/longbridge/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longbridge_SdkNative_quoteContextOptionChainExpiryDateList
  (JNIEnv *, jclass, jlong, jstring, jobject);

/*
 * Class:     com_longbridge_SdkNative
 * Method:    quoteContextOptionChainInfoByDate
 * Signature: (JLjava/lang/String;Ljava/time/LocalDate;Lcom/longbridge/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longbridge_SdkNative_quoteContextOptionChainInfoByDate
  (JNIEnv *, jclass, jlong, jstring, jobject, jobject);

/*
 * Class:     com_longbridge_SdkNative
 * Method:    quoteContextWarrantIssuers
 * Signature: (JLcom/longbridge/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longbridge_SdkNative_quoteContextWarrantIssuers
  (JNIEnv *, jclass, jlong, jobject);

/*
 * Class:     com_longbridge_SdkNative
 * Method:    quoteContextTradingSession
 * Signature: (JLcom/longbridge/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longbridge_SdkNative_quoteContextTradingSession
  (JNIEnv *, jclass, jlong, jobject);

/*
 * Class:     com_longbridge_SdkNative
 * Method:    quoteContextTradingDays
 * Signature: (JLcom/longbridge/Market;Ljava/time/LocalDate;Ljava/time/LocalDate;Lcom/longbridge/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longbridge_SdkNative_quoteContextTradingDays
  (JNIEnv *, jclass, jlong, jobject, jobject, jobject, jobject);

/*
 * Class:     com_longbridge_SdkNative
 * Method:    quoteContextCapitalFlow
 * Signature: (JLjava/lang/String;Lcom/longbridge/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longbridge_SdkNative_quoteContextCapitalFlow
  (JNIEnv *, jclass, jlong, jstring, jobject);

/*
 * Class:     com_longbridge_SdkNative
 * Method:    quoteContextCapitalDistribution
 * Signature: (JLjava/lang/String;Lcom/longbridge/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longbridge_SdkNative_quoteContextCapitalDistribution
  (JNIEnv *, jclass, jlong, jstring, jobject);

/*
 * Class:     com_longbridge_SdkNative
 * Method:    quoteContextWatchList
 * Signature: (JLcom/longbridge/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longbridge_SdkNative_quoteContextWatchList
  (JNIEnv *, jclass, jlong, jobject);

/*
 * Class:     com_longbridge_SdkNative
 * Method:    quoteContextRealtimeQuote
 * Signature: (J[Ljava/lang/String;Lcom/longbridge/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longbridge_SdkNative_quoteContextRealtimeQuote
  (JNIEnv *, jclass, jlong, jobjectArray, jobject);

/*
 * Class:     com_longbridge_SdkNative
 * Method:    quoteContextRealtimeDepth
 * Signature: (JLjava/lang/String;Lcom/longbridge/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longbridge_SdkNative_quoteContextRealtimeDepth
  (JNIEnv *, jclass, jlong, jstring, jobject);

/*
 * Class:     com_longbridge_SdkNative
 * Method:    quoteContextRealtimeBrokers
 * Signature: (JLjava/lang/String;Lcom/longbridge/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longbridge_SdkNative_quoteContextRealtimeBrokers
  (JNIEnv *, jclass, jlong, jstring, jobject);

/*
 * Class:     com_longbridge_SdkNative
 * Method:    quoteContextRealtimeTrades
 * Signature: (JLjava/lang/String;ILcom/longbridge/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longbridge_SdkNative_quoteContextRealtimeTrades
  (JNIEnv *, jclass, jlong, jstring, jint, jobject);

/*
 * Class:     com_longbridge_SdkNative
 * Method:    quoteContextRealtimeCandlesticks
 * Signature: (JLjava/lang/String;Lcom/longbridge/quote/Period;ILcom/longbridge/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longbridge_SdkNative_quoteContextRealtimeCandlesticks
  (JNIEnv *, jclass, jlong, jstring, jobject, jint, jobject);

/*
 * Class:     com_longbridge_SdkNative
 * Method:    newTradeContext
 * Signature: (JLcom/longbridge/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longbridge_SdkNative_newTradeContext
  (JNIEnv *, jclass, jlong, jobject);

/*
 * Class:     com_longbridge_SdkNative
 * Method:    freeTradeContext
 * Signature: (J)V
 */
JNIEXPORT void JNICALL Java_com_longbridge_SdkNative_freeTradeContext
  (JNIEnv *, jclass, jlong);

/*
 * Class:     com_longbridge_SdkNative
 * Method:    tradeContextSetOnOrderChanged
 * Signature: (JLcom/longbridge/trade/OrderChangedHandler;)V
 */
JNIEXPORT void JNICALL Java_com_longbridge_SdkNative_tradeContextSetOnOrderChanged
  (JNIEnv *, jclass, jlong, jobject);

/*
 * Class:     com_longbridge_SdkNative
 * Method:    tradeContextSubscribe
 * Signature: (J[Lcom/longbridge/trade/TopicType;Lcom/longbridge/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longbridge_SdkNative_tradeContextSubscribe
  (JNIEnv *, jclass, jlong, jobjectArray, jobject);

/*
 * Class:     com_longbridge_SdkNative
 * Method:    tradeContextUnsubscribe
 * Signature: (J[Lcom/longbridge/trade/TopicType;Lcom/longbridge/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longbridge_SdkNative_tradeContextUnsubscribe
  (JNIEnv *, jclass, jlong, jobjectArray, jobject);

/*
 * Class:     com_longbridge_SdkNative
 * Method:    tradeContextHistoryExecutions
 * Signature: (JLcom/longbridge/trade/GetHistoryExecutionsOptions;Lcom/longbridge/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longbridge_SdkNative_tradeContextHistoryExecutions
  (JNIEnv *, jclass, jlong, jobject, jobject);

/*
 * Class:     com_longbridge_SdkNative
 * Method:    tradeContextTodayExecutions
 * Signature: (JLcom/longbridge/trade/GetTodayExecutionsOptions;Lcom/longbridge/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longbridge_SdkNative_tradeContextTodayExecutions
  (JNIEnv *, jclass, jlong, jobject, jobject);

/*
 * Class:     com_longbridge_SdkNative
 * Method:    tradeContextHistoryOrders
 * Signature: (JLcom/longbridge/trade/GetHistoryOrdersOptions;Lcom/longbridge/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longbridge_SdkNative_tradeContextHistoryOrders
  (JNIEnv *, jclass, jlong, jobject, jobject);

/*
 * Class:     com_longbridge_SdkNative
 * Method:    tradeContextTodayOrders
 * Signature: (JLcom/longbridge/trade/GetTodayOrdersOptions;Lcom/longbridge/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longbridge_SdkNative_tradeContextTodayOrders
  (JNIEnv *, jclass, jlong, jobject, jobject);

/*
 * Class:     com_longbridge_SdkNative
 * Method:    tradeContextReplaceOrder
 * Signature: (JLcom/longbridge/trade/ReplaceOrderOptions;Lcom/longbridge/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longbridge_SdkNative_tradeContextReplaceOrder
  (JNIEnv *, jclass, jlong, jobject, jobject);

/*
 * Class:     com_longbridge_SdkNative
 * Method:    tradeContextSubmitOrder
 * Signature: (JLcom/longbridge/trade/SubmitOrderOptions;Lcom/longbridge/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longbridge_SdkNative_tradeContextSubmitOrder
  (JNIEnv *, jclass, jlong, jobject, jobject);

/*
 * Class:     com_longbridge_SdkNative
 * Method:    tradeContextCancelOrder
 * Signature: (JLjava/lang/String;Lcom/longbridge/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longbridge_SdkNative_tradeContextCancelOrder
  (JNIEnv *, jclass, jlong, jstring, jobject);

/*
 * Class:     com_longbridge_SdkNative
 * Method:    tradeContextAccountBalance
 * Signature: (JLcom/longbridge/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longbridge_SdkNative_tradeContextAccountBalance
  (JNIEnv *, jclass, jlong, jobject);

/*
 * Class:     com_longbridge_SdkNative
 * Method:    tradeContextCashFlow
 * Signature: (JLcom/longbridge/trade/GetCashFlowOptions;Lcom/longbridge/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longbridge_SdkNative_tradeContextCashFlow
  (JNIEnv *, jclass, jlong, jobject, jobject);

/*
 * Class:     com_longbridge_SdkNative
 * Method:    tradeContextFundPositions
 * Signature: (JLcom/longbridge/trade/GetFundPositionsOptions;Lcom/longbridge/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longbridge_SdkNative_tradeContextFundPositions
  (JNIEnv *, jclass, jlong, jobject, jobject);

/*
 * Class:     com_longbridge_SdkNative
 * Method:    tradeContextStockPositions
 * Signature: (JLcom/longbridge/trade/GetStockPositionsOptions;Lcom/longbridge/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longbridge_SdkNative_tradeContextStockPositions
  (JNIEnv *, jclass, jlong, jobject, jobject);

/*
 * Class:     com_longbridge_SdkNative
 * Method:    tradeContextMarginRatio
 * Signature: (JLjava/lang/String;Lcom/longbridge/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longbridge_SdkNative_tradeContextMarginRatio
  (JNIEnv *, jclass, jlong, jstring, jobject);

#ifdef __cplusplus
}
#endif
#endif
