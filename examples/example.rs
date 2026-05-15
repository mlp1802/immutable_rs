use std::{rc::Rc, sync::Arc};

use immutable_rs::ImmutableUpdate;
#[derive(Clone, Debug, ImmutableUpdate)]
struct SubStruct2 {
    pub c: f32,
}

#[derive(Clone, Debug, ImmutableUpdate)]
struct SubStruct1 {
    pub b: f32,
    pub sub_2: Arc<SubStruct2>,
}

#[derive(Clone, Debug, ImmutableUpdate)]
struct MainTestStruct {
    pub a: f32,
    pub sub: Rc<SubStruct1>,
    pub shared_name: Arc<String>,
}
fn main() {
    let t = MainTestStruct {
        a: 23.0,
        sub: Rc::new(SubStruct1 {
            b: 32.0,
            sub_2: SubStruct2 { c: 200.0 }.to_arc(),
        }),
        shared_name: Arc::new("before".to_string()),
    };
    let t = t.with_a(24.0);
    let t: MainTestStruct = MainTestStruct {
        sub: t.sub().with_b(23.0).to_rc(),
        ..t
    };
    let t = t.update_sub(|sub| sub.with_b(24.0).update_sub_2(|sub_2| sub_2.with_c(98.0)));
    let t = t.with_shared_name("after");
    let b = t.sub().with_b(23.0);
    let _shared = b.to_arc();
}
