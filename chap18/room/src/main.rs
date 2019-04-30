use std::io;
use serde::Serialize;
use serde_json::Serializer;
use std::collections::HashMap;

type RoomId = String;
type RoomExists = Vec<(char, RoomId)>;
type RoomMap = HashMap<RoomId, RoomExists>;

fn main() {
    let mut map = RoomMap::new();
    map.insert("Cobble Crawl".to_string(), vec![('W', "Debris Room".to_string())]);
    map.insert("Debris Room".to_string(), vec![('E', "Cobble Crawl".to_string()), ('W', "Sloping Canyon".to_string())]);

    let mut serializer = Serializer::new(io::stdout());
    map.serialize(&mut serializer).unwrap();
}
