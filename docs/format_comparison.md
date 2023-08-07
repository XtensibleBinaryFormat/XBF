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

## Tests

### Stock Data

### Random Person Data

## Results

### Stock Data

### Random Person Data

## plaintext output data August 6th, 20:31

original stock csv data size: 17160
original random person data size: 13684

Request Type: Stock
Data Format: Csv
Avg Time: 22.826262ms
Bytes Read: Some(19090)

Request Type: Stock
Data Format: MessagePack
Avg Time: 17.011247ms
Bytes Read: Some(11549)

Request Type: Stock
Data Format: Cbor
Avg Time: 21.628923ms
Bytes Read: Some(18479)

Request Type: Stock
Data Format: Json
Avg Time: 31.645693ms
Bytes Read: Some(22103)

Request Type: Stock
Data Format: Xml
Avg Time: 27.781995ms
Bytes Read: Some(32112)

Request Type: Stock
Data Format: Xbf
Avg Time: 16.833106ms
Bytes Read: Some(10140)

Request Type: Person
Data Format: Csv
Avg Time: 22.642948ms
Bytes Read: Some(24254)

Request Type: Person
Data Format: MessagePack
Avg Time: 17.399429ms
Bytes Read: Some(15187)

Request Type: Person
Data Format: Cbor
Avg Time: 22.86973ms
Bytes Read: Some(24089)

Request Type: Person
Data Format: Json
Avg Time: 22.468048ms
Bytes Read: Some(30255)

Request Type: Person
Data Format: Xml
Avg Time: 28.264575ms
Bytes Read: Some(42309)

Request Type: Person
Data Format: Xbf
Avg Time: 22.101917ms
Bytes Read: Some(21751)
