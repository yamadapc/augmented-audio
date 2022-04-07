# augmented-atomics

This crate is part of <https://github.com/yamadapc/augmented-audio/>. It exposes a couple of
extra very limited "atomic" data types which are useful for audio development.

In particular, it exposes:

* `AtomicF32`
* `AtomicF64`
* `AtomicEnum`
* `AtomicOption`

In addition, an `AtomicValue` trait is provided, which allows `AtomicOption` to be available.

License: MIT
