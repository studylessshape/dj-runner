# dj-runner

The runner is for language [dj](https://gitee.com/ZerAx/dj-rs).

The previous version can be found in [dj](https://gitee.com/ZerAx/dj-rs/tree/master/examples/runner). I try to use [crossterm](https://docs.rs/crossterm) and [tui-input](https://docs.rs/tui-input/) to perfect the experience of input.

## How to use
### From source code
First, enter the root directiory of dj-runner, and then open in terminal.

Real time input:

```bash
$> cargo run
```

Run file:

```bash
$> cargo run -- hello.dj
```

Print help:

```bash
$> cargo run -- -h
```

> The symbol `--` aims to provide args to dj-runner not cargo

### Direct use
Just run this program and you will get like this:

```bash
$> dj-runner
dj-runner -- Version 0.2.0
(core) dj language(dj-rs) -- Version 0.1.0
>
```

Input `dj` and it will run the sentence (support multi-line).

Use runner and specified file path. This is the example:

```bash
$> dj-runner hello.dj
```

Print help:

```bash
$> dj-runner -h
```