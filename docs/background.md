# Background on data interchange formats

XML is inefficient because tags are large, and metadata needed for each record.
Example

```xml
<?xml version="1.0"?>
<Addresses>
  <Address>
    <FirstName>Morticia</FirstName>
    <LastName>Addams</LastName>
    <Street>1313 Mockingbird Lane</Street>
    <City>Hackensack</City>
    <State>NJ</State>
    <Zip>07678</Zip>
  </Address>
</Addresses>
```

JSON is less inefficient but still retains metadata per data

```json
{
    "FirstName": "Morticia",
    "LastName": "Addams",
    "Street":    "1313 Mockingbird Lane",
    "City": "Hackensack",
    "State": "NJ",
    "Zip": 07678
}
```

It would be possible to store the metadata once and just store the data each time, even in JSON

```json
[ "FirstName","LastName","Street", "City","State","Zip"]
[
  ["Morticia", "Addams", "1313 Mockingbird Lane", "Hackensack", "NJ", 07678]

  ["Morticia", "Addams", "1313 Mockingbird Lane", "Hackensack", "NJ", 07678]

  ["Morticia", "Addams", "1313 Mockingbird Lane", "Hackensack", "NJ", 07678]

]
```

XDL does the same thing as this last example, but does it in binary.

Example Point:

```xdl
struct Point {
  double x,y,z;
}
List<Point> a
```

```metadata
This would generate the following metadata
LIST8 3         # list of 3 elements
STRUCT8 3       # each element is struct with 3 fields
STRING8 5 Point # name of struct is Point
F64 1 x         # 1st field is double named x
F64 1 y         # 2nd field is double named y
F64 1 z         # 4rd field is double named z
```

Total metadata is 20 bytes


Every XDL request asks for a page with an even number

get(page 0) --> give me the data
get(page 1) --> give me the metadata for 0, then the data

```cpp
class Point {
public:
  double x,y,z;
};
```

Because C++ is not easy to analyze, we write a compiler that generates C++ code.
For CORBA, this was IDL
XDL is a direct descendent

```xdl
struct Point {
  f64 x,y,z;
}
```
will both generate the C++ code above, and also the metadata because we know what is in the class.

server              client
sends data          already knows the data

or

sends data+meta     reads metadata, displays data automatically

```xdl
struct Student {
  string firstname
  string lastname;
  u32    id;
  f32    gpa;
} 

typedef List<Student> Attending;

website:
  0:  give me all students attending this school

  1:  add new student
  2:  delete student

  3:  update my list of students
       incremental update


website: stock quotes
  0: give me all data on "AAPL", "MSFT", "IBM", ...
  1; update since 2023-05-22


