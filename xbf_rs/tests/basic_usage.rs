use indexmap::indexmap;
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

thread_local! {
    static DRAGON_RIDER_METADATA: XbfStructMetadata = XbfStructMetadata::new(
            "DragonRider".to_string(),
            indexmap! {
                "Name".to_string() => XbfPrimitiveMetadata::String.into_base_metadata(),
                "Age".to_string() => XbfPrimitiveMetadata::U16.into_base_metadata(),
            },
        );
}

impl DragonRider {
    fn new(name: String, age: u16) -> Self {
        Self { name, age }
    }

    fn to_xbf_struct(&self) -> XbfStruct {
        XbfStruct::new_unchecked(
            DRAGON_RIDER_METADATA.with(|metadata| metadata.clone()),
            vec![
                self.name.to_xbf_primitive().to_base_type(),
                self.age.to_xbf_primitive().to_base_type(),
            ],
        )
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

    // native vector of the base XbfType
    let vec_of_xbf_rider_structs = dragon_riders
        .iter()
        .map(|rider| rider.to_xbf_struct().to_base_type())
        .collect();

    // metadata for an XbfVec containing DragonRider structs
    let xbf_vec_of_riders_metadata = XbfVecMetadata::new(
        DRAGON_RIDER_METADATA
            .with(|metadata| metadata.clone())
            .to_base_metadata(),
    );

    // XbfVec of DragonRider structs, which we can now serialize
    let xbf_vec_of_riders =
        XbfVec::new(xbf_vec_of_riders_metadata.clone(), vec_of_xbf_rider_structs)
            .expect("vec is valid");

    // serializing to a buffer here, but this could be anything that implements the `Write` trait
    let mut writer = vec![];
    // serialize the metadata
    xbf_vec_of_riders_metadata
        .serialize_vec_metadata(&mut writer)
        .unwrap();
    // serialize the vector itself
    xbf_vec_of_riders.serialize_vec_type(&mut writer).unwrap();

    let mut expected = vec![];
    expected.extend_from_slice(&(VEC_METADATA_DISCRIMINANT).to_le_bytes());
    DRAGON_RIDER_METADATA
        .with(|metadata| metadata.clone())
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
