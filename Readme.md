# dj-runner
The runner is for language [dj](https://gitee.com/ZerAx/dj-rs).

The previous version can be found in [dj](https://gitee.com/ZerAx/dj-rs/tree/master/examples/runner). I try to use [crossterm](https://docs.rs/crossterm) and [tui-input](https://docs.rs/tui-input/) to perfect the experience of input.

## How to use
### Real time input
Just run this program and you will get like this:

```sh
dj-runner -- Version 0.2.0
(core) dj language(dj-rs) -- Version 0.1.0
>
```

Input `dj` and it will run the sentence (support multi-line).

### Run file
Use runner and specified file path. This is the example:

```sh
dj-runner hello.dj
```