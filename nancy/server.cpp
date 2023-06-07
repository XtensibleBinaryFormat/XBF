#include <arpa/inet.h>
#include <iostream>
#include <netinet/in.h>
#include <sys/socket.h>
#include <unistd.h>

// XDL type
struct XDLType {
  int value;
};

int main() {
  int serverSocket, clientSocket;
  struct sockaddr_in serverAddress, clientAddress;
  socklen_t clientAddressLength = sizeof(clientAddress);

  // Create socket
  serverSocket = socket(AF_INET, SOCK_STREAM, 0);
  if (serverSocket == -1) {
    std::cerr << "Failed to create socket.\n";
    return -1;
  }

  // Set up server address
  serverAddress.sin_family = AF_INET;
  serverAddress.sin_addr.s_addr = INADDR_ANY;
  serverAddress.sin_port = htons(12345); // Port number

  // Bind the socket to the specified address and port
  if (bind(serverSocket, (struct sockaddr *)&serverAddress,
           sizeof(serverAddress)) < 0) {
    std::cerr << "Failed to bind the socket.\n";
    return -1;
  }

  // Listen for incoming connections
  listen(serverSocket, 3);
  std::cout << "Waiting for incoming connections...\n";

  // Accept a connection from a client
  clientSocket = accept(serverSocket, (struct sockaddr *)&clientAddress,
                        &clientAddressLength);
  if (clientSocket < 0) {
    std::cerr << "Failed to accept the connection.\n";
    return -1;
  }
  std::cout << "Connection accepted.\n";

  // Send XDLType
  XDLType xdl;
  xdl.value = 42;

  if (send(clientSocket, &xdl, sizeof(xdl), 0) < 0) {
    std::cerr << "Failed to send XDLType.\n";
    return -1;
  }

  std::cout << "XDLType sent.\n";

  // Close the sockets
  close(clientSocket);
  close(serverSocket);

  return 0;
}
