#include <iostream>
#include <vector>
#include <boost/asio.hpp>

using namespace boost::asio;

int main() {
    io_context ioContext;

    // Create socket
    ip::tcp::socket clientSocket(ioContext);

    try {
        // Resolve the server address
        ip::tcp::resolver resolver(ioContext);
        auto endpoints = resolver.resolve("127.0.0.1", "12345");

        // Connect to the server
        boost::asio::connect(clientSocket, endpoints);

        std::cout << "Connected to the server.\n";

        // Receive the metadata from the server
        std::vector<char> buffer(1024);  // Buffer to hold received data
        boost::system::error_code ec;
        std::size_t bytes_transferred;

        do {
            bytes_transferred = clientSocket.read_some(boost::asio::buffer(buffer), ec);
            if (!ec) {
                std::string line(buffer.begin(), buffer.begin() + bytes_transferred);
                std::cout << "Received metadata: " << line << std::endl;
            }
        } while (bytes_transferred > 0);

        // Close the socket
        clientSocket.close();
    } catch (const boost::system::system_error& ex) {
        std::cerr << "Error: " << ex.what() << std::endl;
        return -1;
    }

    return 0;
}