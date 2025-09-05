pub mod event;
pub mod scheduler;

pub mod macros {
    #[macro_export]
    macro_rules! get_set {
        ($name:ident, $set_name:ident, $type:ty) => {
            pub fn $name(&self) -> $type {
                self.$name
            }

            pub fn $set_name(&mut self, value: $type) {
                self.$name = value
            }
        };
    }
}
