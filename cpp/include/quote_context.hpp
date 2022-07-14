#pragma once

#include "async_result.hpp"
#include "callback.hpp"
#include "config.hpp"
#include "push.hpp"
#include "types.hpp"

typedef struct lb_quote_context_t lb_quote_context_t;

namespace longbridge {
namespace quote {

/// Quote context
class QuoteContext
{
private:
  const lb_quote_context_t* ctx_;

public:
  QuoteContext();
  QuoteContext(const lb_quote_context_t* ctx);
  QuoteContext(const QuoteContext& ctx);
  QuoteContext(QuoteContext&& ctx);
  ~QuoteContext();

  QuoteContext& operator=(const QuoteContext& ctx);

  size_t ref_count() const;

  static void create(const Config& config,
                     AsyncCallback<QuoteContext, void> callback);

  /// Subscribe
  void subscribe(const std::vector<std::string>& symbols,
                 SubFlags sub_flags,
                 bool is_first_push,
                 AsyncCallback<QuoteContext, void> callback) const;

  /// Unsubscribe
  void unsubscribe(const std::vector<std::string>& symbols,
                   SubFlags sub_flags,
                   AsyncCallback<QuoteContext, void> callback) const;

  /// Subscribe security candlesticks
  void subscribe_candlesticks(const std::string& symbol,
                              Period period,
                              AsyncCallback<QuoteContext, void> callback) const;

  /// Unsubscribe security candlesticks
  void unsubscribe_candlesticks(
    const std::string& symbol,
    Period period,
    AsyncCallback<QuoteContext, void> callback) const;

  /// Get subscription information
  void subscriptions(
    AsyncCallback<QuoteContext, std::vector<Subscription>> callback) const;

  /// Set quote callback, after receiving the quote data push, it will call back
  /// to this function.
  void set_on_quote(PushCallback<QuoteContext, PushQuote> callback) const;

  /// Set depth callback, after receiving the depth data push, it will call back
  /// to this function.
  void set_on_depth(PushCallback<QuoteContext, PushDepth> callback) const;

  /// Set brokers callback, after receiving the brokers data push, it will call
  /// back to this function.
  void set_on_brokers(PushCallback<QuoteContext, PushBrokers> callback) const;

  /// Set trades callback, after receiving the trades data push, it will call
  /// back to this function.
  void set_on_trades(PushCallback<QuoteContext, PushTrades> callback) const;

  /// Set candlestick callback, after receiving the trades data push, it will
  /// call back to this function.
  void set_on_candlestick(
    PushCallback<QuoteContext, PushCandlestick> callback) const;

  /// Get basic information of securities
  void static_info(const std::vector<std::string>& symbols,
                   AsyncCallback<QuoteContext, std::vector<SecurityStaticInfo>>
                     callback) const;

  /// Get quote of securities
  void quote(
    const std::vector<std::string>& symbols,
    AsyncCallback<QuoteContext, std::vector<SecurityQuote>> callback) const;

  /// Get quote of option securities
  void option_quote(
    const std::vector<std::string>& symbols,
    AsyncCallback<QuoteContext, std::vector<OptionQuote>> callback) const;

  /// Get quote of warrant securities
  void warrant_quote(
    const std::vector<std::string>& symbols,
    AsyncCallback<QuoteContext, std::vector<WarrantQuote>> callback) const;

  /// Get security depth
  void depth(const std::string& symbol,
             AsyncCallback<QuoteContext, SecurityDepth> callback) const;

  /// Get security brokers
  void brokers(const std::string& symbol,
               AsyncCallback<QuoteContext, SecurityBrokers> callback) const;

  /// Get participants
  void participants(
    AsyncCallback<QuoteContext, std::vector<ParticipantInfo>> callback) const;

  /// Get security trades
  void trades(const std::string& symbol,
              uintptr_t count,
              AsyncCallback<QuoteContext, std::vector<Trade>> callback) const;

  /// Get security intraday lines
  void intraday(
    const std::string& symbol,
    AsyncCallback<QuoteContext, std::vector<IntradayLine>> callback) const;

  /// Get security candlesticks
  void candlesticks(
    const std::string& symbol,
    Period period,
    uintptr_t count,
    AdjustType adjust_type,
    AsyncCallback<QuoteContext, std::vector<Candlestick>> callback) const;

  /// Get option chain expiry date list
  void option_chain_expiry_date_list(
    const std::string& symbol,
    AsyncCallback<QuoteContext, std::vector<Date>> callback) const;

  /// Get option chain expiry date list
  void option_chain_info_by_date(
    const std::string& symbol,
    Date expiry_date,
    AsyncCallback<QuoteContext, std::vector<StrikePriceInfo>> callback) const;

  /// Get warrant issuers
  void warrant_issuers(
    AsyncCallback<QuoteContext, std::vector<IssuerInfo>> callback) const;

  /// Get trading session of the day
  void trading_session(
    AsyncCallback<QuoteContext, std::vector<MarketTradingSession>> callback)
    const;

  /// Get market trading days
  ///
  /// The interval must be less than one month, and only the most recent year is
  /// supported.
  void trading_days(Market market,
                    Date begin,
                    Date end,
                    AsyncCallback<QuoteContext, MarketTradingDays> callback);

  /// Get capital flow intraday
  void capital_flow(
    const std::string& symbol,
    AsyncCallback<QuoteContext, std::vector<CapitalFlowLine>> callback) const;

  /// Get capital distribution
  void capital_distribution(
    const std::string& symbol,
    AsyncCallback<QuoteContext, CapitalDistributionResponse> callback) const;

  /// Get real-time quotes
  ///
  /// Get real-time quotes of the subscribed symbols, it always returns the data
  /// in the local storage.
  void realtime_quote(
    const std::vector<std::string>& symbols,
    AsyncCallback<QuoteContext, std::vector<RealtimeQuote>> callback) const;

  /// Get real-time depth
  ///
  /// Get real-time depth of the subscribed symbols, it always returns the data
  /// in the local storage.
  void realtime_depth(
    const std::string& symbol,
    AsyncCallback<QuoteContext, SecurityDepth> callback) const;

  /// Get real-time trades
  ///
  /// Get real-time trades of the subscribed symbols, it always returns the data
  /// in the local storage.
  void realtime_trades(
    const std::string& symbol,
    uint64_t count,
    AsyncCallback<QuoteContext, std::vector<Trade>> callback) const;

  /// Get real-time broker queue
  ///
  /// Get real-time broker queue of the subscribed symbols, it always returns
  /// the data in the local storage.
  void realtime_brokers(
    const std::string& symbol,
    AsyncCallback<QuoteContext, SecurityBrokers> callback) const;

  /// Get real-time candlesticks
  ///
  /// Get real-time candlesticks of the subscribed symbols, it always returns
  /// the data in the local storage.
  void realtime_candlesticks(
    const std::string& symbol,
    Period period,
    uintptr_t count,
    AsyncCallback<QuoteContext, std::vector<Candlestick>> callback) const;
};

} // namespace quote
} // namespace longbridge