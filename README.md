# Agnostik

Agnostik is a layer between your application and the executor for your async stuff.
It lets your switch the executors smooth and easy without having to change your applications code.

## Usage

Agnostik can always be used when you need an executor to execute futures.
If you use Agnostik, you can easily change the executor to be used without changing much code.
Agnostik is especially useful for libraries that want to give their users a free choice about the executor.

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

otherwise add this to your Cargo.toml file
```
agnostik = "0.1.0"
```

## Usage

### Switching executors

You can choose the executor, by using cargo features.
The default runtime is the `bastion-executor`.
To use another executor you just have to disable the default features, and choose one of the valid features.
Valid features are: 
- `runtime_tokio` to use the [Tokio](https://tokio.rs) runtime
- `runtime_asyncstd` to use the [AsyncStd](https://async.rs) runtime
- `runtime_bastion` (default) to use the [Bastion Executor](https://crates.io/crates/bastion-executor)
- `runtime_nostd` (coming soon) to use Agnostik in a no_std environment

E.g. to use the Tokio runtime, add the following line to your Cargo.toml
```
agnostik = { version = "0.1.0", default-features = false, features = ["runtime_tokio"]}
```

### Examples

Agnosiks API is very easy and only has a few methods to use.
Here's an example with the bastion-executor.

```rs
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

If you want to use another exceutor, you just have to replace the `Agnostik::bastion()`
method call, with the method that corresponds to your executor.

Use
- `Agnostik::async_std()` for async std
- `Agnostik::bastion()` for bastion
- `Agnostik::tokio()` for tokio (a new `tokio::runtime::Runtime` object is created using `Runtime::new()` and will panic if it fails to create the runtime)
- `Agnostik::tokio_with_runtime(runtime)` if you want to use your own `tokio::runtime::Runtime` object
- `Agnostik::no_std()` (coming soon) to create a exeutor that works in a nostd environment

### Getting Help

Please head to our [Discord](https://discord.gg/DqRqtRT).

### License

This projcet is licensed under the Apache2 or MIT License.
