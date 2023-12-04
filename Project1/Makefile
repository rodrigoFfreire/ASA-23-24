CONFIG=config.txt
include $(CONFIG)


CC           = g++
CFLAGS       = -std=c++11 -Wall -Werror -Wextra -O3
TARGET       = proj1
GENERATOR    = gen

STOP         = $$(($(START) + $(STEP) * ($(INSTANCES) - 1)))
SEED        := $(od -An -N4 -i /dev/urandom | awk '{print $1}')
TESTS       := $(sort $(shell find $(DATA_DIR)/instances -type f -name "*.in"))


.PHONY: run generate measure clean

build: $(TARGET)

$(TARGET): main.cpp solver.cpp
	$(CC) $(CFLAGS) $^ -o $@ -lm


$(GENERATOR): $(DATA_DIR)/gen_supermarble.cpp
    $(CC) $(CFLAGS) $< -o $@ -lm


run: $(TARGET)
	@./$(TARGET)


generate: $(GENERATOR)
    @for i in $$(seq $(START) $(STEP) $(STOP)); do \
        v=$$(echo "$$i / 2" | bc); \
        ./$(DATA_DIR)/$(GENERATOR) $$v $$v $(PIECES) $(SEED) > ./$(DATA_DIR)/instances/instance$$i.in; \
    done;
    @printf "%d Instances created!\n" $(INSTANCES);

measure: $(TARGET)
    @for i in $(TESTS); do \
        sudo chrt -f 99 perf stat -o $$i.time ./$(TARGET) < $$i > /dev/null; \
        printf "Measured %s\n" $$i; \
    done;
    @printf "%d Instances measured!\n" $(INSTANCES);


clean:
	@$(RM) $(TARGET)
	@$(RM) $(DATA_DIR)/$(GENERATOR)
    @$(RM) $(DATA_DIR)/instances/*.in
    @$(RM) $(DATA_DIR)/instances/*.time