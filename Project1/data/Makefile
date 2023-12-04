CONFIG=config.txt
include $(CONFIG)


CC           = g++
CFLAGS       = -std=c++11 -Wall -Werror -Wextra -O3
TARGET       = proj1
GENERATOR    = gen
INSTANCE_DIR = instances

STOP   		 = $$(($(START) + $(STEP) * ($(INSTANCES) - 1)))
SEED  		:= $(od -An -N4 -i /dev/urandom | awk '{print $1}')
TESTS       := $(sort $(shell find $(INSTANCE_DIR) -type f -name "*.in"))


.PHONY: generate measure clean

generate: $(GENERATOR)
	@for i in $$(seq $(START) $(STEP) $(STOP)); do \
		v=$$(echo "$$i / 2" | bc); \
		./$(GENERATOR) $$v $$v $(PIECES) $(SEED) > $(INSTANCE_DIR)/instance$$i.in; \
	done;
	@printf "%d Instances created!\n" $(INSTANCES);

measure: $(TARGET)
	@for i in $(TESTS); do \
		sudo chrt -f 99 perf stat -o $$i.time ./$(TARGET) < $$i > /dev/null; \
		printf "Measured %s\n" $$i; \
	done;
	@printf "%d Instances measured!\n" $(INSTANCES);


$(TARGET): $(SOURCE_APP)
	$(CC) $(CFLAGS) $< -o $@ -lm

$(GENERATOR): $(SOURCE_GEN)
	$(CC) $(CFLAGS) $< -o $@ -lm

clean:
	@$(RM) $(TARGET) $(GENERATOR)
	@$(RM) $(INSTANCE_DIR)/*.in
	@$(RM) $(INSTANCE_DIR)/*.time