use super::model::IpldStore;
use super::schema::Query;
use db::MemoryDB;
use forest_cid::multihash::Blake2b256;
use forest_ipld::Ipld;
use ipld_blockstore::BlockStore;
use juniper::{EmptyMutation, RootNode, Value, Variables};

#[test]
fn test_basic_resolve() {
    let doc = r#"
        {
            resolve(cid: "bafy2bzaced5n2imaxvvrz6ttuz7hrewypbjb55uzdcmvaqh3qzqwi7jsdygfk") {
                integer
            }
        }"#;

    let db = MemoryDB::default();
    let ipld1 = Ipld::Integer(8);
    let cid = db.put(&ipld1, Blake2b256).unwrap();
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
    let ipld1 = Ipld::Integer(8);
    let cid = db.put(&ipld1, Blake2b256).unwrap();
    assert_eq!(
        cid.to_string(),
        "bafy2bzaced5n2imaxvvrz6ttuz7hrewypbjb55uzdcmvaqh3qzqwi7jsdygfk"
    );
    let ipld2 = Ipld::Link(cid);
    let cid = db.put(&ipld2, Blake2b256).unwrap();
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
