#include <iostream>
#include <sstream>
#include <string>
#include <vector>
#include <map>
#include <iomanip>
#include "rust.hh"
#include "metadata.hh"
#include "Buffer.hh"
using namespace std;

// Forward declarations for functions
template <typename T>
void unchecked_write_vector(const std::vector<T>& v, Buffer& b);

template <typename T>
std::vector<T> unchecked_read_vector(Buffer& b);


class vec3d {
private:
    double x, y, z;

public:
    vec3d() : x(0.0), y(0.0), z(0.0) {} // Default constructor

    vec3d(double x, double y, double z) : x(x), y(y), z(z) {}

    // Serialize the vec3d object
    template <class Archive>
    void serialize(Archive& ar, const unsigned int version) {
        ar & boost::serialization::make_nvp("x", x);
        ar & boost::serialization::make_nvp("y", y);
        ar & boost::serialization::make_nvp("z", z);
    }
};

BOOST_CLASS_EXPORT(vec3d)

BOOST_SERIALIZATION_ASSUME_ABSTRACT(Metadata)

BOOST_CLASS_EXPORT(PrimitiveMetadata)
BOOST_CLASS_EXPORT(VecMetadata)
BOOST_CLASS_EXPORT(StructMetadata)


void test_serde_vec_primitive(Buffer& b) {
    vector<u8> boolVec{ 1, 0, 1 };
    vector<u8> u8Vec{ 1, 2, 3, 0, 255 };
    vector<u16> u16Vec{ 1000, 2000, 3000, 0, 65535 };
    vector<u32> u32Vec{ 1'000'000, 2'000'000, 3'000'000 };
    vector<u64> u64Vec{ 8'000'000'000ULL, 9'000'000'000ULL, 10'000'000'000ULL };
    // TODO: Add vectors for other primitive types

    // Serialization
    unchecked_write_vector(boolVec, b);
    unchecked_write_vector(u8Vec, b);
    unchecked_write_vector(u16Vec, b);
    unchecked_write_vector(u32Vec, b);
    unchecked_write_vector(u64Vec, b);

    // TODO: Serialize other vectors
    b.dump(cout);

    // Deserialization
    vector<u8> desBoolVec = unchecked_read_vector<u8>(b);
    vector<u8> desU8Vec = unchecked_read_vector<u8>(b);
    vector<u16> desU16Vec = unchecked_read_vector<u16>(b);
    vector<u32> desU32Vec = unchecked_read_vector<u32>(b);
    vector<u64> desU64Vec = unchecked_read_vector<u64>(b);

    // Assertions
    assert(boolVec == desBoolVec);
    assert(u8Vec == desU8Vec);
    assert(u16Vec == desU16Vec);
    assert(u32Vec == desU32Vec);
    assert(u64Vec == desU64Vec);
    // TODO: Add assertions for other vectors    
}


void test_serde_vec_metadata(Buffer &b) {
    // Serialization
    const PrimitiveType internalType = PrimitiveType::U16;
    const PrimitiveType vecMetadata = internalType;

    b << vecMetadata;
    b.dump(cout);
    // Deserialization

    PrimitiveType desVecMetadata;
    b >> desVecMetadata;

    // Assertions
    assert(desVecMetadata == PrimitiveType::U16);
}

void test_serde_primitives(Buffer &b){

        bool b1 = false;
        u8 p1 = 3;
        u16 p2 = 1000;
        u32 p3 = 1'000'000;
        u64 p4  = 8'000'000'000ULL;
        //U128
        //U256,
        i8 i1 = 127;
        i16 i2 = 32767;
        i32 i3 = 214748364;
        i64 i4 = 9000000000000;
       // I128,
        //I256,
        f32 f1 = 3.40282347e+36;
        f64 f2 =  1.7976931348623157e+306;
        byte b2 = static_cast<byte>(1);
        string s1 = "hello";
    //serialization
    //b << b1 << p1 << p2 << p3 << p4 << i1 << i2 << i3 << i4 << f1 << f2 << b2 << s1;
    b.unchecked_write(b1);
    b.unchecked_write(p1);
    b.unchecked_write(p2);
    b.unchecked_write(p3);
    b.unchecked_write(p4);
    b.unchecked_write(i1);
    b.unchecked_write(i2);
    b.unchecked_write(i3);
    b.unchecked_write(i4);
    b.unchecked_write(f1);
    b.unchecked_write(f2);
    b.unchecked_write(b2);
    b.unchecked_write(s1);

    b.dump(cout);
    b.reset_pointer_to_buffer();

    //deserialisation

    /*bool des_b1;
    u8 des_p1;
    u16 des_p2;
    u32 des_p3;
    u64 des_p4;
    i8 des_i1;
    i16 des_i2;
    i32 des_i3;
    i64 des_i4;
    f32 des_f1;
    f64 des_f2;
    byte des_b2;
    string des_s1;*/
    bool des_b1 = b.unchecked_read<bool>();
    u8 des_p1 = b.unchecked_read<u8>();
    u16 des_p2 = b.unchecked_read<u16>();
    u32 des_p3 = b.unchecked_read<u32>();
    u64 des_p4 = b.unchecked_read<u64>();
    i8 des_i1 = b.unchecked_read<i8>();
    i16 des_i2 = b.unchecked_read<i16>();
    i32 des_i3 = b.unchecked_read<i32>();
    i64 des_i4 = b.unchecked_read<i64>();
    f32 des_f1 = b.unchecked_read<f32>();
    f64 des_f2 = b.unchecked_read<f64>();
    byte des_b2 = b.unchecked_read<byte>();
    string des_s1 = b.readString(); 
    
    //b  >>  des_b1 >> des_p1 >> des_p2 >> des_p3 >> des_p4 >> des_i1 >> des_i2 >> des_i3 >> des_i4 >> des_f1>> des_f2 >> des_b2 >> des_s1;
    //cout << "p1: " << static_cast<int>(p1) << ", des_p1: " << static_cast<int>(des_p1) << endl; // Print the values for debugging
    assert(b1 == des_b1);
    assert(p1 == des_p1);
    assert(p2 == des_p2);
    assert(p3 == des_p3);
    assert(p4 == des_p4);
    assert(i1 == des_i1);
    assert(i2 == des_i2);
    assert(i3 == des_i3);
    assert(i4 == des_i4);
    assert(f1 == des_f1);
    assert(f2 == des_f2);
    assert(b2 == des_b2);
    assert(s1 == des_s1);
}

void test_serde(){

    Buffer b;
    test_serde_primitives(b);
   cout<<"COMPLETED PRIMITIVES"<<endl;

    test_serde_vec_primitive(b);
   cout<<"completed serde vec metadata " << endl;

    //test case for serde vec metadata when the vec has a primitive
    test_serde_vec_metadata(b);
    cout << "completed serde vec metadata with internal type " << endl;
}

int main() {
    test_serde();
    cout<<"Completeed Test Serde "<< endl;

    return 0;
}