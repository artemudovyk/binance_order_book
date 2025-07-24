use rust_decimal::prelude::*;
use std::collections::BTreeMap;

/// Sell
pub struct Ask {
    pub amount: Decimal,
    pub limit_price: Decimal,
}

/// Buy
pub struct Bid {
    pub amount: Decimal,
    pub limit_price: Decimal,
}

pub struct BookOrder {
    pub asks: BTreeMap<Decimal, Decimal>,
    pub bids: BTreeMap<Decimal, Decimal>,
}

impl BookOrder {
    pub fn new() -> Self {
        Self {
            asks: BTreeMap::new(),
            bids: BTreeMap::new(),
        }
    }

    pub fn new_bid(&mut self, bid: Bid) {
        if bid.amount == Decimal::from(0) {
            self.bids.remove(&bid.limit_price);
            return;
        }
        self.bids.insert(bid.limit_price, bid.amount);
    }

    pub fn new_ask(&mut self, ask: Ask) {
        if ask.amount == Decimal::from(0) {
            self.asks.remove(&ask.limit_price);
            return;
        }
        self.asks.insert(ask.limit_price, ask.amount);
    }

    pub fn new_bids(&mut self, bids: Vec<Bid>) {
        for bid in bids {
            self.new_bid(bid);
        }
    }

    pub fn new_asks(&mut self, asks: Vec<Ask>) {
        for ask in asks {
            self.new_ask(ask);
        }
    }
}
