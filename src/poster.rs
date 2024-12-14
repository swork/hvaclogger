use serde::Serialize;

pub trait Poster<T: Serialize> {
    fn post(&self, item: &T) -> bool {
        let j = serde_json::to_string(&item).unwrap();
        println!("Post!: {j}");
        true
    }
}
