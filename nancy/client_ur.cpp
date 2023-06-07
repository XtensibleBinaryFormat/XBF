#include <iostream>
#include <string>
#include <boost/asio.hpp>

using namespace boost::asio;

int main() {
    io_context ioContext;

    try {
        // Create a socket and connect to the server
        ip::tcp::socket socket(ioContext);
        ip::tcp::resolver resolver(ioContext);
        connect(socket, resolver.resolve("127.0.0.1", "12345"));

        std::cout << "Connected to the server.\n";

        // Send the URL to the server
        std::string url = "https://www.youtube.com/";
        write(socket, buffer(url));

        // Receive the metadata from the server
        std::string metadata(4096, '\0');
        std::size_t bytesRead = socket.read_some(buffer(metadata));

        // Trim the null characters from the metadata
        metadata.erase(metadata.find('\0'));

        // Print the received metadata
        std::cout << "Received metadata:\n" << metadata << std::endl;

        // Close the socket
        socket.close();
    } catch (const boost::system::system_error& ex) {
        std::cerr << "Error: " << ex.what() << std::endl;
        return -1;
    }

    return 0;
}