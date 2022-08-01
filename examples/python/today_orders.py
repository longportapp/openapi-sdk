from longbridge.openapi import TradeContext, Config

config = Config.from_env()
print(config.refresh_access_token())
