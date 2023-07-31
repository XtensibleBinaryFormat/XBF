#include "Buffer.hh"
using namespace std;

Buffer::Buffer(size_t initialSize, bool writing)
    : writing(writing), size(initialSize) {
  availSize = writing ? size : 0;
  preBuffer = new char[size + extra * 2];
  buffer = extra + preBuffer;
  p = buffer;
  memset(preBuffer, '\0', size + extra * 2);
}

void Buffer:: dump(ostream& s) const{
    // TODO: display all bytes in HEX, display the length, reset the buffer

    const int length = p-buffer;
    //display bytes in hex format
    s<<"Raw bytes: ";
    for(size_t i = 0; i < length; i++){   
        s << hex << setw(2)<< setfill('0') << static_cast<int>(buffer[i]) << " ";
    }
    s << endl;
    s << dec;
    //display the length
    s <<" Buffer length: "<< length << " bytes " <<endl;

  }