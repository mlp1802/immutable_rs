# immutable_rs

`immutable_rs` provides a derive macro for building immutable-style update APIs on Rust types.

With `#[derive(ImmutableUpdate)]` on a named-field struct, the macro generates:

- Getter methods for each field, using the field name directly
- `with_<field>` methods that return an updated cloned value
- `update_<field>` methods that apply a closure and return an updated cloned value
- `to_rc(self) -> Rc<Self>`
- `to_arc(self) -> Arc<Self>`

For `Rc<T>` and `Arc<T>` fields, the generated `with_<field>` methods accept the inner value and wrap it automatically. `Rc<String>` and `Arc<String>` setters accept `impl Into<String>`.

## Example

```rust
use std::{rc::Rc, sync::Arc};

use immutable_rs::ImmutableUpdate;

#[derive(Clone, Debug, ImmutableUpdate)]
struct Profile {
    name: Arc<String>,
    visits: u32,
}

#[derive(Clone, Debug, ImmutableUpdate)]
struct AppState {
    profile: Rc<Profile>,
    title: String,
}

fn main() {
    let state = AppState {
        profile: Profile {
            name: Arc::new("Mikkel".to_string()),
            visits: 1,
        }
        .to_rc(),
        title: "Dashboard".to_string(),
    };

    let state = state.with_title("Overview".to_string());
    let state = AppState {
        profile: state.profile().with_visits(2).to_rc(),
        ..state
    };

    let state = state.update_profile(|profile| profile.with_name("Updated"));

    let _shared = state.to_arc();
}
```

## Notes

- Field getters and `with_<field>` methods are generated for named-field structs.
- `to_rc` and `to_arc` are generated on any type using the derive.
- The derive expects `Clone`, because update methods rebuild values with `..self.clone()`.
