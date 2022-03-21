# Serde-teamspeak-querystring

<br>
An alternative teamspeak query string parser based on serde for rust.


## Usage

```rust
// In your main function
let x: MyStruct = serde_teamspeak_querystring::from_str("YOUR QUERY STRING");
```

To see what is supported and what is not, please [read the docs](https://docs.rs/serde-querystring).

There is also `serde-querystring-actix` crate to support using this crate with actix-web. It provides `QueryString` extractor which works just like the actix-web's own web::Query but uses `serde-querystring` to deserialize.

## Why

Existing alternatives don't cover some cases, for example enums (having enums in sequences, usefull for filters in rest apis) is not a first class value in similar crates. This crate tries to cover more real world use cases and it uses a different strategy to parse the query string which gives it some freedom to decide how to parse depending on the resulting data structure. Note that it may not be fully compatible with some existing standards in some cases, but it tries to support the cases they defined.

## Warning

This project is still in its early stage of development and things may change without notice, so use it at your own risk.

## Alternatives

If you're looking for a more production ready alternative, consider looking at these crates:

`serde_urlencoded`: a performant query parser which doesn't support subkeys (aka dicts)

`serde_qs` a better alternative to this one which is more mature and is featureful enough for most cases

## Known bugs

- Doesn't have correct error reporting yet
- Doesn't support unit types
- Doesn't check if it visited all the input when visiting subkeys
- Doesn't support deserializing into a sequence of key-values instead of map(More of a feature)

Tests only cover valid querystrings in some cases, so there can be some bugs here and there. If you face a bug, please open an issue and let me know.

## Credit

It uses a good amount of code ported or copied from `serde_json` crate and some lines from `form_urlencoded` crate for example to parse numbers or strings.

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.
