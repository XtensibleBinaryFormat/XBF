# XDL Specification

## Types

### 12 Primitives

- Boolean
- U8, U16, U32, U64
- I8, I16, I32, I64
- F32, F64
- String (UTF-8)

### Vector

A heterogeneous list of values that has a known length.

### Struct

An aggregate type containing a name as well as named fields.

### Proposed Additional Inclusions

This list can and should be expanded based on any new ideas we have. Possible
candidates for inclusion include:

#### U128 and I128

One of the reference implementations for XDL is written in Rust, which has
native support for these types. Their use in C++ would require compiler
intrinsics assuming that they're supported on a given platform.

The logic for adding this type would be future proofing the specification for
when 128-bit numbers become standard on consumer grade hardware.

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
panic occuring if neither of those values is found?

### Integers

All integers should be sent in little endian format (i.e. the least significant
byte first) with their corresponding size (8, 16, 32, or 64 bits).

For signed integers, they should be represented in two's complement format.

### Floating Point Numbers

All floating point numbers should be sent as 32 or 64 bit IEEE 754 floating
point numbers.

### Strings

Strings should be sent as UTF-8 encoded characters. They should first send their
length as an unsigned 16-bit integer, followed by the corresponding number of
characters specified by their length.

The chosen type for a length of a String is subject to change (possibly
increasing to a 32-bit unsigned integer).

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

UNDER CONSTRUCTION
