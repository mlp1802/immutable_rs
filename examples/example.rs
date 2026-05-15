use std::{rc::Rc, sync::Arc};

use immutable_rs::ImmutableUpdate;
#[derive(Clone, Debug, ImmutableUpdate)]
struct SubStruct {
    pub b: f32,
}

#[derive(Clone, Debug, ImmutableUpdate)]
struct MainTestStruct {
    pub a: f32,
    pub sub: Rc<SubStruct>,
    pub shared_name: Arc<String>,
}
fn main() {
    let t = MainTestStruct {
        a: 23.0,
        sub: Rc::new(SubStruct { b: 32.0 }),
        shared_name: Arc::new("before".to_string()),
    };
    let t = t.with_a(24.0);
    let t: MainTestStruct = MainTestStruct {
        sub: t.sub().with_b(23.0).to_rc(),
        ..t
    };
    let t = t.with_shared_name("after");
    let b = t.sub().with_b(23.0);
    let _shared = b.to_arc();
}
