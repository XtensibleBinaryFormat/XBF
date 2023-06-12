#include <iostream>
#include <fstream>
#include <sstream>
#include <vector>

// Serialize data to binary format
void serializeData(const std::vector<int>& data, std::ostream& output) {
    int size = data.size();
    output.write(reinterpret_cast<const char*>(&size), sizeof(int));
    output.write(reinterpret_cast<const char*>(data.data()), size * sizeof(int));
}

// Deserialize binary data to vector
std::vector<int> deserializeData(std::istream& input) {
    int size;
    input.read(reinterpret_cast<char*>(&size), sizeof(int));
    
    std::vector<int> data(size);
    input.read(reinterpret_cast<char*>(data.data()), size * sizeof(int));
    
    return data;
}

int main() {
    // Create and populate data vector
    std::vector<int> originalData = {1, 2, 3, 4, 5};

    // Serialize data to binary format
    std::stringstream serializedData;
    serializeData(originalData, serializedData);

    // Transfer binary data over the network or store in a file
    std::string binaryData = serializedData.str();

    // Deserialize binary data back to vector
    std::stringstream deserializedData(binaryData);
    std::vector<int> receivedData = deserializeData(deserializedData);

    // Display the deserialized data
    std::cout << "Received Data:\n";
    for (int num : receivedData) {
        std::cout << num << " ";
    }
    std::cout << std::endl;

    return 0;
}