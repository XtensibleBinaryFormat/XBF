use byteorder::ReadBytesExt;
use indexmap::indexmap;
use std::{
    io::{BufReader, BufWriter, Write},
    net::TcpListener,
    sync::OnceLock,
};
use xbf_rs::{
    prelude::*, XbfPrimitiveMetadata, XbfStruct, XbfStructMetadata, XbfVec, XbfVecMetadata,
};

struct DragonRider {
    name: String,
    age: u16,
}

impl DragonRider {
    fn new(name: String, age: u16) -> Self {
        Self { name, age }
    }
}

impl From<DragonRider> for XbfStruct {
    fn from(value: DragonRider) -> Self {
        XbfStruct::new_unchecked(
            get_rider_metadata(),
            [
                value.name.to_xbf_primitive().to_base_type(),
                value.age.to_xbf_primitive().to_base_type(),
            ],
        )
    }
}

static RIDER_METADATA: OnceLock<XbfStructMetadata> = OnceLock::new();
static RIDER_VEC_METADATA: OnceLock<XbfVecMetadata> = OnceLock::new();
static SER_VEC_METADATA: OnceLock<Vec<u8>> = OnceLock::new();
static SER_VEC: OnceLock<Vec<u8>> = OnceLock::new();

fn get_rider_metadata() -> XbfStructMetadata {
    RIDER_METADATA
        .get_or_init(|| {
            XbfStructMetadata::new(
                "DragonRider",
                indexmap! {
                    "name" => XbfPrimitiveMetadata::String.into(),
                    "age" => XbfPrimitiveMetadata::U16.into(),
                },
            )
        })
        .clone()
}

fn get_vec_metadata() -> XbfVecMetadata {
    RIDER_VEC_METADATA
        .get_or_init(|| XbfVecMetadata::new(get_rider_metadata()))
        .clone()
}

fn get_serialized_metadata() -> &'static [u8] {
    SER_VEC_METADATA.get_or_init(|| {
        let mut writer = vec![];
        get_rider_metadata()
            .serialize_struct_metadata(&mut writer)
            .unwrap();
        writer
    })
}

fn get_serialized_data() -> &'static [u8] {
    SER_VEC.get_or_init(|| {
        let mut writer = vec![];
        let data = XbfVec::new_unchecked(
            get_vec_metadata(),
            [
                DragonRider::new("Eragon".to_string(), 16),
                DragonRider::new("Arya".to_string(), 103),
                DragonRider::new("Galbatorix".to_string(), 133),
            ]
            .map(XbfStruct::from),
        );
        data.serialize_vec_type(&mut writer).unwrap();
        writer
    })
}

fn main() {
    let stream = TcpListener::bind("127.0.0.1:6969").unwrap();

    for stream in stream.incoming() {
        if let Ok(stream) = stream {
            println!("Accepted connection from {}", stream.peer_addr().unwrap());
            let mut reader = BufReader::new(stream.try_clone().unwrap());
            let mut writer = BufWriter::new(stream);

            let asked_for = reader.read_u8().unwrap();
            if asked_for == 0 {
                writer.write_all(get_serialized_metadata()).unwrap();
            }
            writer.write_all(get_serialized_data()).unwrap();

            writer.flush().unwrap();
        }
    }
}
