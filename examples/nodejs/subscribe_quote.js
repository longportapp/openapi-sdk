const { Config, QuoteContext, SubType } = require("longbridge");

let config = Config.fromEnv();
let ctx = new QuoteContext(config, (_, event) => console.log(event.toString()));

ctx
  .open()
  .then(() =>
    ctx.subscribe(
      ["700.HK", "AAPL.US", "TSLA.US", "NFLX.US"],
      [SubType.Quote],
      true
    )
  );
