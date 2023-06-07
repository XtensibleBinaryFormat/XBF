#include <iostream>
#include <string>
#include <boost/asio.hpp>
#include <cpprest/http_client.h>
#include <cpprest/filestream.h>

using namespace boost::asio;

std::string getMetadata(const std::string& url) {
    // Perform an HTTP request to retrieve metadata from the given URL
    web::http::client::http_client client(utility::conversions::to_string_t(url));
    web::http::http_request request(web::http::methods::GET);

    web::http::http_response response = client.request(request).get();

    // Check if the request was successful
    if (response.status_code() == web::http::status_codes::OK) {
        // Extract metadata from the response, parse it if needed
        web::json::value jsonResponse = response.extract_json().get();
        return jsonResponse.serialize();
    }

    return "Error retrieving metadata from the URL.";
}

int main() {
    io_context ioContext;

    // Create an acceptor to listen for incoming connections
    ip::tcp::acceptor acceptor(ioContext, ip::tcp::endpoint(ip::tcp::v4(), 12345));

    try {
        std::cout << "Server started. Waiting for client connection...\n";

        // Wait for a client to connect
        ip::tcp::socket socket(ioContext);
        acceptor.accept(socket);

        std::cout << "Client connected.\n";

        // Read the URL sent by the client
        std::string url(256, '\0');
        socket.read_some(buffer(url));

        // Trim the null characters from the URL
        url.erase(url.find('\0'));

        // Get the metadata associated with the URL
        std::string metadata = getMetadata(url);

        // Send the metadata to the client
        write(socket, buffer(metadata));

        std::cout << "Metadata sent to the client.\n";

        // Close the socket
        socket.close();
    } catch (const boost::system::system_error& ex) {
        std::cerr << "Error: " << ex.what() << std::endl;
        return -1;
    }

    return 0;
}