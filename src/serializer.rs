
use std::collections::{ HashMap, BTreeMap };
use rustc_serialize::json::{ Json, ToJson };

type UserID = u64;
type UserList = HashMap<UserID, User>;

struct Serializer {

}

impl Serializer {
    fn hello_world(&self) {
        println!("Hello world");
    }
}

enum Data<K, V, E> {
    User(Box<T>),
    Group,
    Message
}

enum Option<T> {
    Some(Box<T>),
    None
}

// T == Box<User>
// 1234
// 0000

// Groups are formed whenever there is text communication between users, each
// chat window will have a group JSON object given to it to populate the
// window / portlet.

//  "User" : {
//      "first_name" : "Colin"
//      "last_name" : "Kirkpatrick"
//      "id" : "117"
//      "groups" :
//      ["Group" {
//          users: ...
//      },
//      
//      ]
//  }

#[derive(RustcDecodable, RustcEncodable)]
struct User {
    first_name: String,
    last_name: String,
    id: UserID,
    group: Group,
}

/// Group 'users' field holds the JSON objects for users, everything else will
/// refer to UserID to avoid infinite recursion (google it).
#[derive(RustcDecodable, RustcEncodable)]
struct Group {
    users: Vec<User>,
    admins: Vec<UserID>,
    chat_history: Vec<String>,
}

/// Immutable pieces of data which represent a message send to and from someone.
#[derive(RustcDecodable, RustcEncodable)]
struct Message {
    user: UserID,
    msg: str,
    chat_history: Vec<String>,
}

impl ToJson for User {
    fn to_json(&self) -> Json {
        let mut map = BTreeMap::new();
        map.insert("first_name".to_string(), self.first_name.to_json());
        map.insert("last_name".to_string(), self.last_name.to_json());
        map.insert("id".to_string(), self.id.to_json());
        Json::Object(map)
    }
}

//impl ToJson for Group {
    //fn to_json(&self) -> Json {
        //let mut map = BTreeMap::new();
        //map.insert("users".to_string(), self.first_name.to_json());
        //map.insert("admins".to_string(), self.last_name.to_json());
        //map.insert("id".to_string(), self.id.to_json());
        //Json::Object(map)
    //}
//}

