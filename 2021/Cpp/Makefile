PROJECT_ROOT = $(dir $(abspath $(lastword $(MAKEFILE_LIST))))

BUILD_DIR := $(PROJECT_ROOT)build/
SRC_DIRS := $(PROJECT_ROOT)src/

TARGET_EXEC := $(BUILD_DIR)Cpp

BUILD_MODE ?= run

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

SRCS := $(shell find $(SRCS_DIR) -name '*.cpp' -or -name '*.c')
OBJS := $(subst $(PROJECT_ROOT),,$(SRCS))
OBJS := $(abspath $(OBJS:%=$(BUILD_DIR)/%))
OBJS := $(subst .cpp,.o,$(OBJS))
OBJS := $(subst .c,.o,$(OBJS))

get_parent_dir = $(dir $(patsubst %/,%,$1))

get_parent_dirs = $(if $(subst $(PROJECT_ROOT),,$(subst ./,,$(dir $1))),$(call get_parent_dirs,$(call get_parent_dir,$1)) $1)

TARGET_DIRS := $(dir $(abspath $(OBJS)))
TARGET_DIRS := $(sort $(foreach dirname,$(TARGET_DIRS),$(call get_parent_dirs,$(dirname))))

all:	$(TARGET_EXEC)

$(TARGET_EXEC):	$(OBJS)
	$(CXX) $(LDFLAGS) -o $@ $^
	$(EXTRA_CMDS)

$(BUILD_DIR)%.o:	$(PROJECT_ROOT)%.cpp | mkdirs
	$(CXX) -c $(CFLAGS) $(CXXFLAGS) $(CPPFLAGS) -o $@ $<

$(BUILD_DIR)%.o:	$(PROJECT_ROOT)%.c | mkdirs
	$(CC) -c $(CFLAGS) $(CPPFLAGS) -o $@ $<

.PHONY: mkdirs
mkdirs:
	$(foreach dir,$(TARGET_DIRS),$(shell mkdir $(dir)))

.PHONY: clean
clean:
	-rm -rf $(BUILD_DIR)
