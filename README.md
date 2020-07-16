# Rust lifetime demo

Basically, `lifetime` means `Who can hold what for how long`, How long can hold for a particular resource.

We implement 2 versions of `StrSplit`:

```rust
struct StrSplit<'a> {
    reminder: &'a str,
    delimiter: &'a str,
}

struct StrSplit2<'a> {
    reminder: Option<&'a str>,
    delimiter: Option<&'a str>,
}
``` 

This is the very typical case to show how to use `lifetime` in a correct way, in particular the `lifetime` inside an `Option`.

Also, got an example to show how to get back a `mutable reference to the value inside an Option` rather than take the ownership:

```rust
// `&mut &'a str`: A mutable reference to the option value which is `&'a str` (a reference to a str with the specified lifetime)!!!
let reminder: &mut &'a str = self.reminder.as_mut().unwrap();
```
