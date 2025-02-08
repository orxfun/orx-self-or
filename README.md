# orx-self-or

[![orx-self-or crate](https://img.shields.io/crates/v/orx-self-or.svg)](https://crates.io/crates/orx-self-or)
[![orx-self-or crate](https://img.shields.io/crates/d/orx-self-or.svg)](https://crates.io/crates/orx-self-or)
[![orx-self-or documentation](https://docs.rs/orx-self-or/badge.svg)](https://docs.rs/orx-self-or)

Defines SoR (self-or-ref) and SoM (self-or-mut) traits that are useful in reducing code duplication and pushing forward the ownership transfer decision from the type designer to the consumer.

## SoR and SoM Traits

`SoR<T>` trait represents two variants of type `T`:
* self: `T`, and
* shared reference: `&T`.

It has a single method producing a shared references of `T`, which is the common behavior of both variants:

```rust ignore
fn get_ref(&self) -> &T;
```

`SoM<T>` trait is more demanding, stronger and represents the following variants of `T`:
* self: `T`, and
* mutable reference: `&mut T`.

In addition to `get_ref`, it is also capable of creating mutable references of `T`:

```rust ignore
fn get_mut(&mut self) -> &mut T;
```

## Motivation

These two simple traits can be very useful in certain situations.

Consider the following scenario:
* We are creating a type `X` which contains a field of type `Y`.
  * It is straightforward to define this as `struct X(Y)`.
* However, assume that `X` does not need to own `Y`, all it needs is to access certain fields or methods through a shared reference `&Y`.
  * In this case, we might also consider `struct X<'a>(&'a Y)`.

Which one is better? We might not know. It is possible that both have their advantages in different situations. Yet, as the designer of type `X`, we are required to decide, or duplicate code to allow for both variants.

If we don't want to care or we want to leave the choice to the consumer, we can use `SoR` trait as follows:

```rust
use orx_self_or::SoR;

struct Y(usize);

struct X<S: SoR<Y>>(S);

impl<S: SoR<Y>> X<S> {
    fn magic_num(&self) -> usize {
        self.0.get_ref().0
    }
}

// the caller can now create X that owns the Y
let y = Y(42);
let x = X(y);
assert_eq!(x.magic_num(), 42);

// or only holds a reference to Y
let y = Y(42);
let x = X(&y);
assert_eq!(x.magic_num(), 42);
```

The mutable counterpart `SoM` has the same motivation as demonstrated below.

```rust
use orx_self_or::SoM;

struct Y(usize);

struct X<S: SoM<Y>>(S);

impl<S: SoM<Y>> X<S> {
    fn magic_num(&self) -> usize {
        self.0.get_ref().0
    }

    fn new_magic_number(&mut self, magic: usize) {
        self.0.get_mut().0 = magic;
    }
}

// the caller can now create X that owns the Y
let y = Y(42);
let mut x = X(y);
assert_eq!(x.magic_num(), 42);
x.new_magic_number(21);
assert_eq!(x.magic_num(), 21);

// or only holds a mutable reference to Y
let mut y = Y(42);
let mut x = X(&mut y);
assert_eq!(x.magic_num(), 42);
x.new_magic_number(21);
assert_eq!(x.magic_num(), 21);

assert_eq!(y.0, 21);
```

You may find two similar demonstrations in [field_sor.rs](https://github.com/orxfun/orx-self-or/blob/main/examples/field_sor.rs) and [field_som.rs](https://github.com/orxfun/orx-self-or/blob/main/examples/field_som.rs) examples.


## Contributing

Contributions are welcome! If you notice an error, have a question or think something could be improved, please open an [issue](https://github.com/orxfun/orx-self-or/issues/new) or create a PR.

## License

Dual-licensed under [Apache 2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT).
