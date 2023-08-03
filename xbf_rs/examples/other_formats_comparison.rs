use anyhow::Result;
use indexmap::indexmap;
use reqwest::Client;
use serde::{Deserialize, Serialize};
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
    #[serde(rename = "Adj Close")]
    adj_close: f64,
}

#[derive(Serialize)]
struct XmlStockRecord {
    #[serde(rename = "Open")]
    open: f64,
    #[serde(rename = "High")]
    high: f64,
    #[serde(rename = "Low")]
    low: f64,
    #[serde(rename = "Close")]
    close: f64,
    #[serde(rename = "AdjClose")]
    adj_close: f64,
}

impl From<StockRecord> for XmlStockRecord {
    fn from(record: StockRecord) -> Self {
        Self {
            open: record.open,
            high: record.high,
            low: record.low,
            close: record.close,
            adj_close: record.adj_close,
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::new();
    let target = "https://query1.finance.yahoo.com/v7/finance/download/SONY?period1=1659398400&period2=1690934400&interval=1d&events=history&includeAdjustedClose=true";

    let response = client.get(target).send().await?;
    let csv_content = response.text().await?;
    println!("Base csv file size: {}", csv_content.as_bytes().len());

    let mut csv_reader = csv::Reader::from_reader(csv_content.as_bytes());
    let records: Vec<StockRecord> = csv_reader
        .deserialize()
        .map(|result| {
            let record: StockRecord = result?;
            Ok(record)
        })
        .collect::<Result<Vec<StockRecord>>>()?;
    println!(
        "native struct size: {}",
        std::mem::size_of::<StockRecord>() * records.len()
    );

    let msgpack_content = rmp_serde::to_vec(&records)?;
    println!("msgpack bytes size: {}", msgpack_content.len());

    let mut ciborium_content = vec![];
    ciborium::into_writer(&records, &mut ciborium_content)?;
    println!("ciborium bytes size: {}", ciborium_content.len());

    let json_content = serde_json::to_string(&records)?;
    println!("json file size: {}", json_content.as_bytes().len());

    let xml_vec = records
        .iter()
        .cloned()
        .map(|record| XmlStockRecord::from(record))
        .collect::<Vec<XmlStockRecord>>();
    let xml_content = quick_xml::se::to_string_with_root("root", &xml_vec)?;
    println!("xml file size: {}", xml_content.as_bytes().len());

    let xbf_records = {
        let sr_xbf_metadata = XbfStructMetadata::new(
            "StockRecord",
            indexmap! {
                "Open" => XbfPrimitiveMetadata::F64.into(),
                "High" => XbfPrimitiveMetadata::F64.into(),
                "Low" => XbfPrimitiveMetadata::F64.into(),
                "Close" => XbfPrimitiveMetadata::F64.into(),
                "Adj Close" => XbfPrimitiveMetadata::F64.into(),
            },
        );

        XbfVec::new_unchecked(
            XbfVecMetadata::new(sr_xbf_metadata.clone()),
            records.into_iter().map(|record| {
                XbfStruct::new_unchecked(
                    sr_xbf_metadata.clone(),
                    [
                        record.open.into_xbf_primitive().into_base_type(),
                        record.high.into_xbf_primitive().into_base_type(),
                        record.low.into_xbf_primitive().into_base_type(),
                        record.close.into_xbf_primitive().into_base_type(),
                        record.adj_close.into_xbf_primitive().into_base_type(),
                    ],
                )
            }),
        )
    };

    let mut xbf_bytes = vec![];
    xbf_records
        .get_metadata()
        .serialize_vec_metadata(&mut xbf_bytes)?;
    xbf_records.serialize_vec_type(&mut xbf_bytes)?;

    println!("xbf bytes size: {}", xbf_bytes.len());

    Ok(())
}
