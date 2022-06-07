from longbridge.openapi import TradeContext, Config

config = Config.from_env()
ctx = TradeContext(config)

resp = ctx.today_orders()
for obj in resp:
    print(resp)
