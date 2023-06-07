#include <iostream>
#include <vector>
#include <boost/asio.hpp>

using namespace boost::asio;

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

        // Send the metadata to the client
        std::vector<std::string> metadata = {
            "Boolean: true",
            "U8: 255",
            "U16: 65535",
            "U32: 4294967295",
            "U64: 18446744073709551615",
            "U128: 340282366920938463463374607431768211455",
            "U256: 115792089237316195423570985008687907853269984665640564039457584007913129639935",
            "I8: -128",
            "I16: -32768",
            "I32: -2147483648",
            "I64: -9223372036854775808",
            "I128: -170141183460469231731687303715884105728",
            "I256: -57896044618658097711785492504343953926634992332820282019728792003956564819968",
            "F32: 3.14",
            "F64: 3.141592653589793",
            "String (UTF-8): Hello, world!",
            "Vector (heterogeneous list of values with known length): [1, true, 3.14, \"example\"]"
            // Add more metadata as needed
        };

        for (const auto& item : metadata) {
            boost::asio::write(socket, boost::asio::buffer(item + "\n"));
        }

        std::cout << "Metadata sent to the client.\n";

        // Close the socket
        socket.close();
    } catch (const boost::system::system_error& ex) {
        std::cerr << "Error: " << ex.what() << std::endl;
        return -1;
    }

    return 0;
}