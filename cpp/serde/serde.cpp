#include <iostream>
#include <sstream>
#include <string>
#include <vector>
#include <map>
#include <iomanip>
#include "rust.hh"
#include "metadata.hh"
using namespace std;


void dump(const stringstream& ss){
    // TODO: display all bytes in HEX, display the length, reset the buffer
    stringstream ss_copy(ss.str()); //crete a non const copy of the stringstream

    const string& buffer = ss_copy.str();
    const size_t length = buffer.length();

    //display bytes in hex format
    cout<<"Buffer bytes in HEX: ";
    for(size_t i = 0; i < length; i++){
        cout<< hex << setw(2) << setfill('0') << static_cast<int>(buffer[i]);
    }
    cout<< endl;

    //display the length
    cout<<" Buffer length: "<< length << " bytes " <<endl;

    //reset the Buffer
    ss_copy.str(string());
    ss_copy.clear();
}

class vec3d{
  private:
        double x, y, z;
   public:
        vec3d(double x, double y, double z) : x(x), y(y), z(z) {}
         //TODO: make a function that generates metadata for this double
         void gen_metadata(boost::archive::text_oarchive& oa)const{
            oa << boost::serialization::make_nvp("x", x);
            oa << boost::serialization::make_nvp("y", y);
            oa << boost::serialization::make_nvp("z", z);
         }
    friend boost::archive::text_oarchive& operator <<(boost::archive::text_oarchive& oa, const vec3d& v){
        v.gen_metadata(oa);
        return oa;
    }
};

BOOST_SERIALIZATION_ASSUME_ABSTRACT(Metadata)

BOOST_CLASS_EXPORT(PrimitiveMetadata)
BOOST_CLASS_EXPORT(VecMetadata)
BOOST_CLASS_EXPORT(StructMetadata)


void test_serde_vec_primitive(){
    vector<bool> boolVec{ true, false, true };
    vector<u8> u8Vec{ 1, 2, 3 };
    vector<u16> u16Vec{ 1000, 2000, 3000 };
    vector<u32> u32Vec{ 1'000'000, 2'000'000, 3'000'000 };
    vector<u64> u64Vec{ 8'000'000'000ULL, 9'000'000'000ULL, 10'000'000'000ULL };
    // TODO: Add vectors for other primitive types

    // Serialization
    stringstream ss;
    boost::archive::text_oarchive oa(ss);
    oa << boolVec << u8Vec << u16Vec << u32Vec << u64Vec;
    // TODO: Serialize other vectors

    // Deserialization
    boost::archive::text_iarchive ia(ss);
    vector<bool> desBoolVec;
    vector<u8> desU8Vec;
    vector<u16> desU16Vec;
    vector<u32> desU32Vec;
    vector<u64> desU64Vec;
    //TODO: Add vectors for other primitive types
    ia >> desBoolVec >> desU8Vec >> desU16Vec >> desU32Vec >> desU64Vec;
    //TODO: Deserialize other vectors

    // Assertions
    assert(boolVec == desBoolVec);
    assert(u8Vec == desU8Vec);
    assert(u16Vec == desU16Vec);
    assert(u32Vec == desU32Vec);
    assert(u64Vec == desU64Vec);
    // TODO:Add assertions for other vectors    
}



void test_serde_primitives(){
    stringstream ss;
        boost::archive::text_oarchive oa(ss);
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
    oa << b1 << p1 << p2 << p3 << p4 << i1 << i2 << i3 << i4 << f1 << f2 << b2 << s1;
    //calling dump function
    dump(ss);
    //deserialisation
    boost::archive::text_iarchive ia(ss);
    bool des_b1;
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
    string des_s1;
    
    ia  >>  des_b1 >> des_p1 >> des_p2 >> des_p3 >> des_p4 >> des_i1 >> des_i2 >> des_i3 >> des_i4 >> des_f1>> des_f2 >> des_b2 >> des_s1;
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

int main() {
   
   test_serde_primitives();
   cout<<"COMPLETED SERDE"<<endl;

   //test case for serde vec metadata when the vec has a primitive
   test_serde_vec_primitive();
   cout<<"completed serde vec metadata " << endl;

    return 0;
}