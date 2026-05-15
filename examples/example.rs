use std::{ops::Sub, rc::Rc};

use immutable_rs::ImmutableUpdate;
#[derive(Clone, Debug, ImmutableUpdate)]
struct SubStruct {
    pub b: f32,
}
impl SubStruct {
    pub fn to_rc(self) -> Rc<Self> {
        Rc::new(self)
    }
}

#[derive(Clone, Debug, ImmutableUpdate)]
struct MainTestStruct {
    pub a: f32,
    pub sub: Rc<SubStruct>,
}
fn main() {
    let t = MainTestStruct {
        a: 23.0,
        sub: Rc::new(SubStruct { b: 32.0 }),
    };
    let t = t.with_a(24.0);
    let t: MainTestStruct = MainTestStruct {
        sub: t.sub().with_b(23.0).to_rc(),
        ..t
    };
    let b = t.sub().with_b(23.0);
}
