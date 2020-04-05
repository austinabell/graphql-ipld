// use super::ipld_visitor::IpldVisitor;
use db::MemoryDB;
use forest_cid::{multihash::Blake2b256, Cid};
use forest_ipld::Ipld;
use ipld_blockstore::BlockStore;
use juniper::Context;
use juniper::{FieldError, FieldResult};
use std::convert::TryInto;

#[derive(Default)]
pub struct IpldStore {
    store: MemoryDB,
}

impl Context for IpldStore {}

impl IpldStore {
    pub fn new(store: MemoryDB) -> Self {
        Self { store }
    }
    pub fn insert_ipld(&self, value: i32) -> Cid {
        // TODO switch unwrap for handled error
        self.store.put(&value, Blake2b256).unwrap()
    }
    pub fn retrieve_ipld(&self, id: &Cid) -> FieldResult<GQLIpld> {
        Ok(self
            .store
            .get::<Ipld>(id)
            .unwrap()
            .ok_or_else(|| {
                FieldError::new(
                    "Temporary error",
                    graphql_value!({ "internal_error": "I'm too lazy to write a real error" }),
                )
            })?
            .into())
    }
}

#[derive(juniper::GraphQLScalarValue, Clone)]
#[graphql(transparent)]
pub struct Bytes(pub String);

#[derive(juniper::GraphQLScalarValue, Clone)]
#[graphql(transparent)]
pub struct Link(pub String);

#[derive(Clone, Default)]
pub struct GQLIpld {
    null: Option<bool>, // TODO ux of this doesn't seem important but revisit
    bool: Option<bool>,
    integer: Option<i32>, // TODO might need i64
    float: Option<f64>,
    string: Option<String>,
    bytes: Option<Bytes>, // TODO revisit if bytes encoded as string
    list: Option<Vec<GQLIpld>>,
    map: Option<Vec<MapItem>>,
    /// Reprents a string encoded Cid
    link: Option<Link>,
}

#[juniper::object(
    Context = IpldStore,
)]
impl GQLIpld {
    fn null(&self) -> Option<bool> {
        self.null
    }
    fn bool(&self) -> Option<bool> {
        self.bool
    }
    fn integer(&self) -> Option<i32> {
        self.integer
    }
    fn float(&self) -> Option<f64> {
        self.float
    }
    fn string(&self) -> &Option<String> {
        &self.string
    }
    fn bytes(&self) -> &Option<Bytes> {
        &self.bytes
    }
    fn list(&self) -> &Option<Vec<GQLIpld>> {
        &self.list
    }
    fn map(&self) -> &Option<Vec<MapItem>> {
        &self.map
    }
    fn link(&self, context: &IpldStore) -> Option<GQLIpld> {
        if let Some(link) = &self.link {
            // TODO replace unwraps with error handling
            Some(context.retrieve_ipld(&link.0.parse().unwrap()).unwrap())
        } else {
            None
        }
    }
}

/// Hack around no map support with GraphQL
#[derive(Clone, Default)]
pub struct MapItem {
    key: String,
    value: GQLIpld,
}

impl MapItem {
    pub fn new(k: String, v: GQLIpld) -> Self {
        Self { key: k, value: v }
    }
}

#[juniper::object(
    Context = IpldStore,
)]
impl MapItem {
    fn key(&self) -> &str {
        &self.key
    }
    fn value(&self) -> &GQLIpld {
        &self.value
    }
}

impl From<Ipld> for GQLIpld {
    fn from(ipld: Ipld) -> Self {
        match ipld {
            Ipld::Bool(v) => Self {
                bool: Some(v),
                ..Default::default()
            },
            Ipld::Integer(v) => Self {
                // TODO this will panic on integer size more than 32 bits
                integer: Some(v.try_into().unwrap()),
                ..Default::default()
            },
            Ipld::Float(v) => Self {
                float: Some(v),
                ..Default::default()
            },
            Ipld::String(v) => Self {
                string: Some(v),
                ..Default::default()
            },
            Ipld::Bytes(v) => Self {
                // Note: This hex encodes bytes
                bytes: Some(Bytes(hex::encode(v))),
                ..Default::default()
            },
            Ipld::List(v) => Self {
                list: Some(v.into_iter().map(From::from).collect()),
                ..Default::default()
            },
            Ipld::Map(v) => Self {
                map: Some(
                    v.into_iter()
                        .map(|(k, v)| MapItem::new(k, v.into()))
                        .collect(),
                ),
                ..Default::default()
            },
            Ipld::Link(v) => Self {
                link: Some(Link(v.to_string())),
                ..Default::default()
            },
            Ipld::Null => Self {
                null: Some(true),
                ..Default::default()
            },
        }
    }
}
