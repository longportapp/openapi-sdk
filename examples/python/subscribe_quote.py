from time import sleep
from longbridge.openapi import QuoteContext, Config, SubType


class EventHandler:
    def on_event(self, symbol: str, msg):
        print(symbol, msg)


config = Config.from_env()
ctx = QuoteContext(config, EventHandler())

symbols = ["700.HK", "AAPL.US", "TSLA.US",
           "NFLX.US"]
ctx.subscribe(symbols, [SubType.Quote], True)
sleep(30)
