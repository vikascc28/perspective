// ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
// ┃ ██████ ██████ ██████       █      █      █      █      █ █▄  ▀███ █       ┃
// ┃ ▄▄▄▄▄█ █▄▄▄▄▄ ▄▄▄▄▄█  ▀▀▀▀▀█▀▀▀▀▀ █ ▀▀▀▀▀█ ████████▌▐███ ███▄  ▀█ █ ▀▀▀▀▀ ┃
// ┃ █▀▀▀▀▀ █▀▀▀▀▀ █▀██▀▀ ▄▄▄▄▄ █ ▄▄▄▄▄█ ▄▄▄▄▄█ ████████▌▐███ █████▄   █ ▄▄▄▄▄ ┃
// ┃ █      ██████ █  ▀█▄       █ ██████      █      ███▌▐███ ███████▄ █       ┃
// ┣━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┫
// ┃ Copyright (c) 2017, the Perspective Authors.                              ┃
// ┃ ╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌ ┃
// ┃ This file is part of the Perspective library, distributed under the terms ┃
// ┃ of the [Apache License 2.0](https://www.apache.org/licenses/LICENSE-2.0). ┃
// ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

#pragma once
#ifdef PSP_ENABLE_PYTHON

#include <cmath>
#include <chrono>
#include <perspective/base.h>
#include <perspective/binding.h>
#include <perspective/python/base.h>

namespace perspective {
namespace binding {

    /******************************************************************************
     *
     * Helper functions
     */
    template <typename... Args>
    static void
    WARN(Args&&... args) {
        auto loggingModule = PyImport_ImportModule("logging");
        auto criticalCall = PyObject_GetAttrString(loggingModule, "warning");

        // TODO pack into tuple
        PyObject_Call(criticalCall, args...);
        // py::module::import("logging").attr("warning")(args...);
    };

    template <typename... Args>
    static void
    CRITICAL(Args&&... args) {
        auto loggingModule = PyImport_ImportModule("logging");
        auto criticalCall = PyObject_GetAttrString(loggingModule, "critical");
        // TODO pack into tuple
        PyObject_Call(criticalCall, args...);
        // py::module::import("logging").attr("critical")(args...);
    };

    static bool
    IS_BOOL(t_val type_instance) {
        return PyBool_Check(type_instance);
    };
    static bool
    IS_INT(t_val type_instance) {
        return PyLong_Check(type_instance);
    };
    static bool
    IS_FLOAT(t_val type_instance) {
        return PyFloat_Check(type_instance);
    };
    static bool
    IS_STR(t_val type_instance) {
        return PyUnicode_Check(type_instance);
    };
    static bool
    IS_BYTES(t_val type_instance) {
        return PyBytes_Check(type_instance);
    };

    /******************************************************************************
     *
     * Date Parsing
     */

    t_dtype type_string_to_t_dtype(std::string type, std::string name = "");
    t_dtype type_string_to_t_dtype(PyObject* type, PyObject* name = PyUnicode_FromString(""));

    t_val scalar_to_py(const t_tscalar& scalar, bool cast_double = false,
        bool cast_string = false);

} // namespace binding
} // namespace perspective

#endif
