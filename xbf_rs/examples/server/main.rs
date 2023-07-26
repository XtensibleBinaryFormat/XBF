mod storage;

use crate::storage::SerializedStorage;
use byteorder::ReadBytesExt;
use indexmap::indexmap;
use std::{
    io::{BufReader, BufWriter},
    net::TcpListener,
};
use xbf_rs::{
    prelude::*, XbfPrimitive, XbfPrimitiveMetadata, XbfStruct, XbfStructMetadata, XbfVec,
    XbfVecMetadata,
};

struct DragonRider {
    name: String,
    age: u16,
}

impl DragonRider {
    fn new(name: String, age: u16) -> Self {
        Self { name, age }
    }

    fn into_xbf_struct(self, metadata: XbfStructMetadata) -> XbfStruct {
        XbfStruct::new_unchecked(
            metadata,
            [
                self.name.into_xbf_primitive().into_base_type(),
                self.age.into_xbf_primitive().into_base_type(),
            ],
        )
    }
}

#[repr(u8)]
enum RequestType {
    GetData = 0,
    GetMetadataAndData,
    Unknown,
}

impl From<u8> for RequestType {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::GetData,
            1 => Self::GetMetadataAndData,
            _ => Self::Unknown,
        }
    }
}

fn main() {
    let dragon_riders_storage = {
        let dragon_rider_metadata = XbfStructMetadata::new(
            "DragonRider",
            indexmap! {
                "name" => XbfPrimitiveMetadata::String.into(),
                "age" => XbfPrimitiveMetadata::U16.into(),
            },
        );
        let dragon_riders = [
            DragonRider::new("Eragon".to_string(), 16),
            DragonRider::new("Arya".to_string(), 103),
            DragonRider::new("Galbatorix".to_string(), 133),
        ]
        .map(|dragon_rider| dragon_rider.into_xbf_struct(dragon_rider_metadata.clone()));

        SerializedStorage::new(XbfVec::new_unchecked(
            XbfVecMetadata::new(dragon_rider_metadata),
            dragon_riders,
        ))
    };

    let unknown_request_message = XbfPrimitive::String("Unknown request type".to_string());

    let stream = TcpListener::bind("127.0.0.1:6969").unwrap();

    for stream in stream.incoming() {
        if let Ok(stream) = stream {
            let peer_addr = stream.peer_addr();
            match peer_addr {
                Ok(addr) => println!("Accepted connection from {addr}"),
                Err(e) => {
                    eprintln!("Unable to get peer address: {e}");
                    continue;
                }
            }

            let mut reader = {
                let cloned_stream = stream.try_clone();
                match cloned_stream {
                    Ok(stream) => BufReader::new(stream),
                    Err(e) => {
                        eprintln!("Unable to clone stream: {e}");
                        continue;
                    }
                }
            };
            let mut writer = BufWriter::new(stream);

            let response = reader.read_u8().and_then(|request_type| {
                let request_type = RequestType::from(request_type);
                match request_type {
                    RequestType::GetData => dragon_riders_storage.write_metadata(&mut writer),
                    RequestType::GetMetadataAndData => dragon_riders_storage
                        .write_metadata(&mut writer)
                        .and_then(|_| dragon_riders_storage.write_value(&mut writer)),
                    RequestType::Unknown => unknown_request_message
                        .get_metadata()
                        .serialize_primitive_metadata(&mut writer)
                        .and_then(|_| {
                            unknown_request_message.serialize_primitive_type(&mut writer)
                        }),
                }
            });

            if let Err(e) = response {
                eprintln!("Unable to write response: {e}");
            }
        }
    }
}
