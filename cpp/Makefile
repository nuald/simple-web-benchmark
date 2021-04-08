CXXFLAGS=-Wall -fsanitize=address -std=c++20 -Wextra -pedantic -pthread

.PHONY: clean run all

all: target/server

target/server: server.cpp | target
	$(CXX) $(CXXFLAGS) $^ -o $@

run: target/server
	$^

target:
	mkdir -p target

clean:
	$(RM) -r target

