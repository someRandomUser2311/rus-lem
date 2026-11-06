#[macro_export]
macro_rules! record {
    ($name:ident { $($field:ident : $ty:ty),* $(,)? }) => {
        #[derive(Debug, Clone, PartialEq)]
        pub struct $name {
            $(pub $field: $ty),*
        }

        impl $name {
            pub fn report(&self) -> String {
                let parts: Vec<String> =
                    vec![$(format!("{}={:?}", stringify!($field), self.$field)),*];
                if parts.is_empty() {
                    format!("{} {{}}", stringify!($name))
                } else {
                    format!("{} {{ {} }}", stringify!($name), parts.join(", "))
                }
            }

            pub fn field_names() -> Vec<&'static str> {
                Vec::new()
            }
        }
    };
}

record!(Book {
    title: String,
    year: u16,
});

record!(Point { x: i32, y: i32 });

record!(Empty {});
