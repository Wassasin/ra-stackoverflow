use postcard_rpc::{TopicDirection, endpoints, topics};
use postcard_schema::Schema;

#[derive(Schema)]
pub enum Type {
    A,
    B,
}

#[derive(Schema)]
pub struct Class {
    pub name: u64,
    pub type_: Type,
}

#[derive(Schema)]
pub struct Properties {
    pub non_volatile: bool,
    pub proprietary: bool,
}

#[derive(Schema)]
pub struct Id {
    pub id: u16,
}

#[derive(Schema)]
pub struct Descriptor {
    pub id: Id,
    pub class: Class,
    pub properties: Properties,
}

endpoints! {
    list = ENDPOINT_LIST;
    | EndpointTy                | RequestTy     | ResponseTy            | Path              |
    | ----------                | ---------     | ----------            | ----              |
}

topics! {
    list = TOPICS_IN_LIST;
    direction = TopicDirection::ToServer;
    | TopicTy                   | MessageTy     | Path              |
    | -------                   | ---------     | ----              |
}

topics! {
    list = TOPICS_OUT_LIST;
    direction = TopicDirection::ToClient;
    | TopicTy                   | MessageTy      | Path              |
    | -------                   | ---------      | ----              |
    | Test                      | Descriptor     | "test"            |
}
