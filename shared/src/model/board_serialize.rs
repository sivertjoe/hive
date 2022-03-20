// Contains custom de/serialize code for GameResource, mainly the HashMap.

use std::{collections::HashMap, fmt};

use serde::{
    de::{Deserialize, Deserializer, MapAccess, Visitor},
    ser::{Serialize, SerializeMap, Serializer},
};

use crate::model::{Board, Square};

#[inline]
fn serialize_key(sq: &Square) -> String
{
    format!("{:?}", *sq)
}

fn deserialize_key(key: String) -> Square
{
    let s = &key;

    let mut iter = s[1..s.len() - 1].split(',').flat_map(|s| s.trim().parse::<isize>());
    (iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap())
}


impl Serialize for Board
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.board.len()))?;
        for (k, v) in &self.board
        {
            map.serialize_entry(&serialize_key(k), &v)?;
        }
        map.end()
    }
}

impl<'de> Visitor<'de> for Board
{
    type Value = Board;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result
    {
        formatter.write_str("a very special map")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut board = Board {
            turns: 0,
            board: HashMap::with_capacity(access.size_hint().unwrap_or(0)),
        };

        while let Some((key, value)) = access.next_entry()?
        {
            board.board.insert(deserialize_key(key), value);
        }

        Ok(board)
    }
}

impl<'de> Deserialize<'de> for Board
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Instantiate our Visitor and ask the Deserializer to drive
        // it over the input data, resulting in an instance of MyMap.
        deserializer.deserialize_map(Board::default())
    }
}
