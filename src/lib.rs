use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Reference)]
struct Dummy;

#[methods]
impl Dummy {
    fn new(_base: &Reference) -> Self {
        Self
    }

    #[method]
    fn make(&self) -> Instance<Self, Unique> {
        // Same effect with:
        // Instance::emplace(Self)
        Instance::new()
    }
}

fn init(handle: InitHandle) {
    handle.add_class::<Dummy>();
}

godot_init!(init);
