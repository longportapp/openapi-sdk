const { Config, QuoteContext, Period } = require("longport");

let config = Config.fromEnv();
let globalCtx;
QuoteContext.new(config).then((ctx) => {
  globalCtx = ctx; // keep a reference to the context, so it doesn't get garbage collected
  ctx.setOnCandlestick((_, event) => console.log(event.toString()));
  ctx.subscribeCandlesticks("AAPL.US", Period.Min_1);
});
