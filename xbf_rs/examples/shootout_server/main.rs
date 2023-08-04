use std::io::Write;

use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

#[derive(Serialize, Deserialize, Clone)]
struct StockRecord {
    #[serde(rename = "Open")]
    open: f64,
    #[serde(rename = "High")]
    high: f64,
    #[serde(rename = "Low")]
    low: f64,
    #[serde(rename = "Close")]
    close: f64,
    #[serde(rename(serialize = "AdjClose", deserialize = "Adj Close"))]
    adj_close: f64,
}

async fn get_yahoo_data() -> Result<String, reqwest::Error> {
    let client = Client::new();
    let target = "https://query1.finance.yahoo.com/v7/finance/download/SONY?period1=1659398400&period2=1690934400&interval=1d&events=history&includeAdjustedClose=true";

    let response = client.get(target).send().await?;
    let csv_content = response.text().await?;
    Ok(csv_content)
}

fn get_native_vec_from_csv(csv_content: &str) -> Result<Vec<StockRecord>, csv::Error> {
    let mut csv_reader = csv::Reader::from_reader(csv_content.as_bytes());
    csv_reader.deserialize::<StockRecord>().collect()
}

fn to_csv(records: &[StockRecord]) -> anyhow::Result<Vec<u8>> {
    let mut csv_writer = csv::Writer::from_writer(vec![]);
    csv_writer.serialize(records)?;
    Ok(csv_writer.into_inner()?)
}

fn to_msgpack(records: &[StockRecord]) -> Result<Vec<u8>, rmp_serde::encode::Error> {
    rmp_serde::to_vec(records)
}

fn to_cbor(records: &[StockRecord]) -> anyhow::Result<Vec<u8>> {
    let mut vec = vec![];
    ciborium::into_writer(&records, &mut vec)?;
    Ok(vec)
}

#[repr(u8)]
enum RequestType {
    Csv,
    MessagePack,
    Cbor,
    Json,
    Xml,
    Xbf,
    Unknown,
}

impl From<u8> for RequestType {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Csv,
            1 => Self::MessagePack,
            2 => Self::Cbor,
            3 => Self::Json,
            4 => Self::Xml,
            5 => Self::Xbf,
            _ => Self::Unknown,
        }
    }
}

async fn handle_request(mut stream: TcpStream, records: &[StockRecord]) -> anyhow::Result<()> {
    let request_type = RequestType::from(stream.read_u8().await?);

    match request_type {
        RequestType::Csv => {
            let csv_content = to_csv(records)?;
            stream.write_all(&csv_content).await?;
        }
        RequestType::MessagePack => {
            let msgpack_content = to_msgpack(records)?;
            stream.write_all(&msgpack_content).await?;
        }
        RequestType::Cbor => {
            let cbor_content = to_cbor(records)?;
            stream.write_all(&cbor_content).await?;
        }
        RequestType::Json => todo!(),
        RequestType::Xml => todo!(),
        RequestType::Xbf => todo!(),
        RequestType::Unknown => todo!(),
    }

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let native_data = {
        let csv_data = get_yahoo_data().await?;
        let native_data = get_native_vec_from_csv(&csv_data)?;
        native_data
    };

    Ok(())
}
