#include <iostream>
#include <vector>
#include <string>
#include <sstream>
#include <algorithm>
#include <cstdint>

char boolToChar(bool value) {
    return value ? 1 : 0;
}

bool charToBool(char value) {
    return value != 0;
}

template <typename T>
void serializeData(const std::vector<T>& data, std::ostream& output) {
    size_t size = data.size();
    output.write(reinterpret_cast<const char*>(&size), sizeof(size));

    if constexpr (std::is_same_v<T, bool>) {
        std::vector<char> boolData(size);
        std::transform(data.begin(), data.end(), boolData.begin(), boolToChar);
        output.write(boolData.data(), boolData.size());
    } else {
        output.write(reinterpret_cast<const char*>(data.data()), data.size() * sizeof(T));
    }
}

template <typename T>
std::vector<T> deserializeData(std::istream& input) {
    size_t size;
    input.read(reinterpret_cast<char*>(&size), sizeof(size));

    std::vector<T> data(size);

    if constexpr (std::is_same_v<T, bool>) {
        std::vector<char> boolData(size);
        input.read(boolData.data(), boolData.size());
        std::transform(boolData.begin(), boolData.end(), data.begin(), charToBool);
    } else {
        input.read(reinterpret_cast<char*>(data.data()), data.size() * sizeof(T));
    }

    return data;
}

int main() {
    std::vector<bool> boolData = {true, false, true, true};
    std::vector<uint8_t> u8Data = {1, 0, 1, 1};
    std::vector<uint16_t> u16Data = {1, 257, 4, 0};
    std::vector<uint32_t> u32Data = {16842753, 4, 0, 673059850};
    std::vector<uint64_t> u64Data = {17196711937, 2890770044000665600, 4, 112591279187558500};
    std::vector<float> f32Data = {2.36936e-38f, 5.60519e-45f, 0.0f, 8.77511e-15f};
    std::vector<double> f64Data = {8.4963e-314, 1.90842e-115, 1.97626e-323, 3.73412e-301};
    std::vector<std::string> stringData = {"Hello", "World", "C++", "Serialization"};

    std::stringstream serializedData;

    serializeData(boolData, serializedData);
    serializeData(u8Data, serializedData);
    serializeData(u16Data, serializedData);
    serializeData(u32Data, serializedData);
    serializeData(u64Data, serializedData);
    serializeData(f32Data, serializedData);
    serializeData(f64Data, serializedData);
    serializeData(stringData, serializedData);

    std::stringstream deserializedData(serializedData.str());

    std::vector<bool> receivedBoolData = deserializeData<bool>(deserializedData);
    std::vector<uint8_t> receivedU8Data = deserializeData<uint8_t>(deserializedData);
    std::vector<uint16_t> receivedU16Data = deserializeData<uint16_t>(deserializedData);
    std::vector<uint32_t> receivedU32Data = deserializeData<uint32_t>(deserializedData);
    std::vector<uint64_t> receivedU64Data = deserializeData<uint64_t>(deserializedData);
    std::vector<float> receivedF32Data = deserializeData<float>(deserializedData);
    std::vector<double> receivedF64Data = deserializeData<double>(deserializedData);
    std::vector<std::string> receivedStringData = deserializeData<std::string>(deserializedData);

    // Output the deserialized data
    for (bool value : receivedBoolData) {
        std::cout << value << ' ';
    }
    std::cout << '\n';

    for (uint8_t value : receivedU8Data) {
        std::cout << static_cast<int>(value) << ' ';
    }
    std::cout << '\n';

    for (uint16_t value : receivedU16Data) {
        std::cout << value << ' ';
    }
    std::cout << '\n';

    for (uint32_t value : receivedU32Data) {
        std::cout << value << ' ';
    }
    std::cout << '\n';

    for (uint64_t value : receivedU64Data) {
        std::cout << value << ' ';
    }
    std::cout << '\n';

    for (float value : receivedF32Data) {
        std::cout << value << ' ';
    }
    std::cout << '\n';

    for (double value : receivedF64Data) {
        std::cout << value << ' ';
    }
    std::cout << '\n';

    for (const std::string& value : receivedStringData) {
        std::cout << value << ' ';
    }
    std::cout << '\n';

    return 0;
}