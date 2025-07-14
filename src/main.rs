use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct CandleSnapshotBody {
    #[serde(rename = "type")]
    req_type: String,
    #[serde(rename = "req")]
    req_object: CandleSnapshotReq,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct CandleSnapshotReq {
    coin: String,
    interval: String,
    start_time: u64,
    end_time: u64,
}

#[derive(Deserialize, Debug)]
struct CandleSnapshotResponse {
    #[serde(rename = "T")]
    close_time: u64,
    #[serde(rename = "c")]
    close_price: String,
    #[serde(rename = "i")]
    interval: String,
    #[serde(rename = "l")]
    low_price: String,
    #[serde(rename = "n")]
    number_of_trades: u64,
    #[serde(rename = "o")]
    open_price: String,
    #[serde(rename = "s")]
    symbol: String,
    #[serde(rename = "t")]
    open_time: u64,
    #[serde(rename = "v")]
    volume: String,
}

#[tokio::main]
async fn main() {
    let obj = CandleSnapshotReq {
        coin: "BTC".to_string(),
        interval: "5m".to_string(),
        start_time: 1752022800000,
        end_time: 1752026400000,
    };

    let body = CandleSnapshotBody {
        req_type: "candleSnapshot".to_string(),
        req_object: obj,
    };

    let a = serde_json::to_string(&body).unwrap();
    println!("{a}");

    let resp = reqwest::Client::new()
        .post("http://api.hyperliquid.xyz/info")
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .json(&body)
        .send()
        .await
        .unwrap();

    let candles: Vec<CandleSnapshotResponse> = resp.json().await.unwrap();

    println!("total candles: {}", candles.len());
    for candle in candles {
        println!("{candle:#?}");
    }
}
