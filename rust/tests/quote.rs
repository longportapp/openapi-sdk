// use std::sync::Arc;

// use longbridge::{
//     quote::{self, SubFlags},
//     Config,
// };

// // const APP_KEY: &str = "3130a9409b7c384aa5d2b20a8875192e";
// // const APP_SECRET: &str =
// // "77a6577bde0ef1a0f1d6a203aec976bb6bbab085b3506ca0f6419e77624ab659";
// // const ACCESS_TOKEN: &str =
// // "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9.
// // eyJpc3MiOiJsb25nYnJpZGdlIiwic3ViIjoiYWNjZXNzX3Rva2VuIiwiZXhwIjoxNjgxNDc0NDM3LCJpYXQiOjE2NDk5NDQyNDYsImFrIjoiMzEzMGE5NDA5YjdjMzg0YWE1ZDJiMjBhODg3NTE5MmUiLCJhYWlkIjoyMDAwMDAwMSwiYWMiOiJsYiIsIm1pZCI6MSwic2lkIjoiUkFPNE5odkJZRlJXc0NRdUloK05YUT09IiwiYmwiOi0xLCJ1bCI6MCwiaWsiOiJsYl8yMDAwMDAwMSJ9.
// // V17vL_JzmYrV9MiwYuG0eSOtv3y99GP_FRyWUuz-v3aJM4wM23QoqnBlBhMpx06O6bh79pGPmbGSy3UMsGhfSn_u0fJy8RLFL3csRsat-Hd6l5uq2BRouCNec2NRn1aeJDTElKuFQT-Vb_SAT7z-bSb9vn6PgLjw1dqYbX4piaAWXZiVM4NNfP5eipfc3JlzAMkKrmLZeO_h-Dxy7sAFjS7cTAUR6W7bBgo-GT5XBPv6C2WVVdSOPUevNBRk8c6h-vQssckop3GBg4V-DUUIe0pASZNnDLgRghj-pLLGEH2-yVJPhKXV9IYjmZLFXmDo29qyLxYkxYlDhZFafjehk7vhZfV85srOm6I_5y1Evmze8sZ1qvxj5A9zZuDRpNdMop3oYS_xiY3ThAie4VGbhkubp2uJv_4N-CCwPAR3mjEoVQDaWUyPbzaquPxWgoDTEdvsB-r_vA3yrRfVIeMC2mPoUbU_dAn55u9K8BkTOFAO3uzNeaUVq-uVU4e8VXXtKQ8jU8L9Uib-ZemPXB6oJ6EIGqeyUn8Zy0ATxKgFLym_DRM-9bdf_oR-6fzYi0be3s8PlVwM9dld36oW323ScIUauxRD5-U2YbLhSBOqCKq1eL2nHrU_6mDn_BPvmt2AViK7_A_PGOC7cW2JHCPGm8WtczF9tvD9_ESIIyg7sj0"
// // ;

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

// #[tokio::test]
// async fn get_basic_info() {
//     let config = create_config();
//     let (ctx, _) = quote::QuoteContext::try_new(config).await.unwrap();
//     let resp = ctx
//         .static_info(["09988.HK", "AAPL.US", "TSLA.US", "NFLX.US"])
//         .await
//         .unwrap();
//     println!("{:?}", resp);
// }

// #[tokio::test]
// async fn get_quote() {
//     let config = create_config();
//     let (ctx, _) = quote::QuoteContext::try_new(config).await.unwrap();
//     let resp = ctx
//         .quote(["700.HK", "09988.HK", "AAPL.US", "TSLA.US", "NFLX.US"])
//         .await
//         .unwrap();
//     println!("{:?}", resp);
// }

// #[tokio::test]
// async fn get_warrant_quote() {
//     let config = create_config();
//     let (ctx, _) = quote::QuoteContext::try_new(config).await.unwrap();
//     let resp = ctx.warrant_quote(["11131.HK", "14983.HK"]).await.unwrap();
//     println!("{:?}", resp);
// }

// #[tokio::test]
// async fn intraday() {
//     let config = create_config();
//     let (ctx, _) = quote::QuoteContext::try_new(config).await.unwrap();
//     let resp = ctx.intraday("700.HK").await.unwrap();
//     println!("{:?}", resp);
// }

// #[tokio::test]
// async fn subscribe_quote() {
//     let config = create_config();
//     let (ctx, mut events) =
// quote::QuoteContext::try_new(config).await.unwrap();     ctx.subscribe(["700.
// HK"], SubFlags::QUOTE, true)         .await
//         .unwrap();

//     while let Some(event) = events.recv().await {
//         println!("{:?}", event);
//     }
// }

// #[tokio::test]
// async fn subscribe_depth() {
//     let config = create_config();
//     let (ctx, mut events) =
// quote::QuoteContext::try_new(config).await.unwrap();     ctx.subscribe(["700.
// HK"], SubFlags::DEPTH, true)         .await
//         .unwrap();

//     while let Some(event) = events.recv().await {
//         println!("{:?}", event);
//     }
// }

// #[tokio::test]
// async fn subscribe_brokers() {
//     let config = create_config();
//     let (ctx, mut events) =
// quote::QuoteContext::try_new(config).await.unwrap();     ctx.subscribe(["700.
// HK"], SubFlags::BROKER, true)         .await
//         .unwrap();

//     while let Some(event) = events.recv().await {
//         println!("{:?}", event);
//     }
// }

// #[tokio::test]
// async fn subscribe_trades() {
//     let config = create_config();
//     let (ctx, mut events) =
// quote::QuoteContext::try_new(config).await.unwrap();     ctx.subscribe(["700.
// HK"], SubFlags::TRADE, true)         .await
//         .unwrap();

//     while let Some(event) = events.recv().await {
//         println!("{:?}", event);
//     }
// }
