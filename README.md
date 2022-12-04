# Overview

Currently, the flatbuffers library does not have a way to validate a flatbuffer buffer once and to be able to move it around and put it into long lived structures before using the flatbuffer struct later.

The unsafe function root_unchecked lets us accomplish this, but it is
difficult to reason about this unsafe function since the check may
happen at a different time in program execution and it may be difficult
to reason about which buffers are, or are not, validated flatbuffers.

# Approach

This library exposes 3 structs:
* FlatbufferRetained
** This takes an unprefixed flatbuffer and validates it in the new function and allows a user to get the deserialized flatbuffer quickly.
* SizePrefixedFlatbufferRetained
** This takes a size-prefixed flatbuffer and validates it in the new function and allows a user to get the deserialized flatbuffer quickly.
* Retained
** This allows a user to load in either an unprefixed or size-prefixed flatbuffer and deserialize it quickly.


# Example

A basic example, modified from the tutorial
https://google.github.io/flatbuffers/flatbuffers_guide_tutorial.html
is present in examples/basic.rs

it can be run by invoking
`cargo run --example basic`

An example of how this can be used to store data inside
acceleration structures may be observed here.
The test `test_stored_in_map` in tests/test.rs contains a full example.

```rust
#[derive(Default)]
struct MonstersHolder<'a> {
    monsters: std::collections::HashMap<i16, SerializedMonster<'a>>,
}

fn main () -> Result<(), flatbuffers::InvalidFlatbuffer> {
    let mut monsters = MonstersHolder::default();
    monsters.monsters.insert(1, SerializedMonster::new(buf)?);

    monsters.monster.get(1).unwrap().get().hp();

    // Note here we are borrowing the monsters Map mutably
    // but we do not run afoul of the borrow checker since
    // Monster 1's Vec is no longer borrowed but has been
    // safely validated before.
    monsters.monsters.insert(2, SerializedMonster::new(buf)?);

    monsters.monster.get(1).unwrap().get().hp();
    monsters.monster.get(2).unwrap().get().hp();
}
```

