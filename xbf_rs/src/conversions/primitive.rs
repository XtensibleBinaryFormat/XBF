use crate::xbf_primitive::{XbfPrimitive, XbfPrimitiveMetadata};

impl From<&XbfPrimitive> for XbfPrimitiveMetadata {
    fn from(x: &XbfPrimitive) -> Self {
        match x {
            XbfPrimitive::Bool(_) => XbfPrimitiveMetadata::Bool,
            XbfPrimitive::U8(_) => XbfPrimitiveMetadata::U8,
            XbfPrimitive::U16(_) => XbfPrimitiveMetadata::U16,
            XbfPrimitive::U32(_) => XbfPrimitiveMetadata::U32,
            XbfPrimitive::U64(_) => XbfPrimitiveMetadata::U64,
            XbfPrimitive::U128(_) => XbfPrimitiveMetadata::U128,
            XbfPrimitive::U256(_) => XbfPrimitiveMetadata::U256,
            XbfPrimitive::I8(_) => XbfPrimitiveMetadata::I8,
            XbfPrimitive::I16(_) => XbfPrimitiveMetadata::I16,
            XbfPrimitive::I32(_) => XbfPrimitiveMetadata::I32,
            XbfPrimitive::I64(_) => XbfPrimitiveMetadata::I64,
            XbfPrimitive::I128(_) => XbfPrimitiveMetadata::I128,
            XbfPrimitive::I256(_) => XbfPrimitiveMetadata::I256,
            XbfPrimitive::F32(_) => XbfPrimitiveMetadata::F32,
            XbfPrimitive::F64(_) => XbfPrimitiveMetadata::F64,
            XbfPrimitive::String(_) => XbfPrimitiveMetadata::String,
        }
    }
}

macro_rules! xbf_primitive_from_native_impl {
    ($ty:ty, $xbf_type:tt) => {
        impl From<$ty> for XbfPrimitive {
            fn from(x: $ty) -> Self {
                XbfPrimitive::$xbf_type(x)
            }
        }
    };
}

xbf_primitive_from_native_impl!(bool, Bool);

xbf_primitive_from_native_impl!(u8, U8);
xbf_primitive_from_native_impl!(u16, U16);
xbf_primitive_from_native_impl!(u32, U32);
xbf_primitive_from_native_impl!(u64, U64);
xbf_primitive_from_native_impl!(u128, U128);

xbf_primitive_from_native_impl!(i8, I8);
xbf_primitive_from_native_impl!(i16, I16);
xbf_primitive_from_native_impl!(i32, I32);
xbf_primitive_from_native_impl!(i64, I64);
xbf_primitive_from_native_impl!(i128, I128);

xbf_primitive_from_native_impl!(f32, F32);
xbf_primitive_from_native_impl!(f64, F64);

xbf_primitive_from_native_impl!(String, String);

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! primitive_from_native_test {
        ($ty:ty, $xbf_type:tt, $test_num:expr) => {
            let value: $ty = $test_num;
            let primitive: XbfPrimitive = value.clone().into();
            assert_eq!(primitive, XbfPrimitive::$xbf_type(value));
        };
    }

    #[test]
    fn primitive_from_native_works() {
        primitive_from_native_test!(bool, Bool, true);
        primitive_from_native_test!(bool, Bool, false);
        primitive_from_native_test!(u8, U8, 42);
        primitive_from_native_test!(u16, U16, 42);
        primitive_from_native_test!(u32, U32, 42);
        primitive_from_native_test!(u64, U64, 42);
        primitive_from_native_test!(u128, U128, 42);
        primitive_from_native_test!(i8, I8, 42);
        primitive_from_native_test!(i16, I16, 42);
        primitive_from_native_test!(i32, I32, 42);
        primitive_from_native_test!(i64, I64, 42);
        primitive_from_native_test!(i128, I128, 42);
        primitive_from_native_test!(f32, F32, 42.0);
        primitive_from_native_test!(f64, F64, 42.0);
        primitive_from_native_test!(String, String, "Hello World".to_string());
    }

    #[test]
    fn primitve_metadata_from_primitive_works() {
        assert_eq!(
            XbfPrimitiveMetadata::from(&XbfPrimitive::Bool(true)),
            XbfPrimitiveMetadata::Bool
        );
        assert_eq!(
            XbfPrimitiveMetadata::from(&XbfPrimitive::U8(1)),
            XbfPrimitiveMetadata::U8
        );
        assert_eq!(
            XbfPrimitiveMetadata::from(&XbfPrimitive::U16(1)),
            XbfPrimitiveMetadata::U16
        );
        assert_eq!(
            XbfPrimitiveMetadata::from(&XbfPrimitive::U32(1)),
            XbfPrimitiveMetadata::U32
        );
        assert_eq!(
            XbfPrimitiveMetadata::from(&XbfPrimitive::U64(1)),
            XbfPrimitiveMetadata::U64
        );
        assert_eq!(
            XbfPrimitiveMetadata::from(&XbfPrimitive::U128(1)),
            XbfPrimitiveMetadata::U128
        );
        assert_eq!(
            XbfPrimitiveMetadata::from(&XbfPrimitive::U256(())),
            XbfPrimitiveMetadata::U256
        );
        assert_eq!(
            XbfPrimitiveMetadata::from(&XbfPrimitive::I8(1)),
            XbfPrimitiveMetadata::I8
        );
        assert_eq!(
            XbfPrimitiveMetadata::from(&XbfPrimitive::I16(1)),
            XbfPrimitiveMetadata::I16
        );
        assert_eq!(
            XbfPrimitiveMetadata::from(&XbfPrimitive::I32(1)),
            XbfPrimitiveMetadata::I32
        );
        assert_eq!(
            XbfPrimitiveMetadata::from(&XbfPrimitive::I64(1)),
            XbfPrimitiveMetadata::I64
        );
        assert_eq!(
            XbfPrimitiveMetadata::from(&XbfPrimitive::I128(1)),
            XbfPrimitiveMetadata::I128
        );
        assert_eq!(
            XbfPrimitiveMetadata::from(&XbfPrimitive::I256(())),
            XbfPrimitiveMetadata::I256
        );
        assert_eq!(
            XbfPrimitiveMetadata::from(&XbfPrimitive::F32(1.0)),
            XbfPrimitiveMetadata::F32
        );
        assert_eq!(
            XbfPrimitiveMetadata::from(&XbfPrimitive::F64(1.0)),
            XbfPrimitiveMetadata::F64
        );
        assert_eq!(
            XbfPrimitiveMetadata::from(&XbfPrimitive::String("test".to_string())),
            XbfPrimitiveMetadata::String
        );
    }
}
