#pragma once

#include <functional>

#include "status.hpp"

namespace longbridge {

template <typename Ctx, typename T>
struct AsyncResult {
 private:
  Ctx ctx_;
  Status status_;
  const T* data_;

 public:
  AsyncResult(Ctx ctx, Status status, const T* data)
      : ctx_(ctx), status_(std::move(status)), data_(data) {}

  operator bool() { return status_.is_ok(); }
  const T* operator->() { return data_; }

  const Ctx& context() { return ctx_; }

  /// Returns `true` if no errors occurs
  bool is_ok() const { return status_.is_ok(); }

  /// Returns `true` if an errors occurs
  bool is_err() const { return status_.is_err(); }
};

template <typename Ctx, typename T>
using AsyncCallback = std::function<void(AsyncResult<Ctx, T>)>;

}  // namespace longbridge