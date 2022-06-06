import com.longbridge.*;
import com.longbridge.quote.*;

public class Test {
    public static void main(String[] args) {
        try {
            Config config = Config.fromEnv();
            QuoteContext ctx = QuoteContext.create(config, null).get();
            SecurityQuote[] list = ctx.quote(new String[] { "700.HK", "AAPL.US" }).get();

            for (SecurityQuote info : list) {
                System.out.printf("%s %s %s\n", info.getSymbol(), info.getOpen(), info.getLastDone());
            }
        } catch (Exception e) {
            System.out.println(e);
        }
    }
}
