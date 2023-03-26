
enum BidOrAsk {
   Bid,
   Ask, 
}

#[derive(Debug)]
struct Price {
    integral: u64,
    fractional: u64,
    scalar: u64,
}

impl Price {
    fn new(price: f64) -> Price {
        let scalar = 100000;
        let integral = price as u64;
        let fractional = ((price% 1.0) * scalar as f64) as u64;
        Price {
            scalar,integral,fractional
        }
    }
}

struct Limit {
    price: Price,
    order: Vec<Order>,
}

impl Limit {
    fn new(price: 64) -> Limit {
        Limit {
            price: Price::new(price),
            orders: Vec::new(),
        }
    }
}

struct Order {
    size: f64,
    bid_or_ask: BidOrAsk,
}


impl Order {
    fn new(bid_or_ask: BidOrAsk , size: f64) -> Order {
        Order{
            bid_or_ask , size
        }
    }
}

fn main() {
   let price = Price::new(50.5);
   println!("{:?}" , price)
}
