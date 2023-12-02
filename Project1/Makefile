CC=g++
CFLAGS=-std=c++11 -Wall -Wextra -Werror -O3 -lm
TARGET=proj1

all: $(TARGET)

$(TARGET): main.cpp solver.cpp
	$(CC) $(CFLAGS) $^ -o $@ -lm

run: $(TARGET)
	@./$(TARGET)

clean:
	@$(RM) $(TARGET)