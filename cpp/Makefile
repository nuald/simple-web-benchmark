CXXFLAGS=-Wall -fsanitize=address -std=c++20 -Wextra -pedantic -pthread \
	-O3 -march=native -Wall -flto=auto -Wa,-mbranches-within-32B-boundaries

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

