#[derive(Debug)]
enum BidOrAsk{
    Bid,
    Ask,
}

#[derive(Debug)]
struct Price{
    integral: u64,
    fractional: u64,
    scalar: u64,
}

impl Price{
    fn new(price: f64) -> Price{
        let scalar = 100000;
        let integral = price as u64;
        let fractional = ((price % 1.0) * scalar as f64) as u64;
        Price{
            integral,
            fractional,
            scalar,
        }
    }
}

#[derive(Debug)]
struct Limit{
    price: Price,
    orders: Vec<Order>,
}

impl Limit{
    fn new(price: f64) -> Limit{
        Limit{
            price: Price::new(price),
            orders: Vec::new(),
        }
    }

    fn add_order(&mut self, order: Order){
        self.orders.push(order);
    }
}

#[derive(Debug)]
struct Order{
    size: f64,
    bid_or_ask: BidOrAsk,
}

impl Order{
    fn new(size: f64, bid_or_ask: BidOrAsk) -> Order{
        Order{
            size,
            bid_or_ask,
        }
    }
}

fn main() {
    let mut limit = Limit::new(65.0);
    let buy_order = Order::new(5.5, BidOrAsk::Bid);
    limit.add_order(buy_order);
    println!("{:?}", limit)
}