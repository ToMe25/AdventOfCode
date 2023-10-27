PROJECT_ROOT = $(dir $(abspath $(lastword $(MAKEFILE_LIST))))

BUILD_DIR := $(PROJECT_ROOT)build/
SRC_DIRS := $(PROJECT_ROOT)src/

TARGET_EXEC := $(BUILD_DIR)Cpp

BUILD_MODE ?= run

CXXFLAGS += -std=c++17
CFLAGS += -pedantic -Wall

ifeq ($(BUILD_MODE),debug)
CFLAGS += -g
BUILD_DIR := $(BUILD_DIR)debug/
TARGET_EXEC := $(TARGET_EXEC)-debug
else ifeq ($(BUILD_MODE),run)
CFLAGS += -O3
else ifeq ($(BUILD_MODE),linuxtools)
CFLAGS += -g -pg -fprofile-arcs -ftest-coverage
LDFLAGS += -pg -fprofile-arcs -ftest-coverage
EXTRA_CLEAN += Cpp.gcda Cpp.gcno $(PROJECT_ROOT)gmon.out
EXTRA_CMDS = rm -rf Cpp.gcda
else
$(error Build mode $(BUILD_MODE) not supported by this Makefile)
endif

SRCS := $(abspath $(wildcard $(addsuffix /*.cpp ,$(SRC_DIRS))))
SRCS += $(abspath $(wildcard $(addsuffix /*.c ,$(SRC_DIRS))))
OBJS := $(subst $(PROJECT_ROOT),,$(SRCS))
OBJS := $(abspath $(addprefix $(BUILD_DIR),$(OBJS)))
OBJS := $(subst .cpp,.o,$(OBJS))
OBJS := $(subst .c,.o,$(OBJS))
DEPS := $(subst .o,.d,$(OBJS))

get_parent_dir = $(dir $(patsubst %/,%,$1))

get_parent_dirs = $(if $(subst $(PROJECT_ROOT),,$(subst ./,,$(dir $1))),$(call get_parent_dirs,$(call get_parent_dir,$1)) $1)

TARGET_DIRS := $(dir $(abspath $(OBJS)))
TARGET_DIRS := $(sort $(foreach dirname,$(TARGET_DIRS),$(call get_parent_dirs,$(dirname))))

.PHONY: all
all:	$(TARGET_EXEC)

$(TARGET_EXEC):	$(OBJS) | $(DEPS)
	$(CXX) $(LDFLAGS) -o $@ $^
	$(EXTRA_CMDS)

$(BUILD_DIR)%.o:	$(PROJECT_ROOT)%.cpp $(BUILD_DIR)%.d Makefile | mkdirs
	$(CXX) -c $(CFLAGS) $(CXXFLAGS) $(CPPFLAGS) -o $@ $<

$(BUILD_DIR)%.o:	$(PROJECT_ROOT)%.c $(BUILD_DIR)%.d Makefile | mkdirs
	$(CC) -c $(CFLAGS) $(CPPFLAGS) -o $@ $<

$(BUILD_DIR)%.d:	$(PROJECT_ROOT)%.cpp | mkdirs
	$(CXX) -MM -MF $@ -MT $(subst .d,.o,$@) $<

$(BUILD_DIR)%.d:	$(PROJECT_ROOT)%.c | mkdirs
	$(CC) -MM -MF $@ -MT $(subst .d,.o,$@) $<

.PHONY: mkdirs
mkdirs:
	$(foreach dir,$(TARGET_DIRS),$(shell mkdir $(dir)))

.PHONY: clean
clean:
	-$(RM) -rf $(BUILD_DIR)
	-$(RM) -rf $(EXTRA_CLEAN)

ifneq (,$(filter-out clean mkdirs,$(MAKECMDGOALS)))
-include $(wildcard $(BUILD_DIR)**/*.d)
else ifeq (,$(MAKECMDGOALS))
-include $(wildcard $(BUILD_DIR)**/*.d)
endif
