#pragma once

#include <string>
#include <utility>

#include "longbridge.h"

namespace longbridge {

class Decimal {
 private:
  lb_decimal_t *value_;

 public:
  Decimal(const lb_decimal_t *other) { value_ = lb_decimal_clone(other); }
  Decimal(const Decimal &other) { value_ = lb_decimal_clone(other); }
  Decimal(const char *str) { value_ = lb_decimal_from_str(str); }
  Decimal(const std::string &str) { value_ = lb_decimal_from_str(str.c_str()); }
  Decimal(double other) { value_ = lb_decimal_from_double(other); }
  ~Decimal() { lb_decimal_free(value_); }

  Decimal clone() const { return Decimal(value_); }

  operator const lb_decimal_t *() const { return value_; }
  operator lb_decimal_t *() { return value_; }

  Decimal operator+(const Decimal &other) const {
    auto new_value = Decimal(value_);
    lb_decimal_add(new_value, other);
    return new_value;
  }

  Decimal &operator+=(const Decimal &other) {
    lb_decimal_add(value_, other);
    return *this;
  }

  Decimal operator-(const Decimal &other) const {
    auto new_value = Decimal(value_);
    lb_decimal_sub(new_value, other);
    return new_value;
  }

  Decimal &operator-=(const Decimal &other) {
    lb_decimal_sub(value_, other);
    return *this;
  }

  Decimal operator*(const Decimal &other) const {
    auto new_value = Decimal(value_);
    lb_decimal_mul(new_value, other);
    return new_value;
  }

  Decimal &operator*=(const Decimal &other) {
    lb_decimal_sub(value_, other);
    return *this;
  }

  Decimal operator/(const Decimal &other) const {
    auto new_value = Decimal(value_);
    lb_decimal_div(new_value, other);
    return new_value;
  }

  Decimal &operator/=(const Decimal &other) {
    lb_decimal_div(value_, other);
    return *this;
  }

  Decimal operator%(const Decimal &other) const {
    auto new_value = Decimal(value_);
    lb_decimal_rem(new_value, other);
    return new_value;
  }

  Decimal &operator%=(const Decimal &other) {
    lb_decimal_div(value_, other);
    return *this;
  }

  Decimal operator-() const {
    auto new_value = Decimal(value_);
    lb_decimal_neg(new_value);
    return new_value;
  }

  bool operator>(const Decimal &other) const {
    return lb_decimal_gt(value_, other);
  }

  bool operator>=(const Decimal &other) const {
    return lb_decimal_gte(value_, other);
  }

  bool operator==(const Decimal &other) const {
    return lb_decimal_eq(value_, other);
  }

  bool operator<(const Decimal &other) const {
    return lb_decimal_lt(value_, other);
  }

  bool operator<=(const Decimal &other) const {
    return lb_decimal_lte(value_, other);
  }

  double to_double() const { return lb_decimal_to_double(value_); }

  /// Computes the absolute value.
  void abs() { lb_decimal_abs(value_); }

  /// Returns the smallest integer greater than or equal to a number.
  void ceil() { lb_decimal_ceil(value_); }

  /// Returns the largest integer less than or equal to a number.
  void floor() { lb_decimal_floor(value_); }

  /// Returns a new Decimal representing the fractional portion of the number.
  void fract() { lb_decimal_fract(value_); }

  /// Returns `true` if the decimal is negative.
  bool is_negative() const { return lb_decimal_is_negative(value_); }

  /// Returns `true` if the decimal is positive.
  bool is_positive() const { return lb_decimal_is_positive(value_); }

  /// Returns `true` if this Decimal number is equivalent to zero.
  bool is_zero() const { return lb_decimal_is_zero(value_); }

  /// Returns the maximum of the two numbers.
  Decimal max(const Decimal &other) const {
    auto max_value = lb_decimal_max(value_, other);
    return Decimal(lb_decimal_clone(max_value));
  }

  /// Returns the minimum of the two numbers.
  Decimal min(const Decimal &other) const {
    auto max_value = lb_decimal_min(value_, other);
    return Decimal(lb_decimal_clone(max_value));
  }

  /// Strips any trailing zero’s from a Decimal and converts `-0` to `0`.
  void normalize() { lb_decimal_normalize(value_); }

  /// Returns a new Decimal number with no fractional portion (i.e. an integer).
  /// Rounding currently follows “Bankers Rounding” rules. e.g. `6.5` -> `6`,
  /// `7.5` -> `8`
  void round() { lb_decimal_round(value_); }

  /// Returns a new Decimal integral with no fractional portion. This is a true
  /// truncation whereby no rounding is performed.
  void trunc() { lb_decimal_trunc(value_); }

  /// Computes the sine of a number (in radians)
  void sin() { lb_decimal_sin(value_); }

  /// Computes the cosine of a number (in radians)
  void cos() { lb_decimal_cos(value_); }

  /// Computes the tangent of a number (in radians). Panics upon overflow or
  /// upon approaching a limit.
  void tan() { lb_decimal_tan(value_); }

  /// The square root of a Decimal. Uses a standard Babylonian method.
  void sqrt() { lb_decimal_sqrt(value_); }

  /// Raise self to the given Decimal exponent: x<sup>y</sup>. If `exp` is not
  /// whole then the approximation e<sup>y*ln(x)</sup> is used.
  void pow(const Decimal &exp) { lb_decimal_pow(value_, exp); }

  /// Calculates the natural logarithm for a Decimal calculated using Taylor’s
  /// series.
  void ln() { lb_decimal_ln(value_); }

  /// Calculates the base 10 logarithm of a specified Decimal number.
  void log10() { lb_decimal_log10(value_); }

  /// The estimated exponential function, ex. Stops calculating when it is
  /// within tolerance of roughly `0.0000002`.
  void exp() { lb_decimal_exp(value_); }

  /// The estimated exponential function, e<sup>x</sup> using the `tolerance`
  /// provided as a hint as to when to stop calculating. A larger
  /// tolerance will cause the number to stop calculating sooner at the
  /// potential cost of a slightly less accurate result.
  void exp_with_tolerance(const Decimal &tolerance) {
    lb_decimal_exp_with_tolerance(value_, tolerance);
  }

  /// Abramowitz Approximation of Error Function from
  /// [wikipedia](https://en.wikipedia.org/wiki/Error_function#Numerical_approximations)
  void erf() { lb_decimal_erf(value_); }

  /// The Probability density function for a Normal distribution.
  void norm_pdf() { lb_decimal_norm_pdf(value_); }
};

}  // namespace longbridge