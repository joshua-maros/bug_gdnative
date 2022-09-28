use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Reference)]
struct Dummy(Vector2Array);

#[methods]
impl Dummy {
    fn new(_base: &Reference) -> Self {
        Self(Vector2Array::from_iter(std::iter::empty()))
    }

    #[method]
    fn make(&self) -> Instance<Self, Unique> {
        Instance::emplace(Self(Vector2Array::from_iter(std::iter::empty())))
    }
}

fn init(handle: InitHandle) {
    handle.add_class::<Dummy>();
}

godot_init!(init);
