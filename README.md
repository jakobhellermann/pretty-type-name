# pretty-type-name

A shorter version of `std::any::type_name`.

## Example

```rust
use pretty_type_name::pretty_type_name;

mod foo {
    pub mod bar {
        pub struct X<T>(T);
    }

    pub struct Y;
}

println!(pretty_type_name::<foo::bar::X<foo::Y>>());
// prints `X<Y>`
```