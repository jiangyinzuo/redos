# Red OS

The Red OS requires Rust nightly version.

## Compile and Make bootable image
```shell script
cargo xbuild --target=x86_64-redos.json
cargo bootimage
```

## References
- [blog_os](https://github.com/phil-opp/blog_os)
- [xy_os](https://github.com/LearningOS/rcore_step_by_step)
- [writing-an-os-in-rust](https://github.com/rustcc/writing-an-os-in-rust)