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
        /*I8,
        I16,
        I32,
        I64,
        I128,
        I256,
        F32,
        F64,
        Bytes,
        String*/
    oa << b1 << p1 << p2 << p3 << p4;
    dump(ss);
    boost::archive::text_iarchive ia(ss);
    bool des_b1;
    u8 des_p1;
    u16 des_p2;
    u32 des_p3;
    u64 des_p4;
    ia  >>  des_b1 >> des_p1 >> des_p2 >>des_p3 >> des_p4;
    assert(b1 == des_b1);
    assert(p1 == des_p1);
    assert(p2 == des_p2);
    assert(p3 == des_p3);
    assert(p4 == des_p4);

}

int main() {
   
   test_serde_primitives();
   cout<<"COMPLETED SERDE"<<endl;

    return 0;
}