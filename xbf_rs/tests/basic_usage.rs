use indexmap::indexmap;
use xbf_rs::{
    prelude::*, XbfPrimitiveMetadata, XbfStruct, XbfStructMetadata, XbfVec, XbfVecMetadata,
    VEC_METADATA_DISCRIMINANT,
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

fn get_rider_metadata() -> XbfStructMetadata {
    thread_local! {
        static DRAGON_RIDER_METADATA: XbfStructMetadata = XbfStructMetadata::new(
                "DragonRider",
                indexmap! {
                    "Name".to_string() => XbfPrimitiveMetadata::String.into_base_metadata(),
                    "Age".to_string() => XbfPrimitiveMetadata::U16.into_base_metadata(),
                },
            );
    }

    DRAGON_RIDER_METADATA.with(|metadata| metadata.clone())
}

#[test]
fn basic_serialization() {
    let dragon_riders = [
        DragonRider::new("Eragon".to_string(), 16),
        DragonRider::new("Arya".to_string(), 103),
        DragonRider::new("Galbatorix".to_string(), 133),
    ]
    .map(XbfStruct::from);

    let xbf_vec_of_riders =
        XbfVec::new(XbfVecMetadata::new(get_rider_metadata()), dragon_riders).unwrap();

    let mut writer = vec![];

    xbf_vec_of_riders
        .get_metadata()
        .serialize_vec_metadata(&mut writer)
        .unwrap();
    xbf_vec_of_riders.serialize_vec_type(&mut writer).unwrap();

    let mut expected = vec![];
    expected.extend_from_slice(&(VEC_METADATA_DISCRIMINANT).to_le_bytes());
    get_rider_metadata()
        .serialize_struct_metadata(&mut expected)
        .unwrap();
    expected.extend_from_slice(&(xbf_vec_of_riders.len() as u16).to_le_bytes());
    for rider in &xbf_vec_of_riders {
        rider.serialize_base_type(&mut expected).unwrap();
    }

    assert_eq!(writer, expected);
}
