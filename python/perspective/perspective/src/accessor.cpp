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

#ifdef PSP_ENABLE_PYTHON
#include <perspective/base.h>
#include <perspective/binding.h>
#include <perspective/python/accessor.h>
#include <perspective/python/base.h>
#include <perspective/python/utils.h>

namespace perspective {
namespace binding {

    /******************************************************************************
     *
     * Data accessor API
     */

    std::vector<std::string>
    get_column_names(t_val data, std::int32_t format) {
        std::vector<std::string> names;
        if (format == 0) {
            // record
            auto data_list_size = PyList_Size(data);
            std::int32_t max_check = 50;

            if (data_list_size > 0) {
                // get first item in list, a dictionary
                auto data_list_first_dict = PyList_GetItem(data, 0);

                // get list of keys
                auto item_keys = PyDict_Keys(data_list_first_dict);
                auto item_size_keys = PyList_Size(item_keys);

                for (auto int i = 0; i < item_size_keys; ++i) {
                    // get key
                    auto name_object = PyList_GetItem(i);
                    std::string name = PyUnicode_AsUTF8(name_object);
                    names.push_back(name);
                }
            }

            std::int32_t check_index
                = std::min(max_check, int32_t(data_list_size));

            for (auto ix = 1; ix < check_index; ix++) {
                // get next item
                auto data_list_first_dict = PyList_GetItem(data, ix);

                // get list of keys
                auto item_keys = PyDict_Keys(data_list_first_dict);
                auto item_size_keys = PyList_Size(item_keys);

                auto old_size = names.size();

                for (auto int i = 0; i < item_size_keys; ++i) {
                    // get key
                    auto name_object = PyList_GetItem(i);
                    std::string name = PyUnicode_AsUTF8(name_object);
                    names.push_back(name);
                    if (std::find(names.begin(), names.end(), name) == names.end()) {
                        names.push_back(name);
                }

                if (old_size != names.size()) {
                    if (max_check == 50) {
                        WARN("Data parse warning: Array data has inconsistent "
                             "rows");
                    }
                    WARN("Extended from %d to %d", old_size, names.size());
                    max_check *= 2;
                }
            }
        } else if (format == 1 || format == 2) {
            auto dict_keys = PyDict_Keys(data_list_first_dict);
            auto dict_size_keys = PyList_Size(item_keys);

            for (auto i = 0; i < dict_size_keys; ++i) {
                auto name_object = PyList_GetItem(i);
                std::string name = PyUnicode_AsUTF8(name_object);
                names.push_back(name);
            }
        }
        return names;
    }

    t_dtype
    infer_type(t_val x, t_val date_validator) {
        auto type_name_obj = PyType_GetName(x);
        std::string type_string = PyUnicode_AsUTF8(type_name_obj);
        // std::string type_string
        //     = x.get_type().attr("__name__").cast<std::string>();
        t_dtype t = t_dtype::DTYPE_STR;

        // If object provides its own type, use that
        if (PyObject_HasAttrString(x, "_psp_dtype_")) {
            auto new_type = PyObject_GetAttrString(x, "_psp_dtype_");
            // auto new_type = x.attr("_psp_dtype_")();

            if (PyObject_HasAttrString(new_type, "__name__")) {
                // If type is a class, get its name
                type_name_obj = PyObject_GetAttrString(new_type, "__name__");
                type_string = PyUnicode_AsUTF8(type_name_obj);
                // type_string = new_type.attr("__name__").cast<std::string>();

            } else {
                // Assume that the string is the type
                type_string = PyUnicode_AsUTF8(new_type);
                // type_string = new_type.cast<std::string>();
            }

            // Extract representation if not storing as object
            if (type_string != "object") {
                if (PyObject_HasAttrString(x, "_psp_repr_")) {
                    x = PyObject_GetAttrString(x, "_psp_repr_");
                    // x = x.attr("_psp_repr_")();
                } else {
                    x = PyObject_Str(x);
                    // x = x.cast<py::str>();
                }
            }
        }

        if (x == Py_None) {
            t = t_dtype::DTYPE_NONE;
        } else if (py::isinstance<py::bool_>(x) || type_string == "bool") {
            // booleans are both instances of bool_ and int_ -  check for bool
            // first
            t = t_dtype::DTYPE_BOOL;
        } else if (type_string == "long") {
            t = t_dtype::DTYPE_INT64;
        } else if (py::isinstance<py::float_>(x) || type_string == "float") {
            t = t_dtype::DTYPE_FLOAT64;
        } else if (py::isinstance<py::int_>(x) || type_string == "int") {
            if (PY_MAJOR_VERSION < 3) {
                t = t_dtype::DTYPE_INT32;
            } else {
                t = t_dtype::DTYPE_INT64;
            }
        } else if (py::isinstance<py::str>(x) || type_string == "str") {
            t_dtype parsed_type
                = date_validator.attr("format")(x).cast<t_dtype>();
            if (parsed_type == t_dtype::DTYPE_DATE
                || parsed_type == t_dtype::DTYPE_TIME) {
                t = parsed_type;
            } else {
                std::string lower = x.attr("lower")().cast<std::string>();
                if (lower == "true" || lower == "false") {
                    t = t_dtype::DTYPE_BOOL;
                } else {
                    t = t_dtype::DTYPE_STR;
                }
            }
        } else {
            t = type_string_to_t_dtype(type_string);
        }

        return t;
    }

    t_dtype
    get_data_type(
        t_val data, std::int32_t format, py::str name, t_val date_validator) {
        std::int32_t i = 0;
        boost::optional<t_dtype> inferredType;

        if (format == 0) {
            py::list data_list = data.cast<py::list>();

            // loop parameters differ slightly so rewrite the loop
            while (!inferredType.is_initialized() && i < 100
                && i < data_list.size()) {
                if (data_list != Py_None) {
                    if (!data_list[i].cast<py::dict>()[name].is_none()) {
                        inferredType = infer_type(
                            data_list[i].cast<py::dict>()[name].cast<t_val>(),
                            date_validator);
                    }
                }
                i++;
            }
        } else if (format == 1) {
            py::dict data_dict = data.cast<py::dict>();

            while (!inferredType.is_initialized() && i < 100
                && i < data_dict[name].cast<py::list>().size()) {
                if (!data_dict[name].cast<py::list>()[i].is_none()) {
                    inferredType = infer_type(
                        data_dict[name].cast<py::list>()[i].cast<t_val>(),
                        date_validator);
                }
                i++;
            }
        }

        if (!inferredType.is_initialized()) {
            return t_dtype::DTYPE_STR;
        } else {
            return inferredType.get();
        }
    }

    std::vector<t_dtype>
    get_data_types(t_val data, std::int32_t format,
        std::vector<std::string> names, t_val date_validator) {
        std::vector<t_dtype> types;

        if (names.size() == 0) {
            WARN("Cannot determine data types without column names!");
            return types;
        }

        if (format == 2) {
            py::dict data_dict = data.cast<py::dict>();

            for (auto tup : data_dict) {
                auto name = tup.first.cast<std::string>();
                auto data_type = tup.second.get_type()
                                     .attr("__name__")
                                     .cast<std::string>();
                std::string value;

                if (data_type == "type") {
                    value = py::str(
                        tup.second.cast<py::object>().attr("__name__"))
                                .cast<std::string>();
                } else {
                    value = tup.second.cast<std::string>();
                }

                t_dtype type;

                if (name == "__INDEX__") {
                    WARN("Warning: __INDEX__ column should not be in the Table "
                         "schema.");
                    continue;
                }
                type = type_string_to_t_dtype(value, name);
                types.push_back(type);
            }

            return types;
        } else {
            types.resize(names.size());
            for (auto i = 0; i < names.size(); ++i) {
                t_dtype type = get_data_type(
                    data, format, py::str(names[i]), date_validator);
                types[i] = type;
            }
        }

        return types;
    }

} // namespace binding
} // namespace perspective

#endif