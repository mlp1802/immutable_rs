# immutable_rs

`immutable_rs` provides a derive macro for building immutable-style update APIs on Rust types.

With `#[derive(ImmutableUpdate)]` on a named-field struct, the macro generates:

- Getter methods for each field, using the field name directly
- `with_<field>` methods that return an updated cloned value
- `update_<field>` methods that apply a closure and return an updated cloned value, which is useful for nested `Rc<T>` and `Arc<T>` fields such as `update_sub(...)`
- `to_rc(self) -> Rc<Self>`
- `to_arc(self) -> Arc<Self>`

For `Rc<T>` and `Arc<T>` fields, the generated `with_<field>` methods accept the inner value and wrap it automatically. `Rc<String>` and `Arc<String>` setters accept `impl Into<String>`.

## Example

```rust
use std::{rc::Rc, sync::Arc};

use immutable_rs::ImmutableUpdate;

#[derive(Clone, Debug, ImmutableUpdate)]
struct SubStruct2 {
    c: f32,
}

#[derive(Clone, Debug, ImmutableUpdate)]
struct SubStruct1 {
    b: f32,
    sub_2: Arc<SubStruct2>,
}

#[derive(Clone, Debug, ImmutableUpdate)]
struct MainTestStruct {
    a: f32,
    sub: Rc<SubStruct1>,
    shared_name: Arc<String>,
}

fn main() {
    let state = MainTestStruct {
        a: 23.0,
        sub: Rc::new(SubStruct1 {
            b: 32.0,
            sub_2: SubStruct2 { c: 200.0 }.to_arc(),
        }),
        shared_name: Arc::new("before".to_string()),
    };

    let state = state.with_a(24.0);
    let state = MainTestStruct {
        sub: state.sub().with_b(23.0).to_rc(),
        ..state
    };
    let state = state.update_sub(|sub| {
        sub.with_b(24.0)
            .update_sub_2(|sub_2| sub_2.with_c(98.0))
    });
    let state = state.with_shared_name("after");

    let _shared = state.to_arc();
}
```

## Notes

- Field getters and `with_<field>` methods are generated for named-field structs.
- `to_rc` and `to_arc` are generated on any type using the derive.
- The derive expects `Clone`, because update methods rebuild values with `..self.clone()`.
