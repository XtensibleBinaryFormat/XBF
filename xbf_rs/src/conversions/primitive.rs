use crate::xdl_primitive::{XdlPrimitive, XdlPrimitiveMetadata};

impl From<&XdlPrimitive> for XdlPrimitiveMetadata {
    fn from(x: &XdlPrimitive) -> Self {
        match x {
            XdlPrimitive::Bool(_) => XdlPrimitiveMetadata::Bool,
            XdlPrimitive::U8(_) => XdlPrimitiveMetadata::U8,
            XdlPrimitive::U16(_) => XdlPrimitiveMetadata::U16,
            XdlPrimitive::U32(_) => XdlPrimitiveMetadata::U32,
            XdlPrimitive::U64(_) => XdlPrimitiveMetadata::U64,
            XdlPrimitive::U128(_) => XdlPrimitiveMetadata::U128,
            XdlPrimitive::U256(_) => XdlPrimitiveMetadata::U256,
            XdlPrimitive::I8(_) => XdlPrimitiveMetadata::I8,
            XdlPrimitive::I16(_) => XdlPrimitiveMetadata::I16,
            XdlPrimitive::I32(_) => XdlPrimitiveMetadata::I32,
            XdlPrimitive::I64(_) => XdlPrimitiveMetadata::I64,
            XdlPrimitive::I128(_) => XdlPrimitiveMetadata::I128,
            XdlPrimitive::I256(_) => XdlPrimitiveMetadata::I256,
            XdlPrimitive::F32(_) => XdlPrimitiveMetadata::F32,
            XdlPrimitive::F64(_) => XdlPrimitiveMetadata::F64,
            XdlPrimitive::String(_) => XdlPrimitiveMetadata::String,
        }
    }
}

macro_rules! xdl_primitive_from_native_impl {
    ($ty:ty, $xdl_type:tt) => {
        impl From<$ty> for XdlPrimitive {
            fn from(x: $ty) -> Self {
                XdlPrimitive::$xdl_type(x)
            }
        }
    };
}

xdl_primitive_from_native_impl!(bool, Bool);

xdl_primitive_from_native_impl!(u8, U8);
xdl_primitive_from_native_impl!(u16, U16);
xdl_primitive_from_native_impl!(u32, U32);
xdl_primitive_from_native_impl!(u64, U64);
xdl_primitive_from_native_impl!(u128, U128);

xdl_primitive_from_native_impl!(i8, I8);
xdl_primitive_from_native_impl!(i16, I16);
xdl_primitive_from_native_impl!(i32, I32);
xdl_primitive_from_native_impl!(i64, I64);
xdl_primitive_from_native_impl!(i128, I128);

xdl_primitive_from_native_impl!(f32, F32);
xdl_primitive_from_native_impl!(f64, F64);

xdl_primitive_from_native_impl!(String, String);

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! primitive_from_native_test {
        ($ty:ty, $xdl_type:tt, $test_num:expr) => {
            let value: $ty = $test_num;
            let primitive: XdlPrimitive = value.clone().into();
            assert_eq!(primitive, XdlPrimitive::$xdl_type(value));
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
            XdlPrimitiveMetadata::from(&XdlPrimitive::Bool(true)),
            XdlPrimitiveMetadata::Bool
        );
        assert_eq!(
            XdlPrimitiveMetadata::from(&XdlPrimitive::U8(1)),
            XdlPrimitiveMetadata::U8
        );
        assert_eq!(
            XdlPrimitiveMetadata::from(&XdlPrimitive::U16(1)),
            XdlPrimitiveMetadata::U16
        );
        assert_eq!(
            XdlPrimitiveMetadata::from(&XdlPrimitive::U32(1)),
            XdlPrimitiveMetadata::U32
        );
        assert_eq!(
            XdlPrimitiveMetadata::from(&XdlPrimitive::U64(1)),
            XdlPrimitiveMetadata::U64
        );
        assert_eq!(
            XdlPrimitiveMetadata::from(&XdlPrimitive::U128(1)),
            XdlPrimitiveMetadata::U128
        );
        assert_eq!(
            XdlPrimitiveMetadata::from(&XdlPrimitive::U256(())),
            XdlPrimitiveMetadata::U256
        );
        assert_eq!(
            XdlPrimitiveMetadata::from(&XdlPrimitive::I8(1)),
            XdlPrimitiveMetadata::I8
        );
        assert_eq!(
            XdlPrimitiveMetadata::from(&XdlPrimitive::I16(1)),
            XdlPrimitiveMetadata::I16
        );
        assert_eq!(
            XdlPrimitiveMetadata::from(&XdlPrimitive::I32(1)),
            XdlPrimitiveMetadata::I32
        );
        assert_eq!(
            XdlPrimitiveMetadata::from(&XdlPrimitive::I64(1)),
            XdlPrimitiveMetadata::I64
        );
        assert_eq!(
            XdlPrimitiveMetadata::from(&XdlPrimitive::I128(1)),
            XdlPrimitiveMetadata::I128
        );
        assert_eq!(
            XdlPrimitiveMetadata::from(&XdlPrimitive::I256(())),
            XdlPrimitiveMetadata::I256
        );
        assert_eq!(
            XdlPrimitiveMetadata::from(&XdlPrimitive::F32(1.0)),
            XdlPrimitiveMetadata::F32
        );
        assert_eq!(
            XdlPrimitiveMetadata::from(&XdlPrimitive::F64(1.0)),
            XdlPrimitiveMetadata::F64
        );
        assert_eq!(
            XdlPrimitiveMetadata::from(&XdlPrimitive::String("test".to_string())),
            XdlPrimitiveMetadata::String
        );
    }
}
