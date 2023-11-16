# Simple regex based on finite state machine in rust

the goal is to implement a simple regex that can recognize `'a' to 'z'`, `+`, `*` and `$`.

for the experimental purposes and fastet development speed, I decided to follow the ignorance of `Cargo` as tsoding did in his video, so I made a simple `Makefile` that "supposed" also to work properly in all operating systems.

video source: [https://www.youtube.com/watch?v=MH56D5M9xSQ](https://www.youtube.com/watch?v=MH56D5M9xSQ)

# How to use rust-analyzer in vscode without cargo
run `update_sysroot.ps1` on windows or `update_sysroot.sh` on linux (make it executable first `chmod +x update_sysroot.sh`) and you're done.

# makefile commands
simply use `make clean` to clean up output files, `make build` to clean and build, `make run` to clean, build and run.

# link to the boilerplate
it's here --> [https://github.com/knavels/reusable_files/tree/main/rust/no-cargo-boilerplate](https://github.com/knavels/reusable_files/tree/main/rust/no-cargo-boilerplate)