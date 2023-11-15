.PHONY : clean build run

clean:
	rm -f main.pdb main main.exe

build: clean
	rustc main.rs

run: build
	./main