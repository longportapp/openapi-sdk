#pragma once

#include <utility>

#include "longbridge.h"
#include "status.hpp"

namespace longbridge {

class Config {
 private:
  lb_config_t *config_;

 public:
  Config() { config_ = nullptr; }
  Config(lb_config_t *config) { config_ = config; }
  Config(const Config &) = delete;
  Config(Config &&other) {
    config_ = other.config_;
    other.config_ = nullptr;
  }
  ~Config() {
    if (config_) {
      lb_config_free(config_);
    }
  }

  operator const lb_config_t *() const { return config_; }

  static Status from_env(Config &config) {
    lb_error_t *err = nullptr;
    lb_config_t *config_ptr = lb_config_from_env(&err);
    Status status(err);
    if (status.is_ok()) {
      config.config_ = config_ptr;
    }
    return status;
  }
};

}  // namespace longbridge