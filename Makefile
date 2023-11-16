.PHONY : clean build run

clean:
	clear
	rm -f main.pdb main main.exe

build: clean
	rustc main.rs

run: build
	./main