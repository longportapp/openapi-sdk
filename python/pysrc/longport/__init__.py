import sys

from .longport import openapi

sys.modules['longport.openapi'] = openapi


class OpenApiException(Exception):
    def __init__(self, code: int, trace_id: str, message: str):
        self.code = code
        self.trace_id = trace_id
        self.message = message

    def __str__(self):
        if self.code != None:
            return "OpenApiException: (code=%d, trace_id=%s) %s" % (self.code, self.trace_id, self.message)
        else:
            return "OpenApiException: %s" % self.message


openapi.OpenApiException = OpenApiException
