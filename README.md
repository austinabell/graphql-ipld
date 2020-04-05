# graphql-ipld

GraphQL server, built with [Juniper](https://github.com/graphql-rust/juniper), uses an Ipld store to be able to recursively retrieve any objects saved, regardless of their original types.

## Quickstart

```bash
git clone https://github.com/austinabell/graphql-ipld.git
cd graphql-ipld

# Start graphql server with
cargo run
```
This will start the server at `http://localhost:8000` which will be a GraphiQL interface to be able to run any queries or mutations. By default, there is nothing in the store and there is currently only the option to insert an integer value which returns it's Cid. This can be easily expanded upon or have the data store swapped out for a live one to be able to query any Cids and populate the data.

## Explanation

Since GraphQL only supports explicit fields and each field must have a static type, each Ipld object used will have all of the following fields `{null, bool, integer, float, string, bytes, list, map, link}` all with optional values, and whatever type the data was stored at will be included. 

Some fields functionality to clarify:
null => Will return true if that type was serialized as null
bytes => encoded as hex bytes as a string
list => Array of Ipld objects, which can be selected the same as the base type
map => Array of key value pairs. Since there have to be explicit fields, this cannot be done another way. [issue](https://github.com/graphql/graphql-spec/issues/101)
link => Will retrieve the block of data based on the Cid and deserialize it into the Ipld type.

Ipld type is not needed at all to store items to the ipld store, anything that implements serde Serialize works, and custom queries can be built to be able to deserialize as and more easily select on a specific type as well.

## Future work

If continued, support for easier usage retrieval of IPLD structures such as Hamt (Sharded and merkleizable hashmap) and Amt (Sharded and merkleizable array) since they have dynamic depths to traverse, which is a pain with GraphQL.

Support for mutations (simply insertions) can be expanded upon to handle the different Ipld types, but is less of a priority because that use case seems limited.
