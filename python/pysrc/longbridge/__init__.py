import sys

from .longbridge import openapi

sys.modules['longbridge.openapi'] = openapi


class OpenApiException(Exception):
    def __init__(self, code: int, message: str):
        self.code = code
        self.message = message

    def __str__(self):
        if self.code != None:
            return "OpenApiException: (%d) %s" % (self.code, self.message)
        else:
            return "OpenApiException: %s" % self.message


openapi.OpenApiException = OpenApiException
