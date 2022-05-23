import sys

from .longbridge import *

sys.modules['longbridge.trade'] = trade
sys.modules['longbridge.quote'] = quote
