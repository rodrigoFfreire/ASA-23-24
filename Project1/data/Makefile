CONFIG=config.txt
include $(CONFIG)


CC=g++
CFLAGS= -std=c++11 -Wall -Werror -Wextra -O3
TARGET=proj1
GENERATOR=gen

STOP=$$(($(START) + $(STEP) * ($(INSTANCES) - 1)))


all: gen_tests $(TARGET)


gen_tests: $(GENERATOR)
	@for i in $$(seq $(START) $(STEP) $(STOP)); do \
		v=$$(echo "$$i / 2" | bc); \
		./$(GENERATOR) $$v $$v $(PIECES) > $(INSTANCE_DIR)/instance$$i.in; \
	done;
	@printf "%d Instances created!\n", $(INSTANCES);


$(TARGET): $(SOURCE_APP)
	$(CC) $(CFLAGS) $< -o $@ -lm

$(GENERATOR): $(SOURCE_GEN)
	$(CC) $(CFLAGS) $< -o $@ -lm

clean:
	@$(RM) $(TARGET) $(GENERATOR)
	@$(RM) $(INSTANCE_DIR)/*.in