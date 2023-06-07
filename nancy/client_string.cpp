#include <iostream>
#include <cstring>
#include <unistd.h>
#include <sys/socket.h>
#include <arpa/inet.h>
#include <chrono>

using namespace std;

#define MAX_BUFFER_SIZE 1024

// XDL type
struct XDLType {
    char value[MAX_BUFFER_SIZE];
};

int main() {
    int clientSocket;
    struct sockaddr_in serverAddress;

     using SteadyTimePoint = chrono::time_point<chrono::steady_clock>;
    auto start_steady = SteadyTimePoint();
    auto end_steady = SteadyTimePoint();
    auto elapsed_millis = chrono::milliseconds();

    start_steady = chrono::steady_clock::now();


    // Create socket
    clientSocket = socket(AF_INET, SOCK_STREAM, 0);
    if (clientSocket == -1) {
        std::cerr << "Failed to create socket.\n";
        return -1;
    }
    
    // Set up server address
    serverAddress.sin_family = AF_INET;
    serverAddress.sin_port = htons(12345); // Server's port number
    if (inet_pton(AF_INET, "127.0.0.1", &serverAddress.sin_addr) <= 0) {
        std::cerr << "Invalid address/ Address not supported.\n";
        return -1;
    }
    
    // Connect to the server
    if (connect(clientSocket, (struct sockaddr *)&serverAddress, sizeof(serverAddress)) < 0) {
        std::cerr << "Connection failed.\n";
        return -1;
    }
    
    std::cout << "Connected to the server.\n";
    
    // Receive XDLType
    XDLType receivedXDL;
    if (recv(clientSocket, &receivedXDL, sizeof(receivedXDL), 0) < 0) {
        std::cerr << "Failed to receive XDLType.\n";
        return -1;
    }
    
    std::cout << "Received XDLType with value: " << receivedXDL.value << std::endl;
    
    end_steady = chrono::steady_clock::now();

    elapsed_millis = end_steady - start_steady;

    cout << elapsed_millis.count() << " milliseconds\n";

    // Close the socket
    close(clientSocket);
    
    return 0;
}