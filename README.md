# Web Framework Benchmarks

Benchmarking web frameworks written in [rust] with [wrk] tool.

More requests(`Req/Sec`) in the given time frame means that framework performs
better.  Which means it would require less (CPU) resources to achieve the same
thing.

`Transfer` in [wrk] output means received bytes from all the responses.
Some frameworks include extra headers by default which results in higher count.
This shouldn't impact overall performance much.

## Benchmark Types

### Hello World

Respond "Hello, World!" to every request on "/" endpoint.

- [actix-web](benchmark/hello-world/actix-web/src/main.rs)
- [astra](benchmark/hello-world/astra/src/main.rs)
- [axum](benchmark/hello-world/axum/src/main.rs)
- [hyper](benchmark/hello-world/hyper/src/main.rs)
- [hyper-fast](benchmark/hello-world/hyper-fast/src/main.rs)
- [ntex](benchmark/hello-world/ntex/src/main.rs)
- [poem](benchmark/hello-world/poem/src/main.rs)
- [rocket](benchmark/hello-world/rocket/src/main.rs)
- [salvo](benchmark/hello-world/salvo/src/main.rs)
- [tide](benchmark/hello-world/tide/src/main.rs)
- [viz](benchmark/hello-world/viz/src/main.rs)
- [warp](benchmark/hello-world/warp/src/main.rs)

See [results](result/hello-world-AMD-EPYC-7B13.md).

[wrk]: https://github.com/wg/wrk
[rust]: https://github.com/rust-lang/rust
