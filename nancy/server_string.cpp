#include <iostream>
#include <cstring>
#include <unistd.h>
#include <sys/socket.h>
#include <arpa/inet.h>

#define MAX_BUFFER_SIZE 1024

// XDL type
struct XDLType {
    char value[MAX_BUFFER_SIZE];
};

int main() {
    int serverSocket, clientSocket;
    struct sockaddr_in serverAddress, clientAddress;
    socklen_t clientAddressLength;
    
    // Create socket
    serverSocket = socket(AF_INET, SOCK_STREAM, 0);
    if (serverSocket == -1) {
        std::cerr << "Failed to create socket.\n";
        return -1;
    }
    
    // Set up server address
    serverAddress.sin_family = AF_INET;
    serverAddress.sin_port = htons(12345); // Server's port number
    serverAddress.sin_addr.s_addr = INADDR_ANY;
    
    // Bind the socket to the specified IP and port
    if (bind(serverSocket, (struct sockaddr *)&serverAddress, sizeof(serverAddress)) < 0) {
        std::cerr << "Binding failed.\n";
        return -1;
    }
    
    // Listen for incoming connections
    if (listen(serverSocket, 1) < 0) {
        std::cerr << "Listening failed.\n";
        return -1;
    }
    
    std::cout << "Server is listening for connections.\n";
    
    // Accept the incoming connection
    clientAddressLength = sizeof(clientAddress);
    clientSocket = accept(serverSocket, (struct sockaddr *)&clientAddress, &clientAddressLength);
    if (clientSocket < 0) {
        std::cerr << "Failed to accept the connection.\n";
        return -1;
    }
    
    std::cout << "Client connected.\n";
    
    // Send XDLType
    XDLType xdl;
    strcpy(xdl.value, "Hello from server!");
    if (send(clientSocket, &xdl, sizeof(xdl), 0) < 0) {
        std::cerr << "Failed to send XDLType.\n";
        return -1;
    }
    
    // Close the sockets
    close(clientSocket);
    close(serverSocket);
    
    return 0;
}