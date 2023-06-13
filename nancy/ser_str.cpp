#include <iostream>
#include <vector>
#include <string>

// Serialize data into a binary vector
template<typename T>
std::vector<char> serializeData(const std::vector<T>& data) {
    const char* rawData = reinterpret_cast<const char*>(data.data());
    size_t dataSize = data.size() * sizeof(T);
    return std::vector<char>(rawData, rawData + dataSize);
}

// Deserialize binary data into a vector of type T
template<typename T>
std::vector<T> deserializeData(const std::vector<char>& serializedData) {
    const T* dataPtr = reinterpret_cast<const T*>(serializedData.data());
    size_t dataSize = serializedData.size() / sizeof(T);
    return std::vector<T>(dataPtr, dataPtr + dataSize);
}

int main() {
    // Sample data
    std::vector<int> intData = { 1, 2, 3, 4, 5 };
    std::vector<double> doubleData = { 1.23, 4.56, 7.89 };
    std::vector<std::string> stringData = { "Hello", "World", "こんにちは" };

    // Serialize the data
    std::vector<char> serializedIntData = serializeData(intData);
    std::vector<char> serializedDoubleData = serializeData(doubleData);
    std::vector<char> serializedStringData = serializeData(stringData);

    // Transfer the serialized data over the network (simulated by copying vectors)

    // Deserialize the received data
    std::vector<int> receivedIntData = deserializeData<int>(serializedIntData);
    std::vector<double> receivedDoubleData = deserializeData<double>(serializedDoubleData);
    std::vector<std::string> receivedStringData = deserializeData<std::string>(serializedStringData);

    // Print the received data
    std::cout << "Received int data:\n";
    for (int value : receivedIntData) {
        std::cout << value << ' ';
    }
    std::cout << '\n';

    std::cout << "Received double data:\n";
    for (double value : receivedDoubleData) {
        std::cout << value << ' ';
    }
    std::cout << '\n';

    std::cout << "Received string data:\n";
    for (const std::string& value : receivedStringData) {
        std::cout << value << '\n';
    }

    return 0;
}