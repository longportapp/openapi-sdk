#pragma once

#include "longbridge.h"

namespace longbridge {
class Status {
 private:
  const lb_error_t *err_;
  bool need_free_;

 public:
  Status() { err_ = nullptr; }
  Status(const lb_error_t *err) {
    err_ = err;
    need_free_ = false;
  }
  Status(lb_error_t *err) {
    err_ = err;
    need_free_ = true;
  }
  Status(Status &&status) {
    err_ = status.err_;
    status.err_ = nullptr;
    status.need_free_ = false;
  }
  ~Status() {
    if (err_ && need_free_) {
      lb_error_free((lb_error_t *)err_);
    }
  }

  /// Returns `true` if no errors occurs
  bool is_ok() const { return err_ == nullptr; }

  /// Returns `true` if an errors occurs
  bool is_err() const { return err_ != nullptr; }

  /// Returns the error code
  int code() const { return lb_error_code(err_); }

  /// Returns the error message
  const char *message() const { return lb_error_message(err_); }
};

}  // namespace longbridge