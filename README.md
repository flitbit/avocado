# Avocado, the strongly-typed MongoDB driver

[![Avocado on crates.io](https://img.shields.io/crates/v/avocado.svg)](https://crates.io/crates/avocado)
[![Avocado on docs.rs](https://docs.rs/avocado/badge.svg)](https://docs.rs/avocado)
[![rustc](https://img.shields.io/badge/rustc-1.32+-turquoise.svg)](https://blog.rust-lang.org/2019/01/17/Rust-1.32.0.html)
[![Avocado Download](https://img.shields.io/crates/d/avocado.svg)](https://crates.io/crates/avocado)
[![Avocado License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/H2CO3/avocado/blob/master/LICENSE.txt)
[![Lines of Code](https://tokei.rs/b1/github/H2CO3/avocado)](https://github.com/Aaronepower/tokei)
[![Twitter](https://img.shields.io/badge/twitter-@H2CO3_iOS-blue.svg?style=flat&colorB=64A5DE&label=Twitter)](http://twitter.com/H2CO3_iOS)

[![goto counter](https://img.shields.io/github/search/H2CO3/avocado/goto.svg)](https://github.com/H2CO3/avocado/search?q=goto)
[![unsafe counter](https://img.shields.io/github/search/H2CO3/avocado/unsafe.svg)](https://github.com/H2CO3/avocado/search?q=unsafe)
[![fuck counter](https://img.shields.io/github/search/H2CO3/avocado/fuck.svg)](https://github.com/H2CO3/avocado/search?q=fuck)

## Usage

* See the [online documentation](https://docs.rs/avocado) above, or open it locally:
* `cargo doc --open`
* Check out the [`examples/`](https://github.com/H2CO3/avocado/blob/master/examples/) folder
* More high-level information can be found on the [project page](https://h2co3.github.io/avocado/).
* The `schema_validation` feature can be enabled (it's enabled by default), in which case the `DatabaseExt::empty_collection()` method becomes available. If a collection is created using this method, it will add a JSON schema validation pass and specify the schema as generated by [`magnet`](https://github.com/H2CO3/magnet).
* The `raw_uuid` feature (also enabled by default) adds some useful extension methods to make it more convenient to work with UUIDs as the type of the `_id` field.

    **This can potentially be slow if you are performing many insertions into a collection of a complex type. However, it dynamically ensures that other users/drivers can't put malformed data in the collection.** Therefore it's probably more useful if you or somebody else are accessing a database from outside the Avocado driver too. It's also great for debugging Avocado itself.

## Changelog

### v0.3.3

* Fix a deprecation warning related to `Uuid::from_random_bytes()`
* Catch some errors that incorrectly pass through the MongoDB client's `Cursor` API

### v0.3.2

* Added a `remove_inner_doc()` method to `DocumentExt`. This allows for the easy chaining of removal from hierarchically contained `Document`s in `transform()`.
* Fixed a bug where inserting 0 entities into a collection failed.
* Fixed the incorrect ordering of the key and the value type in the error message generated by `DocumentExt::remove_*()` methods.

### v0.3.1
* Added a `DocumentExt` trait to the prelude for convenient and idiomatic implementation of `transform(raw: Document) -> Result<Bson>` methods
* Hopefully fixed the code so that docs.rs can handle it now

### v0.3.0
* Added `Doc::id()` and `Doc::set_id()` methods for the sake of better efficiency in some `Collection` methods
    * This means that single-element wrappers such as `Box<Doc>` and `RefCell<Doc>` can no longer implement `Doc` themselves
* Added `Collection::find_one_and_delete()`,  `Collection::find_one_and_replace()`, and `Collection::find_one_and_update()` methods
* Added more documentation and clarified/improved existing docs
* Added more tests, including compile-time tests for cases when `#[derive(Doc)]` should fail, as well as testing that an optional `_id` is correctly allowed

## Compile-time testing the derive macro

Due to a bug in `compiletest_rs`, running the tests that check the error
messages of the `#[derive(Doc)]` proc-macro requires running `cargo clean`
first, otherwise compilation will fail with `E0464`.

Therefore, the recommended way of running the tests is:

    cargo clean && cargo test

## TODO:

* Add `weights` property to text indices
* Add migrations
* Default `Doc::Id` to `ObjectId` and `Query::Output` and `FindAndUpdate::Output` to `T`, once [#29661](https://github.com/rust-lang/rust/issues/29661) is stabilized
