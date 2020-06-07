use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Source {
    pub(crate) url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Image {
    pub(crate) source: Source,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Preview {
    pub(crate) images: Vec<Image>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct PostData {
    pub(crate) title: String,
    pub(crate) preview: Option<Preview>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Post {
    pub(crate) data: PostData,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct RespData {
    pub(crate) dist: isize,
    pub(crate) children: Vec<Post>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Resp {
    pub(crate) kind: String,
    pub(crate) data: RespData,
}
