use serde::{Deserialize, Serialize};
use xbf_rs::{
    prelude::*, XbfPrimitiveMetadata, XbfStruct, XbfStructMetadata, XbfVec, XbfVecMetadata,
    VEC_METADATA_DISCRIMINANT,
};

#[derive(Serialize, Deserialize)]
struct DragonRider {
    name: String,
    age: u16,
}

impl DragonRider {
    fn new(name: String, age: u16) -> Self {
        Self { name, age }
    }
}

#[test]
fn basic_serialization() {
    // Native struct and vector
    let dragon_riders = vec![
        DragonRider::new("Eragon".to_string(), 16),
        DragonRider::new("Arya".to_string(), 103),
        DragonRider::new("Galbatorix".to_string(), 133),
    ];

    // metadata for the struct
    let dragon_rider_metadata = XbfStructMetadata::new(
        "DragonRider".to_string(),
        vec![
            (
                "Name".to_string(),
                XbfPrimitiveMetadata::String.into_base_metadata(),
            ),
            (
                "Age".to_string(),
                XbfPrimitiveMetadata::U16.into_base_metadata(),
            ),
        ],
    )
    .unwrap();

    // Make a native vec of XbfType, where the XbfType is an XbfStruct corresponding to the native
    // DragonRider struct
    let vec_of_xbf_rider_structs = dragon_riders
        .iter()
        .map(|rider| {
            XbfStruct::new(
                dragon_rider_metadata.clone(),
                vec![
                    rider.name.to_xbf_primitive().to_base_type(),
                    rider.age.to_xbf_primitive().to_base_type(),
                ],
            )
            .expect("struct is valid")
            .to_base_type()
        })
        .collect();

    let xbf_vec_of_riders_metadata =
        XbfVecMetadata::new(dragon_rider_metadata.clone().to_base_metadata());

    // Make an XbfVec from the native vec and convert it to the base type so we can serialize it
    // with its metadata.
    let xbf_vec_of_riders =
        XbfVec::new(xbf_vec_of_riders_metadata.clone(), vec_of_xbf_rider_structs)
            .expect("vec is valid");

    let mut writer = vec![];
    xbf_vec_of_riders_metadata
        .serialize_vec_metadata(&mut writer)
        .unwrap();
    xbf_vec_of_riders.serialize_vec_type(&mut writer).unwrap();

    let mut expected = vec![];
    expected.extend_from_slice(&(VEC_METADATA_DISCRIMINANT).to_le_bytes());
    dragon_rider_metadata
        .serialize_struct_metadata(&mut expected)
        .unwrap();
    expected.extend_from_slice(&(xbf_vec_of_riders.len() as u16).to_le_bytes());
    for rider in &xbf_vec_of_riders {
        rider.serialize_base_type(&mut expected).unwrap();
    }

    assert_eq!(writer, expected);

    // comparison to serde
    let json = serde_json::to_string(&dragon_riders).unwrap();
    assert!(writer.len() < json.len());
}
