use db::MemoryDB;
use forest_cid::multihash::Blake2b256;
use forest_ipld::Ipld;
use graphql_ipld::{IpldStore, Query};
use ipld_blockstore::BlockStore;
use juniper::{EmptyMutation, RootNode, Value, Variables};
use std::collections::BTreeMap;

#[test]
fn test_basic_resolve() {
    let doc = r#"
        {
            resolve(cid: "bafy2bzaced5n2imaxvvrz6ttuz7hrewypbjb55uzdcmvaqh3qzqwi7jsdygfk") {
                integer
            }
        }"#;

    let db = MemoryDB::default();
    let cid = db.put(&8, Blake2b256).unwrap();
    assert_eq!(
        cid.to_string(),
        "bafy2bzaced5n2imaxvvrz6ttuz7hrewypbjb55uzdcmvaqh3qzqwi7jsdygfk"
    );
    let store = IpldStore::new(db);
    let schema = RootNode::new(Query, EmptyMutation::<IpldStore>::new());

    assert_eq!(
        juniper::execute(doc, None, &schema, &Variables::new(), &store),
        Ok((
            Value::object(
                vec![(
                    "resolve",
                    Value::object(vec![("integer", Value::scalar(8))].into_iter().collect()),
                )]
                .into_iter()
                .collect()
            ),
            vec![]
        ))
    );
}

#[test]
fn test_link_resolution() {
    let doc = r#"
    {
        resolve(cid: "bafy2bzacebdyrodpi5ivwjnqgzkys73khawycs7ch5olmjk2a56tvydcdlcu2") {
          link { integer },
        }
      }"#;

    let db = MemoryDB::default();
    let cid = db.put(&8, Blake2b256).unwrap();
    assert_eq!(
        cid.to_string(),
        "bafy2bzaced5n2imaxvvrz6ttuz7hrewypbjb55uzdcmvaqh3qzqwi7jsdygfk"
    );
    let cid = db.put(&cid, Blake2b256).unwrap();
    assert_eq!(
        cid.to_string(),
        "bafy2bzacebdyrodpi5ivwjnqgzkys73khawycs7ch5olmjk2a56tvydcdlcu2"
    );
    let store = IpldStore::new(db);
    let schema = RootNode::new(Query, EmptyMutation::<IpldStore>::new());

    assert_eq!(
        juniper::execute(doc, None, &schema, &Variables::new(), &store),
        Ok((
            Value::object(
                vec![(
                    "resolve",
                    Value::object(
                        vec![(
                            "link",
                            Value::object(
                                vec![("integer", Value::scalar(8))].into_iter().collect()
                            )
                        )]
                        .into_iter()
                        .collect()
                    ),
                )]
                .into_iter()
                .collect()
            ),
            vec![]
        ))
    );
}

#[test]
fn test_map() {
    let doc = r#"
        {
            resolve(cid: "bafy2bzacebjpfhxkkbdtr3a4ag34rw2uedwzwlzuppqfmawvxafx62h5hr6pq") {
                map { key, value { string } }
            }
        }"#;

    let db = MemoryDB::default();
    let mut e_map = BTreeMap::<String, Ipld>::new();
    e_map.insert("1".to_string(), Ipld::Integer(1));
    e_map.insert("2".to_string(), Ipld::String("test_name".to_owned()));
    let ipld_list = Ipld::Map(e_map);
    let cid = db.put(&ipld_list, Blake2b256).unwrap();
    assert_eq!(
        cid.to_string(),
        "bafy2bzacebjpfhxkkbdtr3a4ag34rw2uedwzwlzuppqfmawvxafx62h5hr6pq"
    );
    let store = IpldStore::new(db);
    let schema = RootNode::new(Query, EmptyMutation::<IpldStore>::new());

    assert_eq!(
        juniper::execute(doc, None, &schema, &Variables::new(), &store),
        Ok((
            Value::object(
                vec![(
                    "resolve",
                    Value::object(
                        vec![(
                            "map",
                            Value::list(vec![
                                Value::object(
                                    vec![
                                        ("key", Value::scalar("1")),
                                        (
                                            "value",
                                            Value::object(
                                                vec![("string", Value::Null)].into_iter().collect()
                                            )
                                        )
                                    ]
                                    .into_iter()
                                    .collect()
                                ),
                                Value::object(
                                    vec![
                                        ("key", Value::scalar("2")),
                                        (
                                            "value",
                                            Value::object(
                                                vec![("string", Value::scalar("test_name"))]
                                                    .into_iter()
                                                    .collect()
                                            )
                                        )
                                    ]
                                    .into_iter()
                                    .collect()
                                ),
                            ]),
                        )]
                        .into_iter()
                        .collect()
                    ),
                )]
                .into_iter()
                .collect()
            ),
            vec![]
        ))
    );
}

#[test]
fn test_list() {
    let doc = r#"
        {
            resolve(cid: "bafy2bzaceatj4dsfk5ylhzvgmg2pn3yhi5mj6z2uzgftqjfvskxgvy5m2dnu4") {
                list { bool, bytes }
            }
        }"#;

    let db = MemoryDB::default();
    let ipld1 = Ipld::List(vec![Ipld::Bool(true), Ipld::Bytes(vec![8, 2])]);
    let cid = db.put(&ipld1, Blake2b256).unwrap();
    assert_eq!(
        cid.to_string(),
        "bafy2bzaceatj4dsfk5ylhzvgmg2pn3yhi5mj6z2uzgftqjfvskxgvy5m2dnu4"
    );
    let store = IpldStore::new(db);
    let schema = RootNode::new(Query, EmptyMutation::<IpldStore>::new());

    assert_eq!(
        juniper::execute(doc, None, &schema, &Variables::new(), &store),
        Ok((
            Value::object(
                vec![(
                    "resolve",
                    Value::object(
                        vec![(
                            "list",
                            Value::list(vec![
                                Value::object(
                                    vec![("bool", Value::scalar(true)), ("bytes", Value::Null)]
                                        .into_iter()
                                        .collect()
                                ),
                                Value::object(
                                    vec![("bool", Value::Null), ("bytes", Value::scalar("0802"))]
                                        .into_iter()
                                        .collect()
                                ),
                            ]),
                        )]
                        .into_iter()
                        .collect()
                    ),
                )]
                .into_iter()
                .collect()
            ),
            vec![]
        ))
    );
}
