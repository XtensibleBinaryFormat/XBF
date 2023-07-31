#pragma once

#include <fcntl.h>
#include <unistd.h>

#include <cstddef>
#include <cstdint>
#include <cstring>
#include <fstream>
#include <iostream>
#include <regex>
#include <string>
#include <vector>
#include <iomanip>

#include "metadata.hh"
#include "rust.hh"



class XDLRaw;

class Buffer {
 public:
  Buffer(size_t initialSize = 32768, bool writing = true);
  Buffer(const char filename[], size_t initialSize);
  Buffer(const char filename[], size_t initialSize, const char*);
  Buffer(const Buffer& c) = delete;
  void operator=(const Buffer& orig) = delete;
  ~Buffer() {
    if (writing) {  //TODO: && !isSockBuf 
      flush();
    }
    delete[] preBuffer;  
  }

  void dump(std::ostream& s) const;

  void flush() {  // TODO: this will fail if we overflow slightly
    uint32_t writeSize = (p - buffer >= size) ? size : (p - buffer);
    /*if (isSockBuf)
      SocketIO::send(fd, buffer, writeSize, 0);
    else {
      if (::write(fd, buffer, writeSize) < 0) throw Ex1(Errcode::FILE_WRITE);
    }*///TODO: write to boost socket(more portable)
    if (writeSize > 0) {
    #if 0 
    try {
      boost::asio::write(socket, boost::asio::buffer(buffer, writeSize));
    } catch (const boost::system::system_error& error) {
      // Handle the error accordingly
      cerr << "Error occurred during write: " << error.what() << endl;
      throw; // Re-throw the exception if desired or perform alternative error handling
    }
    #endif
  }
    p = buffer;
    availSize = size;
  }

  //TODO: void readNext();
  // write is binary

  void reset_pointer_to_buffer(){
    p = buffer;
  }
  
  void unchecked_write(const char* s, u16 len){
    checkSpace(len+2);
    *(u16*)p = len;
    p += 2;
    memcpy(p, s, len);
    p += len;
    availSize -= len+2;
  }

// Serialize a vector of any primitive data type
template <typename T>
void unchecked_write_vector(const std::vector<T>& v, Buffer& b) {
    size_t len = v.size();
    b.unchecked_write(static_cast<uint16_t>(len));
    for (const auto& elem : v) {
        b.unchecked_write(elem);
    }
}

// Serialize a vector<bool>
template <typename T = bool>
typename std::enable_if<std::is_same<T, bool>::value>::type
unchecked_write_vector(const std::vector<bool>& v, Buffer& b) {
    size_t len = v.size();
    b.unchecked_write(static_cast<uint16_t>(len));
    for (const bool& value : v) {
        b.unchecked_write(value);
    }
}

// Deserialize a vector of any primitive data type
template <typename T>
std::vector<T> unchecked_read_vector(Buffer& b) {
    std::vector<T> v;
    size_t len = b.readU16();
    v.reserve(len);
    for (size_t i = 0; i < len; ++i) {
        v.push_back(b.unchecked_read<T>());
    }
    return v;
}

// Deserialize a vector<bool>
template <typename T = bool>
typename std::enable_if<std::is_same<T, bool>::value, std::vector<bool>>::type
unchecked_read_vector(Buffer& b) {
    std::vector<bool> v;
    size_t len = b.readU16();
    v.reserve(len);
    for (size_t i = 0; i < len; ++i) {
        v.push_back(b.readU8() != 0);
    }
    return v;
}

  void unchecked_write(const std::string& s){
      unchecked_write(s.c_str(), s.length());
  }

  #if 0 
  void writeStructMeta(const char name[], uint32_t numMembers){
    write(PrimitiveType::Struct);
    write(name, strlen(name));
    write(numMembers);
  }
  #endif

   std::string readString(){
      size_t strLength = readU16();
      checkAvailableRead(strLength);
      std::string str(p, strLength);
      p += strLength;
      availSize -= strLength;
      return str;
    }

  /*
    Write out a data type as a single byte
   */
  void unchecked_write(PrimitiveType t) { 
    unchecked_write(uint8_t(t)); 
    }
  PrimitiveType readType() { 
    return PrimitiveType(*p++); 
    }

  void unchecked_write(PrimitiveType t, const char* name) {
    unchecked_write(t);
    unchecked_write(name, strlen(name));
  }

  /**
   * write is the fast write that does not check for buffer overrun.
   * Use only when checking size of a large block
   *
   * @tparam T the tpe of the integer to write
   * @param v the value
   */
  //************ uint8_t uint16_t uint32_t uint64_t *************//
  template <typename T>
  void unchecked_write(T v) {
    *(T*)p = v;
    p = p + sizeof(T);
    availSize -= sizeof(T);
  }


  // for writing big objects, don't copy into the buffer, write it to the socket
  // directly
  //TODO: this is not efficient, improve it!
  void specialWrite(const char* buf, const uint32_t len) {
    flush();
    #if 0
    //TODO: convert to boost ::write(fd, buf, len);
    boost::asio::write(socket, boost::asio::buffer(buf, len));
    #endif
  }

  //TODO: implement as a vector
  #if 0 
  template <typename T>
  void writeList(List1<T>& list) {
    checkSpace(list.serializeSize());
    list.write(p);
    p += list.serializeSize();
    availSize -= list.serializeSize();
  }
  #endif


  //************ uint8_t uint16_t uint32_t uint64_t array *************//
  /*
   if there is not enough space to write this data without overflowing the
   buffer overflow region, then flush.  This assumes that the object being
   written is small enough to fit into the buffer at all
   */
  void checkSpace(size_t sz) {
    if (p + sz > buffer + size + extra) {  // p>buffer+size
      flush();
    }
  }

  //************ uint8_t uint16_t uint32_t uint64_t array *************//
  /*
    The fastest way to write 32k at a time is to write each object into
                the buffer as long as it is less than the overflow size.
                Then, after writing, if you have filled the buffer, flush
                and move the remaining bytes to the beginning of the buffer and
    start over.
   */
  void fastCheckSpace(size_t sz) {
    if (p > buffer + size) {  // p>buffer+size
      uint32_t beyondEnd = p - (buffer + size);
      flush();
      memcpy(buffer, buffer + size, beyondEnd);
      p += beyondEnd;
      availSize -= beyondEnd;
    }
  }

  //*********************************//
  //************ uint8_t uint16_t uint32_t uint64_t array *************//

  template <typename T>
  void checkArraySpace(T v[], size_t n) {
    size_t dataSize = n * sizeof(T);
    if (size < dataSize) {
      // TODO: buffer is not big enough to copy data, write directly
      specialWrite(reinterpret_cast<const char*>(v), static_cast<uint32_t>(dataSize));
      return;
    }
    size_t remainingSize = size - (p - buffer);
    // TODO: efficiency, and for big arrays if (n > ???)
    if (remainingSize < dataSize) {
      flush();
    }
  }
  //*********************************//
  //************ uint8_t uint16_t uint32_t uint64_t vector *************//
  template <typename T>
  void checkVectorSpace(const std::vector<T>& v) {
    size_t dataSize = v.size() * sizeof(T);
    if (size < dataSize) {
      // TODO: buffer is not big enough to copy data, write directly
      specialWrite(reinterpret_cast<const char*>(v.data()), static_cast<uint32_t>(dataSize));
      return;
    }
    if (availSize < dataSize){
        flush();
    } 
  }

  //*********************************//
  //************ uint8_t uint16_t uint32_t uint64_t operator *************//
  template <typename T>
  Buffer& operator<<(T v) {  // there is a write in flush function
    checkSpace(sizeof(T));
    unchecked_write(v);
    return *this;
  }
 

  /*
   TODO: For writing large arrays, it would be more efficient to flush the
   buffer, then write directly from the arrays.
*/

  int8_t unchecked_readI8() {
    int8_t temp = *(int8_t*)p;
    p += sizeof(int8_t);
    availSize -= sizeof(int8_t);
    return temp;
  }

  int16_t unchecked_readI16() {
    int16_t temp = *(int16_t*)p;
    p += sizeof(int16_t);
    availSize -= sizeof(int16_t);
    return temp;
  }

  int32_t unchecked_readI32() {
    int32_t temp = *(int32_t*)p;
    p += sizeof(int32_t);
    availSize -= sizeof(int32_t);
    return temp;
  }

  int64_t unchecked_readI64() {
    int64_t temp = *(int64_t*)p;
    p += sizeof(int64_t);
    availSize -= sizeof(int64_t);
    return temp;
  }

  int8_t readI8() {
    checkAvailableRead(sizeof(int8_t));
    return unchecked_readI8();
  }

  int16_t readI16() {
    checkAvailableRead(sizeof(int16_t));
    return unchecked_readI16();
  }

  int32_t readI32() {
    checkAvailableRead(sizeof(int32_t));
    return unchecked_readI32();
  }

  int64_t readI64() {
    checkAvailableRead(sizeof(int64_t));
    return unchecked_readI64();
  }

  uint8_t unchecked_readU8() {
    uint8_t temp = *(uint8_t*)p;
    p += sizeof(uint8_t);
    availSize -= sizeof(uint8_t);
    return temp;
  }

  uint16_t unchecked_readU16() {
    uint16_t temp = *(uint16_t*)p;
    p += sizeof(uint16_t);
    availSize -= sizeof(uint16_t);
    return temp;
  }

  uint32_t unchecked_readU32() {
    uint32_t temp = *(uint32_t*)p;
    p += sizeof(uint32_t);
    availSize -= sizeof(uint32_t);
    return temp;
  }

  uint64_t unchecked_readU64() {
    uint64_t temp = *(uint64_t*)p;
    p += sizeof(uint64_t);
    availSize -= sizeof(uint64_t);
    return temp;
  }

  float unchecked_readF32() {
    float temp = *(float*)p;
    p += sizeof(float);
    availSize -= sizeof(float);
    return temp;
  }

  double unchecked_readF64() {
    double temp = *(double*)p;
    p += sizeof(double);
    availSize -= sizeof(double);
    return temp;
  }

  uint8_t readU8() {
    checkAvailableRead(sizeof(uint8_t));
    return unchecked_readU8();
  }

  uint16_t readU16() {
    checkAvailableRead(sizeof(uint16_t));
    return unchecked_readU16();
  }

  uint32_t readU32() {
    checkAvailableRead(sizeof(uint32_t));
    return unchecked_readU32();
  }

  uint64_t readU64() {
    checkAvailableRead(sizeof(uint64_t));
    return unchecked_readU64();
  }

  float readF32() {
    checkAvailableRead(sizeof(float));
    return unchecked_readF32();
  }

  double readF64() {
    checkAvailableRead(sizeof(double));
    return unchecked_readF64();
  }

  template <typename T>
  T unchecked_read() {
    T temp = *(T*)p;
    p += sizeof(T);
    availSize -= sizeof(T);
    return temp;
  }

  
  
    template <typename T>
  Buffer& operator>>(T& v) {  // there is a write in flush function
    checkSpace(sizeof(T));
    v = unchecked_read<T>();
    return *this;
  }

//TODO: test it!
//unchecked_read<u16>()

 private:
  bool writing;
  size_t size;
  const size_t extra = 128;
  char* preBuffer;
  char* buffer;       // pointer to the buffer
  int32_t availSize;  // how much space is left in the buffer
  char* p;            // cursor to current byte for reading/writing
  uint32_t blockSize;  // Max block size for output
  void checkAvailableRead(size_t sz) {
    if (availSize < sz) {
      size_t overflowSize = availSize;
      memcpy(buffer - overflowSize, p, overflowSize);
      //readNext();
      availSize += overflowSize;
      p = buffer - overflowSize;
    }
  }

 public:
  void checkAvailableWrite() {
    if (p > buffer + size) {
      uint32_t overflow = p - (buffer + size);
      flush();
      memcpy(p, p + size, overflow);
      p += overflow;
      availSize -= overflow;
    }
  }
#if 0
 private:
  void checkAvailableWrite(const char* ptr, uint32_t len) {
    if (p + len > buffer + size) {
      memcpy(p, ptr, availSize);
      uint32_t remaining = len - availSize;
      flush();
      // TODO: Check if the string is too big!
      memcpy(p, ptr + size, remaining);
      p += remaining;
      availSize -= remaining;
      return;
    }
    // TODO: check case where string is bigger than buffer
    if (len > size) {
      flush();
      // TODO: Do something completely different
      ::write(fd, ptr, len);
      return;
    }
    memcpy(p, ptr, len);
    p += len;
    availSize -= len;
  }
#endif
};