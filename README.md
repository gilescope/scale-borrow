# scale-borrow

Decode scale in a dynamic way but reasonably efficiently because rust can.

## Who this crate is not for

If you are targeting a few specific parachains --> use subxt

If you are targeting lots of parachains and just need something easy --> use scale-value (or https://github.com/virto-network/scales)

## Goals

   * fun, pleasent
   * efficient
   * few deps, fast to compile (32 deps)
   * wasm compatible - currently using integritee's fork of frame-metadata to achieve this.

## How to use


### All the world is a `Value`

```rust
   let val = ValueBuilder::parse(&encoded, top_type_id, &types);
   assert_eq!(
      val,
      Value::Object(Box::new(vec![
         ("val", Value::Bool(true)),
         ("name", Value::Str("hi val"))
      ]))
   );
```

## Status

Very experimental

TODO non-panic error handling,
