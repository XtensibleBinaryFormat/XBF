#include <iostream>
#include <vector>
#include <string>
#include <cstring>
#include <type_traits>
#include <cstdint>

template <typename T>
std::vector<char> serializeData(const std::vector<T>& data) {
    static_assert(!std::is_same<T, bool>::value, "Serialization of bool not supported.");

    std::vector<char> serializedData;
    serializedData.reserve(data.size() * sizeof(T));

    const char* rawData = reinterpret_cast<const char*>(data.data());
    serializedData.insert(serializedData.end(), rawData, rawData + data.size() * sizeof(T));

    return serializedData;
}

template <>
std::vector<char> serializeData<bool>(const std::vector<bool>& data) {
    std::vector<char> serializedData;
    serializedData.reserve((data.size() + 7) / 8);  // Adjusted to handle any size

    char currentByte = 0;
    int bitCount = 0;

    for (const bool value : data) {
        currentByte |= (value ? (1 << bitCount) : 0);
        ++bitCount;

        if (bitCount == 8) {
            serializedData.push_back(currentByte);
            currentByte = 0;
            bitCount = 0;
        }
    }

    if (bitCount > 0) {
        serializedData.push_back(currentByte);
    }

    return serializedData;
}

template <typename T>
std::vector<T> deserializeData(const std::vector<char>& serializedData) {
    std::vector<T> data;
    data.reserve(serializedData.size() / sizeof(T));

    const T* rawData = reinterpret_cast<const T*>(serializedData.data());
    data.insert(data.end(), rawData, rawData + serializedData.size() / sizeof(T));

    return data;
}

template <>
std::vector<bool> deserializeData<bool>(const std::vector<char>& serializedData) {
    std::vector<bool> data;
    data.reserve(serializedData.size() * 8);

    for (const char byte : serializedData) {
        for (int i = 0; i < 8; ++i) {
            data.push_back(byte & (1 << i));
        }
    }

    return data;
}

int main() {
    // Sample data
    std::vector<bool> boolData = { true, false, true, true };
    std::vector<uint8_t> u8Data = { 0, 128, 255 };
    std::vector<uint16_t> u16Data = { 0, 32768, 65535 };
    std::vector<uint32_t> u32Data = { 0, 2147483648, 4294967295 };
    std::vector<uint64_t> u64Data = { 0, 9223372036854775808ULL, 18446744073709551615ULL };
    std::vector<int8_t> i8Data = { -128, 0, 127 };
    std::vector<int16_t> i16Data = { -32768, 0, 32767 };
    std::vector<int32_t> i32Data = { -2147483648, 0, 2147483647 };
    std::vector<int64_t> i64Data = { -9223372036854775807LL, 0, 9223372036854775807LL };
    std::vector<float> f32Data = { 0.0f, 3.14f, -1.23f };
    std::vector<double> f64Data = { 0.0, 3.14159, -1.23456 };
    std::vector<std::string> stringData = { "Hello", "World", "!", "こんにちは" , "This is done by Nancy","नैंसी", "نینسی", "ナンシー", "نانسی" };

    // Serialization
    std::vector<char> serializedBoolData = serializeData(boolData);
    std::vector<char> serializedU8Data = serializeData(u8Data);
    std::vector<char> serializedU16Data = serializeData(u16Data);
    std::vector<char> serializedU32Data = serializeData(u32Data);
    std::vector<char> serializedU64Data = serializeData(u64Data);
    std::vector<char> serializedI8Data = serializeData(i8Data);
    std::vector<char> serializedI16Data = serializeData(i16Data);
    std::vector<char> serializedI32Data = serializeData(i32Data);
    std::vector<char> serializedI64Data = serializeData(i64Data);
    std::vector<char> serializedF32Data = serializeData(f32Data);
    std::vector<char> serializedF64Data = serializeData(f64Data);
    std::vector<char> serializedStringData = serializeData(stringData);

    // Deserialization
    std::vector<bool> receivedBoolData = deserializeData<bool>(serializedBoolData);
    std::vector<uint8_t> receivedU8Data = deserializeData<uint8_t>(serializedU8Data);
    std::vector<uint16_t> receivedU16Data = deserializeData<uint16_t>(serializedU16Data);
    std::vector<uint32_t> receivedU32Data = deserializeData<uint32_t>(serializedU32Data);
    std::vector<uint64_t> receivedU64Data = deserializeData<uint64_t>(serializedU64Data);
    std::vector<int8_t> receivedI8Data = deserializeData<int8_t>(serializedI8Data);
    std::vector<int16_t> receivedI16Data = deserializeData<int16_t>(serializedI16Data);
    std::vector<int32_t> receivedI32Data = deserializeData<int32_t>(serializedI32Data);
    std::vector<int64_t> receivedI64Data = deserializeData<int64_t>(serializedI64Data);
    std::vector<float> receivedF32Data = deserializeData<float>(serializedF32Data);
    std::vector<double> receivedF64Data = deserializeData<double>(serializedF64Data);
    std::vector<std::string> receivedStringData = deserializeData<std::string>(serializedStringData);

    // Output
    std::cout << "Received bool data:\n";
    for (bool value : receivedBoolData) {
        std::cout << std::boolalpha << value << ' ';
    }
    std::cout << '\n';

    std::cout << "Received uint8_t data:\n";
    for (uint8_t value : receivedU8Data) {
        std::cout << static_cast<int>(value) << ' ';
    }
    std::cout << '\n';

    std::cout << "Received uint16_t data:\n";
    for (uint16_t value : receivedU16Data) {
        std::cout << value << ' ';
    }
    std::cout << '\n';

    std::cout << "Received uint32_t data:\n";
    for (uint32_t value : receivedU32Data) {
        std::cout << value << ' ';
    }
    std::cout << '\n';

    std::cout << "Received uint64_t data:\n";
    for (uint64_t value : receivedU64Data) {
        std::cout << value << ' ';
    }
    std::cout << '\n';

    std::cout << "Received int8_t data:\n";
    for (int8_t value : receivedI8Data) {
        std::cout << static_cast<int>(value) << ' ';
    }
    std::cout << '\n';

    std::cout << "Received int16_t data:\n";
    for (int16_t value : receivedI16Data) {
        std::cout << value << ' ';
    }
    std::cout << '\n';

    std::cout << "Received int32_t data:\n";
    for (int32_t value : receivedI32Data) {
        std::cout << value << ' ';
    }
    std::cout << '\n';

    std::cout << "Received int64_t data:\n";
    for (int64_t value : receivedI64Data) {
        std::cout << value << ' ';
    }
    std::cout << '\n';

    std::cout << "Received float data:\n";
    for (float value : receivedF32Data) {
        std::cout << value << ' ';
    }
    std::cout << '\n';

    std::cout << "Received double data:\n";
    for (double value : receivedF64Data) {
        std::cout << value << ' ';
    }
    std::cout << '\n';

    std::cout << "Received string data:\n";
    for (const std::string& value : receivedStringData) {
        std::cout << value << ' ';
    }
    std::cout << '\n';

    return 0;
}