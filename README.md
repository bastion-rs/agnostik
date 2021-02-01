# Agnostik

[![Crates.io](https://img.shields.io/crates/v/agnostik)](https://crates.io/crates/agnostik)
[![doc](https://img.shields.io/badge/rustdoc-agnostik-blue.svg)](https://docs.rs/agnostik)
[![CI](https://github.com/bastion-rs/bastion/workflows/CI/badge.svg)](https://github.com/bastion-rs/agnostik/actions)

Agnostik is a layer between your application and the executor for your async stuff.
It lets you switch the executors smooth and easy without having to change your applications code.

## Features

- Run futures and wait for them to finish
- Spawn Futures using the underlying executor
- Spawn blocking tasks using special threads that are able to execute blocking code

## Get started

Check the [tests](https://github.com/bastion-rs/agnostik/tree/master/tests) for simple examples.

If you have [cargo-edit](https://github.com/killercup/cargo-edit) installed, you can just execute 
this:
```
cargo add agnostik
```

otherwise, add this to your Cargo.toml file
```
agnostik = "0.2"
```

## Usage

### Switching executors

**Note:** Libraries should not enable any runtime feature.
You can choose the executor, by using cargo features.
There can only be one enabled runtime.
Valid features are: 
- `runtime_bastion` to use the [Bastion Executor](https://crates.io/crates/bastion-executor)
- `runtime_tokio` to use the [Tokio](https://tokio.rs) runtime
- `runtime_asyncstd` to use the [AsyncStd](https://async.rs) runtime
- `runtime_smol` to use the new and awesome [smol](https://docs.rs/smol) runtime

E.g. to use the Tokio runtime, add the following line to your Cargo.toml
```
agnostik = { version = "0.2", features = ["runtime_tokio"]}
```

### Examples

Agnostiks API is very easy and only has a few methods to use.
Here's an example with the bastion-executor.

```rust
use agnostik::prelude::*;

fn main() {
    let runtime = Agnostik::bastion();

    let future = runtime.spawn(async {
        println!("Hello from bastions executor!");
    })
    runtime.block_on(future)
    
    let future = runtime.spawn_blocking(|| {
        expensive_blocking_method();
    })
    runtime.block_on(future)
}
```


There's also a global executor instance that can be used to spawn futures
without creating and storing your own executor.
If you specify multiple runtimes, the global executor will be the following:
- `smol` if `tokio` and `smol` are enabled
- `bastion` if `async_std`, `smol` and / or `tokio` is enabled

```rust
fn main() {
    let future = agnostik::spawn(async { println!("Hello from bastion executor!"); 1 });
    let result = agnostik::block_on(future);
    assert_eq!(result, 1);
}
```

If you want to use another executor, you just have to replace the `Agnostik::bastion()`
method call, with the method that corresponds to your executor.

Use
- `Agnostik::bastion()` for bastion
- `Agnostik::async_std()` for async std
- `Agnostik::tokio()` for tokio. **Warning:** See "How to use tokio runtime"
- `Agnostik::tokio_with_runtime(runtime)` if you want to use your own `tokio::runtime::Runtime` object. **Warning:** See "How to use tokio runtime"
- `Agnostik::no_std()` (coming soon) to create an exeutor that works in a nostd environment

### How to use tokio runtime

It's not supported to use the `tokio::main` macro together with agnostik,
because Agnostik requires a `Runtime` object, which is created by calling `Runtime::new()`.
If your are using the `tokio::main` macro, there will be a panic, because you can't create a runtime
inside a runtime.

Here's how to fix it:

```rust
use agnostik::prelude::*;

#[tokio::main]
async fn main() {
    let runtime = Agnostik::tokio();
    
    let result = runtime.spawn(async_task()).await;

    println!("The result is {}", result)
}
```

This would fail with a panic.
How to do it correctly:

```rust
use agnostik::prelude::*;
use tokio::runtime::Runtime;

fn main() {
    // see tokio docs for more methods to create a runtime
    let runtime = Runtime::new().expect("Failed to create a runtime"); // 1
    let runtime = Agnostik::tokio_with_runtime(runtime); // 2

    let result = runtime.spawn(async_task());
    let result = runtime.block_on(result);

    println!("The result is {}", result)
}
```

You can replace 1 and 2 with `Agnostik::tokio()`, because this method call will
create a Runtime object using `Runtime::new()`.

### Getting Help

Please head to our [Discord](https://discord.gg/DqRqtRT).

### License

This project is licensed under the Apache2 or MIT License.
