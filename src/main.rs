use std::net::SocketAddr;

use crate::order::Order;
use crate::receipt_printer::ReceiptPrinter;
use axum::http::{Method, StatusCode};
use axum::routing::post;
use axum::{Json, Router};
use tower_http::cors::{Any, CorsLayer};

mod order;
mod receipt_printer;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/order", post(print_order)).layer(
        CorsLayer::new()
            .allow_origin(Any) // Allow requests from any origin
            .allow_methods(Any) // Allow any HTTP method
            .allow_headers(Any), // Allow any headers
    );

    let addr = SocketAddr::from(([127, 0, 0, 1], 8081));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("Listening on {}", addr);

    axum::serve(listener, app).await.unwrap()
}

async fn print_order(Json(order): Json<Order>) -> StatusCode {
    match ReceiptPrinter::new(0x0416, 0x5011) {
        Ok(mut printer) => {
            printer.print_order(order).unwrap();
            StatusCode::OK
        }
        Err(e) => {
            println!("{}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        },
    }
}
