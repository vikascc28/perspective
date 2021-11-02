/******************************************************************************
 *
 * Copyright (c) 2017, the Perspective Authors.
 *
 * This file is part of the Perspective library, distributed under the terms of
 * the Apache License 2.0.  The full license can be found in the LICENSE file.
 *
 */

#pragma once
#include <perspective/first.h>
#include <perspective/base.h>
#include <perspective/raw_types.h>
#include <perspective/exports.h>
#include <perspective/date.h>
#include <perspective/time.h>
#include <perspective/none.h>
#include <type_traits>

namespace perspective {

struct t_type_check_result {
    t_type_check_result() = default;

    t_type_check_result(int value);

    void clear();

    bool is_numeric() const;
    bool is_valid() const;
    t_dtype get_dtype() const;

    bool operator==(const t_type_check_result& rhs) const;
    bool operator!=(const t_type_check_result& rhs) const;
    bool operator<(const t_type_check_result& rhs) const;
    bool operator>(const t_type_check_result& rhs) const;
    bool operator>=(const t_type_check_result& rhs) const;
    bool operator<=(const t_type_check_result& rhs) const;

    t_type_check_result operator+() const;
    t_type_check_result operator-() const;

    t_type_check_result operator+(const t_type_check_result& rhs) const;
    t_type_check_result operator-(const t_type_check_result& rhs) const;
    t_type_check_result operator*(const t_type_check_result& rhs) const;
    t_type_check_result operator/(const t_type_check_result& rhs) const;
    t_type_check_result operator%(const t_type_check_result& rhs) const;

    template <typename T>
    t_type_check_result operator+(T rhs) const;
    template <typename T>
    t_type_check_result operator-(T rhs) const;
    template <typename T>
    t_type_check_result operator*(T rhs) const;
    template <typename T>
    t_type_check_result operator/(T rhs) const;
    template <typename T>
    t_type_check_result operator%(T rhs) const;

    t_type_check_result& operator+=(const t_type_check_result& rhs);
    t_type_check_result& operator-=(const t_type_check_result& rhs);
    t_type_check_result& operator*=(const t_type_check_result& rhs);
    t_type_check_result& operator/=(const t_type_check_result& rhs);
    t_type_check_result& operator%=(const t_type_check_result& rhs);

    t_dtype m_type;
    t_status m_status;
};

bool operator>(const std::size_t& lhs, const t_type_check_result& rhs);

} // end namespace perspective