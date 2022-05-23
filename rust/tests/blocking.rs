// use std::{sync::Arc, time::Duration};

// use longbridge::{blocking::QuoteContextSync, quote::SubFlags,
// Config};

// const APP_KEY: &str = "cfa54f754d89579f3107a7cc9e7402e6";
// const APP_SECRET: &str =
// "2ee2aa73489c33b2e26e28c9d25abc38a9b1595801422b1eec1c89a9a46e8207";
// const ACCESS_TOKEN: &str =
// "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9.
// eyJpc3MiOiJsb25nYnJpZGdlIiwic3ViIjoiYWNjZXNzX3Rva2VuIiwiZXhwIjoxNjgxNDc0NDM3LCJpYXQiOjE2NTExMjk5ODUsImFrIjoiY2ZhNTRmNzU0ZDg5NTc5ZjMxMDdhN2NjOWU3NDAyZTYiLCJhYWlkIjoyMjgsImFjIjoicHNwbF9zZyIsIm1pZCI6MjI4LCJzaWQiOiI3QUovWXNEdFFCVnBjKzFMWkZDQVJnPT0iLCJibCI6LTEsInVsIjowLCJpayI6InBzcGxfc2dfMjI4In0.
// WgC57k8qXmO6oVC4v3LO1k21-he2ag68gACk84pN2D4ieAZJ7M79kf8bEQDIq3VCle-2j5seSHvcdcqUvjiLtYe3ZlEgyKYK6OAHC4DL0AqZ503ITfk8nx6h0c1c_MK6fhhfF6HuAfT3TWLSSUI-3KuL5J91F_7d8vLkrxM0BTCcqZeEUCEj31G7HVXbeKrp-wGZqNBkGDcmwnQUDm4sr8a8F7QwakpB8fLBJEFCoc3u8XqTgTF7CAm3X4Kc5HJ_lhOxTOd4uiRNFjEBJPcCwZ3vHkwsTefQFSP7PggiXSVVUCUSQRkjSa-_e8WKmLezS-or4tlmaI4yldWnh08qgfnTVwkf9Ehzm7wVFjLmIKmJtG_-KkJ0ukGGMGZc1EG4kdjkFgs-2JbCIDFVAK5Ssm5QB0CEuOTjnn2u7mwe6v6cVI_nM325t85SbpuGWNFoA9CTKCQkhEJuTAkHZibV254yMAzEpWSKk0cYly3uVdPpf0rnsLUszSARMn5FaQDylFT3FbkM0lRrnS0QUDqBBvEdbqIuk2B0FCUY_QusonD0KBPGgrAgZxm5CspiXEfgko-7K5dkn5XTJGxTK-JH0ul9clbMf-ipIqEsonxWRTYOleWiIAff3tOKNg5-JUhzVpjhyUVk8-Klufun95BQyZRoHvKGlyB3ez6-Fdk7axA"
// ;

// fn create_config() -> Arc<Config> {
//     Arc::new(
//         Config::new(APP_KEY, APP_SECRET, ACCESS_TOKEN)
//             .http_url("https://openapi.longbridge.xyz")
//             .quote_ws_url("wss://openapi-quote.longbridge.xyz")
//             .trade_ws_url("wss://openapi-trade.longbridge.xyz"),
//     )
// }

// #[cfg(feature = "blocking")]
// #[test]
// fn basic_info() {
//     let ctx = QuoteContextSync::try_new(create_config(), |_| ()).unwrap();
//     let resp = ctx
//         .static_info(["09988.HK", "AAPL.US", "TSLA.US", "NFLX.US"])
//         .unwrap();
//     println!("{:?}", resp);
// }

// #[cfg(feature = "blocking")]
// #[test]
// fn subscribe_quote() {
//     let ctx = QuoteContextSync::try_new(create_config(), |event| {
//         println!("{:?}", event);
//     })
//     .unwrap();
//     ctx.subscribe(["09988.HK", "AAPL.US"], SubFlags::QUOTE, true)
//         .unwrap();
//     std::thread::sleep(Duration::from_secs(15));
// }
