use exchange::websockets::*;
use std::sync::atomic::AtomicBool;

fn main() {
    //user_stream();
    //user_stream_websocket();
    //market_websocket();
    //kline_websocket();
    //all_trades_websocket();
    //last_price_for_one_symbol();
    multiple_streams();
}

fn multiple_streams() {
    let symbols: Vec<_> = vec!["ethbtc", "bnbeth"]
        .into_iter()
        .map(String::from)
        .collect();
    let mut endpoints: Vec<String> = Vec::new();

    for symbol in symbols.iter() {
        endpoints.push(format!("{}@depth@100ms", symbol.to_lowercase()));
    }

    let keep_running = AtomicBool::new(true);
    let mut web_socket: WebSockets<'_> = WebSockets::new(|event: WebsocketEvent| {
        if let WebsocketEvent::DepthOrderBook(depth_order_book) = event {
            println!("{:?}", depth_order_book);
        }

        Ok(())
    });

    web_socket.connect_multiple_streams(&endpoints).unwrap(); // check error
    if let Err(e) = web_socket.event_loop(&keep_running) {
        println!("Error: {}", e);
    }
    web_socket.disconnect().unwrap();
}
