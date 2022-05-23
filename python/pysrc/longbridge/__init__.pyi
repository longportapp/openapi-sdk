from typing import Type


class Config:
    """
    Configuration options for Lonhbridge sdk

    :param app_key: App Key
    :param app_secret: App Secret
    :param access_token: Access Token
    :param http_url: HTTP API url
    :param quote_ws_url: Websocket url for quote API
    :param trade_ws_url: Websocket url for trade API
    """

    def __init__(
        self,
        app_key: str,
        app_secret: str,
        access_token: str,
        http_url="https://openapi.longbridge.global",
        quote_ws_url="https://openapi.longbridge.global",
        trade_ws_url="https://openapi.longbridge.global",
    ) -> None: ...

    @classmethod
    def from_env(cls: Type) -> Config:
        ...


class Market:
    class Unknown(Market):
        ...

    class US(Market):
        ...

    class HK(Market):
        ...

    class CN(Market):
        ...

    class SG(Market):
        ...
