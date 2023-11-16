.PHONY : clean build run

OS := $(shell uname)
ifeq ($(findstring "NT", $(OS)),)
	EXECUTABLE := .exe
else
	EXECUTABLE :=
endif

clean:
	clear
	rm -f output/main.* output/main$(EXECUTABLE)

build: clean
	rustc main.rs -o output/main$(EXECUTABLE)

run: build
	./output/main