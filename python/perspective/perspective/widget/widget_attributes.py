################################################################################
#
# Copyright (c) 2019, the Perspective Authors.
#
# This file is part of the Perspective library, distributed under the terms of
# the Apache License 2.0.  The full license can be found in the LICENSE file.
#
from .widget_traitlets import PerspectiveTraitlets


class PerspectiveWidgetAttributes(PerspectiveTraitlets):
    """For the front-end viewer to serve as the source of truth, all
    attributes that are on the viewer (columns, pivots, etc.) fetch their
    state from the front-end using save(), and setting those attributes
    will call the front-end using restore(). Thus, there should be no
    instance where an attribute on the Python widget != the attribute
    in the Javascript front-end viewer."""

    def save(self):
        raise NotImplementedError

    def restore(self):
        raise NotImplementedError

    @property
    def columns(self):
        return self.save().get("columns")

    @columns.setter
    def columns(self, value):
        self.restore(columns=value)

    @property
    def row_pivots(self):
        return self.save().get("row_pivots")

    @row_pivots.setter
    def row_pivots(self, value):
        self.restore(row_pivots=value)

    @property
    def column_pivots(self):
        return self.save().get("column_pivots")

    @column_pivots.setter
    def column_pivots(self, value):
        self.restore(column_pivots=value)

    @property
    def aggregates(self):
        return self.save().get("aggregates")

    @aggregates.setter
    def aggregates(self, value):
        self.restore(aggregates=value)

    @property
    def filters(self):
        return self.save().get("filters")

    @filters.setter
    def filters(self, value):
        self.restore(filters=value)

    @property
    def sort(self):
        return self.save().get("sort")

    @sort.setter
    def sort(self, value):
        self.restore(sort=value)

    @property
    def expressions(self):
        return self.save().get("expressions")

    @expressions.setter
    def expressions(self, value):
        self.restore(expressions=value)

    @property
    def plugin(self):
        return self.save().get("plugin")

    @plugin.setter
    def plugin(self, value):
        self.restore(plugin=value)

    @property
    def plugin_config(self):
        return self.save().get("plugin_config")

    @plugin_config.setter
    def plugin_config(self, value):
        self.restore(plugin_config=value)

    @property
    def config(self):
        return self.save().get("config")

    @config.setter
    def config(self, value):
        self.restore(config=value)
