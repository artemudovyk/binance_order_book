use binance_order_book::{Ask, Bid, BookOrder};
use futures_util::StreamExt;
use rust_decimal::Decimal;
use serde::Deserialize;
use tokio_tungstenite::connect_async;

static URL: &str = "wss://stream.binance.com:9443/ws/bnbbtc@depth";

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("Failed to install rustls crypto provider");

    let (mut socket, _) = connect_async(URL).await?;
    println!("WebSocket handshake has been successfully completed");

    let mut book_order = BookOrder::new();

    #[derive(Deserialize, Debug)]
    struct Event {
        a: Vec<Vec<Decimal>>,
        b: Vec<Vec<Decimal>>,
    }

    while let Some(message) = socket.next().await {
        let message = message?;
        println!("message: {message:#}");
        let event: Event = match serde_json::from_str(message.to_text()?) {
            Ok(v) => v,
            Err(_) => {
                println!("Recieved unexpected message via websocket");
                continue;
            }
        };
        println!("json: {event:#?}");
        let asks: Vec<Ask> = event
            .a
            .iter()
            .map(|a| Ask {
                limit_price: a[0],
                amount: a[1],
            })
            .collect();
        let bids: Vec<Bid> = event
            .b
            .iter()
            .map(|b| Bid {
                limit_price: b[0],
                amount: b[1],
            })
            .collect();
        book_order.new_asks(asks);
        book_order.new_bids(bids);

        println!("Asks:");
        for item in book_order.asks.iter().take(5) {
            println!("{} | {}", item.0, item.1)
        }

        println!("Bids:");
        for item in book_order.bids.iter().rev().take(5) {
            println!("{} | {}", item.0, item.1)
        }
    }

    Ok(())
}
