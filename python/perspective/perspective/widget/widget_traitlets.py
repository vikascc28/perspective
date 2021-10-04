################################################################################
#
# Copyright (c) 2019, the Perspective Authors.
#
# This file is part of the Perspective library, distributed under the terms of
# the Apache License 2.0.  The full license can be found in the LICENSE file.
#

from traitlets import HasTraits, Bool


class PerspectiveTraitlets(HasTraits):
    """Define the traitlet interface with `PerspectiveJupyterWidget` on the
    front end, which does not include viewer configuration such as columns
    and pivots. Instead, the only traitlets are for options that are set
    via attribute, such as dark mode and "editable".
    """

    # `perspective-viewer` options
    dark = Bool(None, allow_none=True).tag(sync=True)
    editable = Bool(False).tag(sync=True)
    server = Bool(False).tag(sync=True)
    client = Bool(False).tag(sync=True)
