CONFIG=config.txt
include $(CONFIG)


CC           = g++
CFLAGS       = -std=c++11 -Wall -Werror -Wextra -O3
TARGET       = main.py
GENERATOR    = data/gen

STOP         = $$(($(START) + $(STEP) * ($(INSTANCES) - 1)))
SEED        := $(od -An -N4 -i /dev/urandom | awk '{print $1}')
TESTS       := $(shell find data/instances -type f -name "*.in" | sort -t_ -k2 -n)


.PHONY: run generate measure clean


run: $(TARGET)
	@python3 $<

$(GENERATOR): data/gen_ubiquity.cpp
	$(CC) $(CFLAGS) $< -o $@ -lm


generate: $(GENERATOR)
	@INST_I=0; \
	for i in $$(seq $(START) $(STEP) $(STOP)); do \
		./$(GENERATOR) $$(echo "$$i - $$i / 3" | bc) $$(echo "$$i / 3" | bc) 0 50 100 100 $(SEED) > ./data/instances/instance_$${INST_I}_$$i.in; \
		INST_I=$$((INST_I + 1)); \
	done;
	@printf "%d Instances created!\n" $(INSTANCES);

measure: $(TARGET)
	@for i in $(TESTS); do \
		sudo chrt -f 99 perf stat -o $$i.time python3 $(TARGET) < $$i > /dev/null; \
		printf "Measured %s\n" $$i; \
	done;
	@printf "%d Instances measured!\n" $(INSTANCES);
	@(cd ../AxisGen; python3 gen_axis.py ../Project3/data/instances);


clean:
	@$(RM) $(GENERATOR)
	@$(RM) data/instances/*.in
	@$(RM) data/instances/*.time
