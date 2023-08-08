# Comparison of XBF With Other Self Describing Formats

## Contending Formats

### CSV

Format specification: <https://www.rfc-editor.org/rfc/rfc4180>

Parser implementation: <https://github.com/BurntSushi/rust-csv>

A plain-text file that contains a header specifying the name of each column, followed by rows of data containing values separated by a delimited (usually ",") corresponding to each column.

### MessagePack

Format specification: <https://github.com/msgpack/msgpack/blob/master/spec.md>

Parser implementation: <https://github.com/3Hren/msgpack-rust>

An efficient binary format similar to JSON.

### CBOR

Format specification: <http://cbor.io/spec.html>

Parser implementation: <https://github.com/enarx/ciborium>

To quote their website: "[CBOR] is a data format whose design goals include the possibility of extremely small code size, fairly small message size, and extensibility without the need for version negotiation".

### JSON

### XML

### XBF

## Test Methodology

The goal of this test is to measure the how many bytes a particular format requires in order to transmit a given set of data, as well as roughly how long that takes for a round

The dataset is one year of Sony stock history downloaded from Yahoo Finance, in CSV format.

On the client side, the client measures the time it takes to open a connection to the server, send a request of what format it would like back, wait for the data to be sent back, and count the number of bytes received. This loop is performed 100 times and then the average of the times is taken.

On the server side, the server downloads the stock data from Yahoo and parses it into a vector of Rust native structs using the previously mentioned CSV parser implementation and the Serde crate. It then waits for a connection, and depending on the request type received serializes the native list into the requested format and sends it over the wire. This serialization result is not cached, and is performed every time a given format is asked for to ensure that parser performance is included in the measured round trip time.

## Results

Original stock CSV data size: 17160

Native data size: 14558

| Format      | Avg Time (ms) | Bytes Read |
| ----------- | ------------- | ---------- |
| CSV         | 18.931802     | 16411      |
| MessagePack | 11.220957     | 15565      |
| CBOR        | 16.957873     | 25507      |
| JSON        | 21.912745     | 31180      |
| XML         | 21.873043     | 43699      |
| XBF         | 11.322245     | 14686      |
