# aj 
![ci status](https://github.com/cptrodgers/aj/actions/workflows/test-and-build.yml/badge.svg)

aj is background jobs solution (based on actix framework - Actor Model).

## Features & Docs

- [x] Jobs.
  - [x] Type: Instantly, Schedule (Run at specific time), Cron.
  - [x] Update job
  - [x] Cancel job
  - [x] Get job
- [x] Retry mechanism
  - [x] Customizable failed and retry logic: You can handle case that you want to retry based on job output.
  - [x] Configurable:
    - [x] Max times
    - [x] Strategy:
      - [x] Interval Strategy
      - [ ] Exponential Strategy
- [x] Async (execution).
- [x] Persistent.
- [x] Flexible Broker and Backend with `Backend` trait: You can choose your database or storage engine that you want to use.
  - [x] Native support: 
    - [x] Redis
    - [x] In-memory (Not recommend for production, it does not support persisted job)
  - [x] `Backend` trait: you can implement your backend by your demand.
- [x] Custom processing speed.
  - [x] Scan job period (tick).
  - [x] Number of job per tick.
- [ ] Multiple Node (Distributed Mode)
- [ ] DAG (https://en.wikipedia.org/wiki/Directed_acyclic_graph)
- [ ] APIs
- [ ] Dashboard UI
- [ ] Integration

[examples](https://github.com/openexamhq/aj/tree/master/examples)

## Using by:

- [ZenClass](https://zenclass.co) - ZenClass is an education platform that help you build your class.
- [Ikigai](https://ikigai.li) - Ikigai is an AI open assignment platform.

If you're using `aj`, please contact us to update the list.

## LICENSE

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in aj by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
</sub>
