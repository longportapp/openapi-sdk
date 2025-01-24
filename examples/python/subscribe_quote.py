from time import sleep
from longport.openapi import QuoteContext, Config, SubType, PushQuote, Period, AdjustType


def on_quote(symbol: str, event: PushQuote):
    print(symbol, event)
    sleep(1)


config = Config.from_env()
ctx = QuoteContext(config)
ctx.set_on_quote(on_quote)
ctx.subscribe(["TSLA.US"], [SubType.Quote], is_first_push=True)
sleep(10000)
