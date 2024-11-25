const { Config, QuoteContext, SubType } = require("longport");

let config = Config.fromEnv();
let globalCtx;
QuoteContext.new(config).then((ctx) => {
  globalCtx = ctx; // keep a reference to the context, so it doesn't get garbage collected
  ctx.setOnQuote((_, event) => console.log(event.toString()));
  ctx.subscribe(["TSLA.US"], [SubType.Quote], true);
});
