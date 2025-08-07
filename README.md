# Backend API and services for StackClass

[![License](https://img.shields.io/github/license/stackclass/backend)](https://github.com/stackclass/backend/blob/master/LICENSE)
[![GitHub contributors](https://img.shields.io/github/contributors/stackclass/backend)](https://github.com/stackclass/backend/graphs/contributors)
[![GitHub issues](https://img.shields.io/github/issues/stackclass/backend)](https://github.com/stackclass/backend/issues)

## Installation

Prebuilt binaries Windows, Linux and macOS can be downloaded from the
[Github release page](https://github.com/stackclass/backend/releases/latest).
If there is no distro package available in your preferred manager,
you need [Rust and cargo](https://www.rust-lang.org/tools/install) to build it.

### Install from source:

1. Clone the repository with `git clone https://github.com/stackclass/backend.git`
2. From the project directory, run `cargo build --release` to build the
   application in release mode.
3. After a successful compilation, launch the executable with:
   `target/release/stackclass-server`.

### Install with cargo

To get the latest bug fixes and features, you can install the development
version from git. However, this is not fully tested. That means you're probably
going to have more bugs despite having the latest bug fixes.

```
cargo install --git https://github.com/stackclass/backend
```

This will download the source from the main branch, build and install it in
Cargo's global binary directory (`~/.cargo/bin/` by default).

## Usage

```text
Usage: stackclass-server [OPTIONS]

Options:
  --port                The server port
  --cache-dir           Base directory for storing cached repositories
  --github-token        A personal token to use for authentication
  --database-url        Database connection URL
  --allowed-origin      Allowed CORS origin
  --git-server-endpoint Git server endpoint
  --git-server-username Username for authenticating with the git server
  --git-server-password Password for authenticating with the git server
  --webhook-endpoint    Webhook handler endpoint
  --git-committer-name  Git committer name
  --git-committer-email Git committer email
  --help                Print help
```

## Development

To build this project, you will need to install the following pre-requisites:
[Git](https://git-scm.com/downloads),
[Rust](https://www.rust-lang.org/tools/install) and
[Just](https://github.com/casey/just).

Clone the Repository

```bash
git clone --recurse-submodules https://github.com/stackclass/backend.git
```

*(If you’ve already cloned the repository without submodules, run `git submodule update --init --recursive` to initialize them.)*

After cloning the repository, you can simply run `just` in the package directory
to list all available commands. For your first local build, please run `just
install` command to install the dependencies for this project.

## Contributing

If anything feels off, or if you feel that some functionality is missing, please
check out the [contributing page](CONTRIBUTING.md). There you will find
instructions for sharing your feedback, building the project locally, and
submitting pull requests to the project.

## License

Copyright (c) The StackClass Authors. All rights reserved.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
