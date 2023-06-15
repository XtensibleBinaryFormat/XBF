# XDL Specification

## Types

### 16 Primitives

- Boolean
- U8, U16, U32, U64, U128, U256
- I8, I16, I32, I64, I128, I256
- F32, F64
- String (UTF-8)

### Vector

A homogenous list of values that has a known length.

### Struct

An aggregate type containing a name as well as named fields.

### Proposed Additional Inclusions

This list can and should be expanded based on any new ideas we have. Possible
candidates for inclusion include:

#### Bytes

As a counterpart to String, Bytes will not be guaranteed to be valid UTF-8, and
are simply a list of bytes, analogous to a `Vec<u8>` in Rust.

This could possibly be easier to use than the built-in XDL list type depending
on how it is implemented? It's also something that likely comes up very often
when sending data, so direct support for it may make things easier.

#### Char

This is not the same as a U8. Taking inspiration from Rust's `char` type, it's a
single Unicode Scalar Value:
<https://www.unicode.org/glossary/#unicode_scalar_value>.

Whether this is better than just sending a String of length one is to be
determined.

#### Optional Types

It may be useful to add support for a field or type that may specifically
contain nothing. This avoids the need to have a Void or Null type, and encodes
directly in the type system when something may not be present. This seems most
useful as a field within a Struct or an element in a Vector. Optional types are
something that takes inspiration from Rust's type system, but it is also present
within C++ as std::optional.

## Direct Representations

### Boolean

A boolean should be sent as an unsigned 8-bit integer with the value 0 or 1.
However, should an arbitrary U8 be sent a value of 0 should be taken to be
false, and any other value be taken to be true.

REQUEST FOR REVIEW: Should we allow this coercion behavior from a U8 to a
Boolean, or should exclusively 0 and 1 values be allowed, with an exception /
panic occurring if neither of those values is found?

### Integers

All integers should be sent in little endian format, with the least significant
bit first.

For signed integers, they should be represented in two's complement format.

### Floating Point Numbers

All floating point numbers should be sent as 32 or 64 bit IEEE 754 floating
point numbers.

### Strings

Strings should be sent as UTF-8 encoded characters. They should first send their
length as an unsigned 16-bit integer, followed by the corresponding number of
characters specified by their length.

REQUEST FOR REVIEW: Should the chosen length type be changed from a 16-bit
integer to something else? The most common platforms are using 32-bit integers
as the default "int" type, so should we choose to conform to that?

### Vector

Vectors should first include their length as an unsigned 16-bit integer,
followed by the corresponding number of elements. The type contained within a
Vector is **not** sent to the client. That information is carried in the
metadata for a particular request, which will be explained more later in this
specification.

### Struct

UNDER CONSTRUCTION

The exact representation for Structs has not been finalized yet, and we will
likely hold off on finishing it until the Metadata Specification is finished.

## Metadata Specification

### Primitives

For primitives, the metadata should be sent as a single byte discriminant value.
After receiving one of these discriminant values, the client should read the
following byte(s) and interpret them as the type given by the discriminant.

Here is the current list of primitives and their expected discriminant values:

| Primitive | Discriminant |
| --------- | ------------ |
| Boolean   | 0            |
| U8        | 1            |
| U16       | 2            |
| U32       | 3            |
| U64       | 4            |
| U128      | 5            |
| U256      | 6            |
| I8        | 7            |
| I16       | 8            |
| I32       | 9            |
| I64       | 10           |
| I128      | 11           |
| I256      | 12           |
| F32       | 13           |
| F64       | 14           |
| String    | 15           |

REQUEST FOR REVIEW:

Should we leave room between some of these discriminant numbers? For example,
leave some space between U256 and I8? That way in the future if there's ever a
need to expand new values can go in a place that logically makes sense, instead
of being tacked on at the end.

### Vector

For a vector, a discriminant value should first be sent, similarly to primitives
(following the same size requirement). Following this, the client should expect
to receive metadata information for the internal type contained within the
Vector. This process may continue recursively with nested types of Vectors and
Structs. Once all the metadata is received, the vector will send its length and
data as expected.

### Struct

UNDER CONSTRUCTION
