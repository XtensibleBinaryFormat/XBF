use fake::{faker::address::en::StreetName, faker::name::en::Name, Dummy};
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

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Dummy)]
struct Person {
    #[dummy(faker = "Name()")]
    name: String,
    #[dummy(faker = "1..120")]
    age: u8,
    #[dummy(faker = "StreetName()")]
    address: String,
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

fn to_csv<T: Serialize>(records: &[T]) -> anyhow::Result<Vec<u8>> {
    let mut csv_writer = csv::Writer::from_writer(vec![]);
    csv_writer.serialize(records)?;
    Ok(csv_writer.into_inner()?)
}

fn to_msgpack<T: Serialize>(records: &[T]) -> Result<Vec<u8>, rmp_serde::encode::Error> {
    rmp_serde::to_vec(records)
}

fn to_cbor<T: Serialize>(records: &[T]) -> anyhow::Result<Vec<u8>> {
    let mut vec = vec![];
    ciborium::into_writer(&records, &mut vec)?;
    Ok(vec)
}

fn to_json<T: Serialize>(records: &[T]) -> Result<Vec<u8>, serde_json::Error> {
    Ok(serde_json::to_string(records)?.into_bytes())
}

fn to_xml<T: Serialize>(records: &[T]) -> Result<Vec<u8>, quick_xml::de::DeError> {
    Ok(quick_xml::se::to_string_with_root("root", records)?.into_bytes())
}

fn stocks_to_xbf(records: &[StockRecord]) -> Result<Vec<u8>, std::io::Error> {
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

fn people_to_xbf(records: &[Person]) -> Result<Vec<u8>, std::io::Error> {
    let person_xbf_metadata = XbfStructMetadata::new(
        "Person",
        indexmap::indexmap! {
            "Name" => XbfPrimitiveMetadata::String.into(),
            "Age" => XbfPrimitiveMetadata::U8.into(),
            "Address" => XbfPrimitiveMetadata::String.into(),
        },
    );

    let vec = XbfVec::new_unchecked(
        XbfVecMetadata::new(person_xbf_metadata.clone()),
        records.into_iter().map(|person| {
            XbfStruct::new_unchecked(
                person_xbf_metadata.clone(),
                [
                    person.name.to_xbf_primitive().into_base_type(),
                    person.age.to_xbf_primitive().into_base_type(),
                    person.address.to_xbf_primitive().into_base_type(),
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
    Stock,
    Person,
    Unknown,
}

impl From<u8> for RequestType {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Stock,
            1 => Self::Person,
            _ => Self::Unknown,
        }
    }
}

#[repr(u8)]
#[derive(Debug)]
enum DataFormat {
    Csv,
    MessagePack,
    Cbor,
    Json,
    Xml,
    Xbf,
    Unknown,
}

impl From<u8> for DataFormat {
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

async fn handle_stock_request(
    mut stream: TcpStream,
    records: &[StockRecord],
) -> anyhow::Result<()> {
    let data_format = DataFormat::from(stream.read_u8().await?);
    eprintln!("data format: {:?}", data_format);

    let bytes = match data_format {
        DataFormat::Csv => to_csv(records)?,
        DataFormat::MessagePack => to_msgpack(records)?,
        DataFormat::Cbor => to_cbor(records)?,
        DataFormat::Json => to_json(records)?,
        DataFormat::Xml => to_xml(records)?,
        DataFormat::Xbf => stocks_to_xbf(records)?,
        DataFormat::Unknown => "Unknown request type".into(),
    };

    stream.write_all(&bytes).await?;
    stream.flush().await?;

    Ok(())
}

async fn handle_person_request(mut stream: TcpStream, records: &[Person]) -> anyhow::Result<()> {
    let data_format = DataFormat::from(stream.read_u8().await?);

    let bytes = match data_format {
        DataFormat::Csv => to_csv(records)?,
        DataFormat::MessagePack => to_msgpack(records)?,
        DataFormat::Cbor => to_cbor(records)?,
        DataFormat::Json => to_json(records)?,
        DataFormat::Xml => to_xml(records)?,
        DataFormat::Xbf => people_to_xbf(records)?,
        DataFormat::Unknown => "Unknown request type".into(),
    };

    stream.write_all(&bytes).await?;
    stream.flush().await?;

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let stock_data = {
        let csv_data = get_yahoo_data().await?;
        println!(
            "original stock csv data size: {}",
            csv_data.as_bytes().len()
        );
        let native_data = get_native_vec_from_csv(&csv_data)?;
        Arc::new(native_data)
    };
    let person_data = {
        let data = Arc::new(fake::vec![Person; 500]);
        let total_size = data.iter().fold(0usize, |acc, x| {
            let name_size = x.name.as_bytes().len();
            let address_size = x.address.as_bytes().len();
            let age_size = std::mem::size_of::<u8>();
            acc + name_size + address_size + age_size
        });
        println!("original random person data size: {}", total_size);
        data
    };

    let listener = TcpListener::bind("0.0.0.0:42069").await?;
    eprintln!("server listening on 0.0.0.0:42069");

    loop {
        if let Ok((mut request, _)) = listener.accept().await {
            eprintln!("connection from {}", request.peer_addr()?);
            let request_type = RequestType::from(request.read_u8().await?);
            eprintln!("request type: {:?}", request_type);

            match request_type {
                RequestType::Stock => {
                    let data = Arc::clone(&stock_data);
                    tokio::spawn(async move {
                        if let Err(e) = handle_stock_request(request, &data).await {
                            eprintln!("{}", e);
                        }
                    });
                }
                RequestType::Person => {
                    let data = Arc::clone(&person_data);
                    tokio::spawn(async move {
                        if let Err(e) = handle_person_request(request, &data).await {
                            eprintln!("{}", e);
                        }
                    });
                }
                _ => {}
            }
        }
    }
}
