#[macro_use]
extern crate avocado_derive;
extern crate avocado;
#[macro_use]
extern crate serde_derive;
extern crate serde;

#[derive(Debug, Clone, Serialize, Deserialize, Doc)] //~ ERROR proc-macro derive panicked
#[id_type = "String"] //~| a `Doc` must contain a field serialized as `_id`
#[serde(rename_all = "UPPERCASE")]
struct Bar {
    _id: Uid<Bar>,
}

fn main() {}
