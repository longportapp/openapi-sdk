#pragma once

#include "async_result.hpp"
#include "config.hpp"
#include "longbridge.h"

namespace longbridge {

class QuoteContext {
 private:
  const lb_quote_context_t *ctx_;

 public:
  QuoteContext(const lb_quote_context_t *ctx) {
    ctx_ = ctx;
    lb_quote_context_retain(ctx_);
  }

  QuoteContext(const QuoteContext &ctx) {
    ctx_ = ctx.ctx_;
    lb_quote_context_retain(ctx_);
  }

  ~QuoteContext() { lb_quote_context_release(ctx_); }

  static void create(Config &config,
                     AsyncCallback<QuoteContext, void> callback) {
    lb_quote_context_new(
        config,
        [](auto res) {
          auto callback_ptr =
              (AsyncCallback<QuoteContext, void> *)res->userdata;
          (*callback_ptr)(AsyncResult<QuoteContext, void>(
              QuoteContext((const lb_quote_context_t *)res->ctx),
              Status(res->error), nullptr));
          delete callback_ptr;
        },
        new AsyncCallback<QuoteContext, void>(callback));
  }
};

}  // namespace longbridge