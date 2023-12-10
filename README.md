# aj (Actix Background Job)

This repository contains a simple background job that supports a web server based on actix-web or other web/API frameworks in Rust.

## Features:

- Update Job
- Cancel job.
- Persistent Job
- Job Types:
  - Normal Job (instant)
  - Schedule Job
  - Cron Job

## Architecture

AJ want to solve the background job done in the simple way.
So, it should bea:
- Minimal
- Easy to understand
- Easy to integrate

[Architecture Doc](https://github.com/cptrodgers/aj/blob/master/ARCHITECTURE.md)

## LICENSE

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in simple-aws-s3 by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
</sub>