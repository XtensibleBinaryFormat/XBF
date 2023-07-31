DEBUG	:= -g
OPT		:= # -O2
CXX		:= g++ $(DEBUG) $(OPT)

OBJ	:=	serde.o Buffer.o

bin/01serde: $(OBJ)
	g++ -g $(OBJ) -o $@ -lboost_serialization

serde.o: serde.cpp metadata.hh Buffer.hh rust.hh
	$(CXX) -c $<

Buffer.o: Buffer.cpp Buffer.hh rust.hh
	$(CXX) -c $<



clean:
	rm bin/01serde
	