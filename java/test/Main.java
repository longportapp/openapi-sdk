import com.longbridge.*;
import com.longbridge.trade.*;

class Main {
    public static void main(String[] args) throws Exception {
        try (Config config = Config.fromEnv()) {
            try (TradeContext ctx = TradeContext.create(config).get()) {
                Order[] orders = ctx.getTodayOrders(null).join();
                for (Order order : orders) {
                    System.out.println(order);
                }
            }
        }
    }
}