Welcome! Thanks for looking into contributing to our project!

# Table of Contents

- [Looking for Help?](#looking-for-help)
  - [Documentation](#documentation)
  - [Chat Rooms](#chat-rooms)
- [Reporting Issues](#reporting-issues)
- [Submitting Code](#submitting-code)
  - [Coding Style](#coding-style)
  - [Modifying Endpoints](#modifying-endpoints)
  - [Submitting PRs](#submitting-prs)
  - [Where do I start?](#where-do-i-start)
- [Testing](#testing)
- [LLM Contributions](#llm-contributions)
  - [Project-related use of LLMs](#project-related-use-of-llms)

# Looking for Help?

Here is a list of helpful resources you can consult:

## Documentation

- [Synapse Admin API Documentation](https://element-hq.github.io/synapse/latest/usage/administration/admin_api/index.html)

## Chat Rooms

- Ruma Matrix room: [#ruma:matrix.org](https://matrix.to/#/#ruma:matrix.org)
- Ruma Development Matrix room: [#ruma-dev:matrix.org](https://matrix.to/#/#ruma-dev:matrix.org)
- Synapse Community room: [#synapse:matrix.org](https://matrix.to/#/#synapse:matrix.org)

# Reporting Issues

If you find any bugs, inconsistencies or other problems, feel free to submit
a GitHub [issue](https://github.com/ruma/synapse-admin-api/issues/new).

If you have a quick question, it may be easier to leave a message in
[#ruma:matrix.org](https://matrix.to/#/#ruma:matrix.org).

Also, if you have trouble getting on board, let us know so we can help future
contributors to the project overcome that hurdle too.

# Submitting Code

Ready to write some code? Great! Here are some guidelines to follow to
help you on your way:

## Coding Style

In general, try to replicate the coding style that is already present. Specifically:

### Naming

For internal consistency, Ruma uses American spelling for variable names. Names may differ in the
serialized representation, as the Matrix specification has a mix of British and American English.

### Common Types

When writing endpoint definitions, use the following mapping from request /
response field types listed in the specification to Rust types:

Specification type | Rust type
-------------------|---------------------------------------------------------------------------------------------------------------------
`boolean`          | `bool`
`integer`          | `js_int::UInt` (unless denoted as signed, then `js_int::Int`)
`string`           | If for an identifier (e.g. user ID, room ID), use one of the types from `ruma-identifiers`. Otherwise, use `String`.
`object`           | `serde_json::Value`
`[…]`              | `Vec<…>`
`{string: …}`      | `BTreeMap<String, …>` (or `BTreeMap<SomeId, …>`)

### Code Formatting and Linting

We use [rustfmt] to ensure consistent formatting code and [clippy] to catch
common mistakes not caught by the compiler as well as enforcing a few custom
code style choices.

```sh
# if you don't have them installed, install or update the nightly toolchain
rustup install nightly
# … and install prebuilt rustfmt and clippy executables (available for most platforms)
rustup component add rustfmt clippy
```

Before committing your changes, run `cargo +nightly fmt` to format the code (if
your editor / IDE isn't set up to run it automatically) and
`cargo +nightly clippy --workspace`¹ to run lints.

¹ If you modified feature-gated code (`#[cfg(feature = "something")]`), you
have to pass `--all-features` or `--features something` to clippy for it to
check that code

[rustfmt]: https://github.com/rust-lang/rustfmt#readme
[clippy]: https://github.com/rust-lang/rust-clippy#readme

### (Type) Privacy and Forwards Compatibility

Generally, all `struct`s that are mirroring types defined in the Synapase Admin API docs should have
all their fields `pub`lic. Where there are restrictions to the fields value beyond their type, these
should generally be implemented by creating or using a more constrained type than the spec uses for
that field – for example, we have a number of identifier types but the Matrix spec uses `string` for
fields that hold user IDs / room IDs and so on.

Almost all types use the `#[non_exhaustive]` attribute, to allow us to adapt to new releases of
Synapse without having a major release of our crate. You can generally just apply
`#[non_exhaustive]` to everything – it's a backwards compatible change to remove it in the rare case
it is not warranted.

Due to this combination of public fields and non-exhaustiveness, all `struct`s generally need a
constructor function or `From` / `TryFrom` implementation to be able to create them in a
straight-forward way (always going through `Deserialize` would be quite ugly).

### Import Formatting

Organize your imports into three groups separated by blank lines:

1. `std` imports
1. External imports (from other crates)
1. Local imports (`self::`, `super::`, `crate::` and things like `LocalEnum::*`)

For example,

```rust
use std::collections::BTreeMap;

use ruma_common::api::ruma_api;

use super::MyType;
```

### Commit Messages

Write commit messages using the imperative mood, as if completing the sentence:
"If applied, this commit will \_\_\_." For example, use "Fix some bug" instead
of "Fixed some bug" or "Add a feature" instead of "Added a feature".

(Take a look at this
[blog post](https://www.freecodecamp.org/news/writing-good-commit-messages-a-practical-guide/)
for more information on writing good commit messages.)

## Modifying Endpoints

### Endpoint Module Structure

Synapse uses versioned endpoints (with a few small exceptions), we follow this versioning approach
in modules as well.

We structure endpoints and their versions like the following;

`endpoint_name::v1`

All bits pertaining a specific version (that can be linked to in the spec) reside in such a module,
some bits may be shared between endpoint versions, but this should be handled on a case-by-case basis.

Endpoint files may have their version modules embedded;

```rs
// endpoint_name.rs

mod v1 {
  // (version-specific stuff)
}
```

This happens if the endpoint either has a single version, or a few versions of sufficiently small size.

### Endpoint Documentation Header

Add a comment to the top of each endpoint file that includes the path
and a link to the documentation of the spec. Replace the version
marker (`v2`) with a `*`, like so;

```rust
//! `GET /_synapse/admin/*/users`
```

Then, in the subsequent version module, embed the version and specification link like so;

```rs
pub mod v2 {
    //! `/v2/` ([spec])
    //!
    //! [spec]: https://github.com/element-hq/synapse/blob/master/docs/admin_api/user_admin_api.md#list-accounts-v2
}
```

### Naming Endpoints

When adding new endpoints, select the module that fits the purpose of the
endpoint. When naming the endpoint itself, you can use the following
guidelines:
- The name should be a verb describing what the client is requesting, e.g.
  `get_some_resource`.
- Endpoints which are basic CRUD operations should use the prefixes
  `create`, `get`, `update`, and `delete`.
- The prefix `set` is preferred to create if the resource is a singleton.
  In other words, when there's no distinction between `create` and `update`.
- Try to use names that are as descriptive as possible and distinct from
  other endpoints in all other modules. (For example, instead of
  `v1::room::get_event`, use `v1::room::get_room_event`).
- If you're not sure what to name it, pick any name and we can help you
  with it.

### Tracking Changes

If your changes affect the public API add an entry about them to the change log
(`CHANGELOG.md`). Where applicable, try to find and denote the version of
Synapse that included the change you are making.

## Submitting PRs

Once you're ready to submit your code, create a pull request, and one of our
maintainers will review it. Once your PR has passed review, a maintainer will
merge the request and you're done! 🎉

## Where do I start?

If this is your first contribution to the project, we recommend taking a look
at one of the [open issues][] we've marked for new contributors.

[open issues]: https://github.com/ruma/synapse-admin-api/issues?q=is%3Aissue+is%3Aopen+label%3A"help+wanted"

# Testing

Before committing, run `cargo check` to make sure that your changes can build,
as well as running the formatting and linting tools
[mentioned above](#code-formatting-and-linting).

# LLM Contributions

Contributions must not include content generated by large language models
or other probabilistic tools like ChatGPT, Claude, and Copilot.

This policy exists due to

- ethical concerns about the data gathering for training these models
- the disproportionate use of electricity and water of building / running them
- the potential negative influence of LLM-generated content on quality
- potential copyright violations

This ban of LLM-generated content applies to all parts of the projects,
including, but not limited to, code, documentation, issues, and artworks.
An exception applies for purely translating texts for issues and comments to
English. We may make more exceptions for other accessibility-related uses.

## Project-related use of LLMs

We heavily discourage the use of LLM chat bots as a replacement for reading
Ruma's documentation and API reference.

Support requests referencing misleading or false LLM output relating to the
project may be ignored, since it is a waste of time for us to "debug" where
things went wrong based on this output before human support was sought.
