/******************************************************************************
 *
 * Copyright (c) 2017, the Perspective Authors.
 *
 * This file is part of the Perspective library, distributed under the terms of
 * the Apache License 2.0.  The full license can be found in the LICENSE file.
 *
 */
#include <perspective/type_check.h>

namespace perspective {

bool
operator>(const std::size_t& lhs, const t_type_check_result& rhs) {
    return false;
}

t_type_check_result::t_type_check_result()
    : m_type(DTYPE_NONE)
    , m_status(STATUS_INVALID) {}

t_type_check_result::t_type_check_result(int value) {
    std::cout << "constructing new t_typecheck from " << value << std::endl;
    m_type = DTYPE_FLOAT64;
    m_status = STATUS_VALID;
};

void
t_type_check_result::clear() {
    m_type = DTYPE_NONE;
    m_status = STATUS_INVALID;
}

bool
t_type_check_result::is_numeric() const {
    return is_numeric_type(m_type);
}

bool
t_type_check_result::is_valid() const {
    return m_status == STATUS_VALID;
}

t_dtype
t_type_check_result::get_dtype() const {
    return m_type;
}

bool
t_type_check_result::operator==(const t_type_check_result& rhs) const {
    return true;
};

bool
t_type_check_result::operator!=(const t_type_check_result& rhs) const {
    return true;
};

bool
t_type_check_result::operator<(const t_type_check_result& rhs) const {
    return true;
};

bool
t_type_check_result::operator>(const t_type_check_result& rhs) const {
    return true;
};

bool
t_type_check_result::operator>=(const t_type_check_result& rhs) const {
    return true;
};

bool
t_type_check_result::operator<=(const t_type_check_result& rhs) const {
    return true;
};

t_type_check_result
t_type_check_result::operator+() const {
    t_type_check_result rval;
    rval.m_type = m_type;
    rval.m_status = STATUS_INVALID;

    if (is_numeric() && is_valid()) {
        rval.m_status = STATUS_VALID;
    }

    return rval;
};

t_type_check_result
t_type_check_result::operator-() const {
    t_type_check_result rval;
    rval.m_type = m_type;
    rval.m_status = STATUS_INVALID;

    if (is_numeric() && is_valid()) {
        rval.m_status = STATUS_VALID;
    }

    return rval;
};

t_type_check_result
t_type_check_result::operator+(const t_type_check_result& rhs) const {
    t_type_check_result rval;
    rval.m_type = DTYPE_FLOAT64;
    rval.m_status = STATUS_INVALID;
    std::cout << "add " << get_dtype_descr(m_type) << " vs " << get_dtype_descr(rhs.m_type) << std::endl;

    if (is_numeric() && is_valid() && rhs.is_numeric() && rhs.is_valid()) {
        rval.m_status = STATUS_VALID;
    }

    return rval;
};

t_type_check_result
t_type_check_result::operator-(const t_type_check_result& rhs) const {
    t_type_check_result rval;
    rval.m_type = DTYPE_FLOAT64;
    rval.m_status = STATUS_INVALID;

    if (is_numeric() && is_valid() && rhs.is_numeric() && rhs.is_valid()) {
        rval.m_status = STATUS_VALID;
    }

    return rval;
};

t_type_check_result
t_type_check_result::operator*(const t_type_check_result& rhs) const {
    std::cout << "multiply " << *this << " vs " << rhs << std::endl;
    t_type_check_result rval;
    rval.m_type = DTYPE_FLOAT64;
    rval.m_status = STATUS_INVALID;

    if (is_numeric() && is_valid() && rhs.is_numeric() && rhs.is_valid()) {
        rval.m_status = STATUS_VALID;
    }

    return rval;
};

t_type_check_result
t_type_check_result::operator/(const t_type_check_result& rhs) const {
    t_type_check_result rval;
    rval.m_type = DTYPE_FLOAT64;
    rval.m_status = STATUS_INVALID;

    if (is_numeric() && is_valid() && rhs.is_numeric() && rhs.is_valid()) {
        rval.m_status = STATUS_VALID;
    }

    return rval;
};

t_type_check_result
t_type_check_result::operator%(const t_type_check_result& rhs) const {
    t_type_check_result rval;
    rval.m_type = DTYPE_FLOAT64;
    rval.m_status = STATUS_INVALID;

    if (is_numeric() && is_valid() && rhs.is_numeric() && rhs.is_valid()) {
        rval.m_status = STATUS_VALID;
    }

    return rval;
};

template <typename T>
t_type_check_result
t_type_check_result::operator+(T rhs) const {
    t_type_check_result rval;
    rval.m_type = DTYPE_FLOAT64;
    rval.m_status = STATUS_INVALID;

    if (is_numeric() && is_valid() && std::is_arithmetic<T>::value) {
        rval.m_status = STATUS_VALID;
    }

    return rval;
};

template <typename T>
t_type_check_result
t_type_check_result::operator-(T rhs) const {
    t_type_check_result rval;
    rval.m_type = DTYPE_FLOAT64;
    rval.m_status = STATUS_INVALID;

    if (is_numeric() && is_valid() && std::is_arithmetic<T>::value) {
        rval.m_status = STATUS_VALID;
    }

    return rval;
};

template <typename T>
t_type_check_result
t_type_check_result::operator*(T rhs) const {
    t_type_check_result rval;
    rval.m_type = DTYPE_FLOAT64;
    rval.m_status = STATUS_INVALID;

    std::cout << "multiply by T" << std::endl;

    if (is_numeric() && is_valid() && std::is_arithmetic<T>::value) {
        rval.m_status = STATUS_VALID;
    }

    return rval;
};

template <typename T>
t_type_check_result
t_type_check_result::operator/(T rhs) const {
    t_type_check_result rval;
    rval.m_type = DTYPE_FLOAT64;
    rval.m_status = STATUS_INVALID;

    if (is_numeric() && is_valid() && std::is_arithmetic<T>::value) {
        rval.m_status = STATUS_VALID;
    }

    return rval;
};

/**
 * Win32 build complains if the std::uint64_t specialization is unset, and
 * Linux complains if the std::uint64_t is set as it somehow conflicts with
 * the unsigned long specialization below the ifdef.
 */
#ifdef WIN32
template <>
t_type_check_result
t_type_check_result::operator/(std::uint64_t rhs) const {
    t_type_check_result rval;
    rval.m_type = DTYPE_FLOAT64;
    rval.m_status = STATUS_INVALID;

    if (is_numeric() && is_valid()) {
        rval.m_status = STATUS_VALID;
    }

    return rval;
}
#endif

template <>
t_type_check_result
t_type_check_result::operator/(unsigned long rhs) const {
    t_type_check_result rval;
    rval.m_type = DTYPE_FLOAT64;
    rval.m_status = STATUS_INVALID;

    if (is_numeric() && is_valid()) {
        rval.m_status = STATUS_VALID;
    }

    return rval;
}

template <>
t_type_check_result
t_type_check_result::operator/(double rhs) const {
    t_type_check_result rval;
    rval.m_type = DTYPE_FLOAT64;
    rval.m_status = STATUS_INVALID;

    if (is_numeric() && is_valid()) {
        rval.m_status = STATUS_VALID;
    }

    return rval;
}

template <typename T>
t_type_check_result
t_type_check_result::operator%(T rhs) const {
    t_type_check_result rval;
    rval.m_type = DTYPE_FLOAT64;
    rval.m_status = STATUS_INVALID;

    if (is_numeric() && is_valid() && std::is_arithmetic<T>::value) {
        rval.m_status = STATUS_VALID;
    }

    return rval;
};

t_type_check_result&
t_type_check_result::operator+=(const t_type_check_result& rhs) {
    this->m_status = STATUS_INVALID;

    if (is_numeric() && is_valid() && rhs.is_numeric() && rhs.is_valid()) {
        this->m_status = STATUS_VALID;
    }

    return *this;
};

t_type_check_result&
t_type_check_result::operator-=(const t_type_check_result& rhs) {
    this->m_status = STATUS_INVALID;

    if (is_numeric() && is_valid() && rhs.is_numeric() && rhs.is_valid()) {
        this->m_status = STATUS_VALID;
    }

    return *this;
};

t_type_check_result&
t_type_check_result::operator*=(const t_type_check_result& rhs) {
    this->m_status = STATUS_INVALID;

    if (is_numeric() && is_valid() && rhs.is_numeric() && rhs.is_valid()) {
        this->m_status = STATUS_VALID;
    }

    return *this;
};

t_type_check_result&
t_type_check_result::operator/=(const t_type_check_result& rhs) {
    this->m_status = STATUS_INVALID;

    if (is_numeric() && is_valid() && rhs.is_numeric() && rhs.is_valid()) {
        this->m_status = STATUS_VALID;
    }

    return *this;
};

t_type_check_result&
t_type_check_result::operator%=(const t_type_check_result& rhs) {
    this->m_status = STATUS_INVALID;

    if (is_numeric() && is_valid() && rhs.is_numeric() && rhs.is_valid()) {
        this->m_status = STATUS_VALID;
    }

    return *this;
};

} // end namespace perspective

namespace std {

std::ostream& operator<<(
    std::ostream& os, const perspective::t_type_check_result& t) {
    os << perspective::get_dtype_descr(t.m_type) << ":" << perspective::get_status_descr(t.m_status);
    return os;
}

} // end namespace std