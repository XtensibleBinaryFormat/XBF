use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};
use xbf_rs::{
    NativeToXbfPrimitive, XbfPrimitiveMetadata, XbfStruct, XbfStructMetadata, XbfTypeUpcast,
    XbfVec, XbfVecMetadata,
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

fn to_json(records: &[StockRecord]) -> Result<Vec<u8>, serde_json::Error> {
    Ok(serde_json::to_string(records)?.into_bytes())
}

fn to_xml(records: &[StockRecord]) -> Result<Vec<u8>, quick_xml::de::DeError> {
    Ok(quick_xml::se::to_string(records)?.into_bytes())
}

fn to_xbf(records: &[StockRecord]) -> Result<Vec<u8>, std::io::Error> {
    let sr_xbf_metadata = XbfStructMetadata::new(
        "StockRecord",
        indexmap::indexmap! {
            "Open" => XbfPrimitiveMetadata::F64.into(),
            "High" => XbfPrimitiveMetadata::F64.into(),
            "Low" => XbfPrimitiveMetadata::F64.into(),
            "Close" => XbfPrimitiveMetadata::F64.into(),
            "AdjClose" => XbfPrimitiveMetadata::F64.into(),
        },
    );

    let vec = XbfVec::new_unchecked(
        XbfVecMetadata::new(sr_xbf_metadata.clone()),
        records.into_iter().map(|record| {
            XbfStruct::new_unchecked(
                sr_xbf_metadata.clone(),
                [
                    record.open.to_xbf_primitive().into_base_type(),
                    record.high.to_xbf_primitive().into_base_type(),
                    record.low.to_xbf_primitive().into_base_type(),
                    record.close.to_xbf_primitive().into_base_type(),
                    record.adj_close.to_xbf_primitive().into_base_type(),
                ],
            )
        }),
    );

    let mut bytes = vec![];
    vec.get_metadata().serialize_vec_metadata(&mut bytes)?;
    vec.serialize_vec_type(&mut bytes)?;

    Ok(bytes)
}

#[repr(u8)]
#[derive(Debug)]
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

    let bytes = match request_type {
        RequestType::Csv => to_csv(records)?,
        RequestType::MessagePack => to_msgpack(records)?,
        RequestType::Cbor => to_cbor(records)?,
        RequestType::Json => to_json(records)?,
        RequestType::Xml => to_xml(records)?,
        RequestType::Xbf => to_xbf(records)?,
        RequestType::Unknown => "bruh what".into(),
    };

    eprintln!("request type: {:?}", request_type);
    eprintln!("bytes to write: {}", bytes.len());

    stream.write_all(&bytes).await?;
    stream.flush().await?;

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let native_data = {
        let csv_data = get_yahoo_data().await?;
        let native_data = get_native_vec_from_csv(&csv_data)?;
        Arc::new(native_data)
    };

    let listener = TcpListener::bind("0.0.0.0:42069").await?;

    loop {
        if let Ok((request, _)) = listener.accept().await {
            let data = Arc::clone(&native_data);
            tokio::spawn(async move {
                if let Err(e) = handle_request(request, &data).await {
                    eprintln!("{}", e);
                }
            });
        }
    }
}
