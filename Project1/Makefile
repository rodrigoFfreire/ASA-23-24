CONFIG=config.txt
include $(CONFIG)


CC           = g++
CFLAGS       = -std=c++11 -Wall -Werror -Wextra -O3
TARGET       = proj1
GENERATOR    = data/gen

STOP         = $$(($(START) + $(STEP) * ($(INSTANCES) - 1)))
SEED        := $(od -An -N4 -i /dev/urandom | awk '{print $1}')
TESTS       := $(shell find data/instances -type f -name "*.in" | sort -t_ -k2 -n)


.PHONY: run generate measure clean

build: $(TARGET)

$(TARGET): main.cpp
	$(CC) $(CFLAGS) $< -o $@ -lm


$(GENERATOR): data/gen_supermarble.cpp
	$(CC) $(CFLAGS) $< -o $@ -lm


run: $(TARGET)
	@./$(TARGET)


generate: $(GENERATOR)
	@INST_I=0; \
	for i in $$(seq $(START) $(STEP) $(STOP)); do \
		v=$$(echo "$$i / 2" | bc); \
		./$(GENERATOR) $$v $$v $(PIECES) $(SEED) > ./data/instances/instance$${INST_I}_$$i.in; \
		INST_I=$$((INST_I + 1)); \
	done;
	@printf "%d Instances created!\n" $(INSTANCES);
	@(cd ../AxisGen; python3 gen_axis.py ../Project1/data/instances);

measure: $(TARGET)
	@for i in $(TESTS); do \
		sudo chrt -f 99 perf stat -o $$i.time ./$(TARGET) < $$i > /dev/null; \
		printf "Measured %s\n" $$i; \
	done;
	@printf "%d Instances measured!\n" $(INSTANCES);	


clean:
	@$(RM) $(TARGET)
	@$(RM) data/$(GENERATOR)
	@$(RM) data/instances/*.in
	@$(RM) data/instances/*.time
