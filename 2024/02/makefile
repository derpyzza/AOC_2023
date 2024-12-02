CC = clang
TESTS = tests

CFLAGS :=  -DDEBUG -DLOG_COLORS -Wall -pedantic -Wswitch -g -ggdb3 --std=c99
CFLAGS += -DFINAL
LDFLAGS := -L ./derp/src
INCLUDE := -I ./derp/include/

# final: CFLAGS := -Wall -pedantic -Wswitch -g -ggdb3 --std=c99	
# final: p1

p1: p1.c
	$(CC) $^ ./derp/src/* -o $@ $(CFLAGS) $(INCLUDE)
p2: p2.c
	$(CC) $^ ./derp/src/* -o $@ $(CFLAGS) $(INCLUDE)

clean:
	rm -rf p1 p2
