################################################################################
#
# Copyright (c) 2019, the Perspective Authors.
#
# This file is part of the Perspective library, distributed under the terms of
# the Apache License 2.0.  The full license can be found in the LICENSE file.
#
import asyncio
import logging

from random import random

from ..client.client import PerspectiveClient


class PerspectiveWidgetAsyncClient(PerspectiveClient):
    def __init__(self, widget_send):
        """ipywidgets.Widget have a send() method on the instance, which we
        use here to pass messages to the front-end."""
        super(PerspectiveWidgetAsyncClient, self).__init__()
        self.widget_send = widget_send
        self.name = "viewer_{}".format(str(random()))

    def send(self, msg, binary=False):
        if binary:
            self.widget_send(None, buffers=[msg])
        else:
            # The Widget comm will automatically serialize and deserialize, so
            # just send the object instead of calling json.dumps() on it.
            self.widget_send(msg)

    def terminate(self):
        pass

    def open_table(self):
        raise NotImplementedError(
            "Cannot `open_table` on PerspectiveWidget async client!"
        )

    def table(self):
        raise NotImplementedError("Cannot `table` on PerspectiveWidget async client!")

    # Call methods on the viewer using the async client, which allows us to
    # await the response or error from PerspectiveViewer in the widget front-end.
    def viewer_save(self):
        return self._async_queue("save", "viewer_method")

    def async_queue_widget(self, msg_method, *args, **kwargs):
        """Create a new Future, send a message to the widget front-end,
        and store the Future to be resolved when _handle is called.
        """
        arguments = list(args)

        if len(kwargs) > 0:
            arguments.append(kwargs)

        # FIXME: new message schema lol
        msg = {"type": "viewer_method", "method": msg_method, "args": arguments}

        try:
            loop = asyncio.get_running_loop()
        except RuntimeError:
            logging.error("Could not find running asyncio event loop!")
            return

        future = loop.create_future()
        self.post(msg, future)
        return future


class PerspectiveWidgetMessage(object):
    """A custom message that will be passed from the Python widget to the
    front-end.

    When creating new messages, use this class as it defines a concrete schema
    for the message and prevents loosely creating `dict` objects everywhere.
    Use `to_dict()` to obtain the message in a form that can be sent through
    IPyWidgets.
    """

    def __init__(self, msg_id, msg_type, msg_data):
        """Create a new PerspectiveWidgetMessage."""
        self.id = msg_id
        self.type = msg_type
        self.data = msg_data

    def to_dict(self):
        """Returns a dictionary representation of the message."""
        return {"id": self.id, "type": self.type, "data": self.data}
