use crate::*;

#[derive(Copy, Clone)]
struct Entry {
    key: object::ObjString,
    value: value::Value,
}

impl Entry {
    fn new() -> Entry {
        return Entry {
            key: object::ObjString::new(),
            value: value::Value::new(),
        };
    }
}

pub struct Table {
    count: i64,
    capacity: i64,
    entries: [Entry; 256],
}

impl Table {
    pub fn new() -> Table {
        return Table {
            count: 0,
            capacity: 0,
            entries: [Entry::new(); 256],
        };
    }

    pub fn table_get(&self, obj: object::ObjString, value: value::Value) -> bool {
        todo!();
        }

    pub fn table_set(&self, obj: object::ObjString, value: value::Value) -> bool {
        todo!();
    }

    pub fn table_delete(&self, obj: object::ObjString) -> bool {
        todo!();
    }

    pub fn table_find_string(&self, chars: [char; 256], length: usize, hash: u32) -> Option<object::ObjString> {
        if self.count == 0 {
            return None;
        }

        let index: u32 = hash % (self.capacity as u32);

        loop{
            let entry: Entry = self.entries[index as usize];
            if entry.key.get_length() == 0 {
                return None;
            }
        }
        todo!();
    }

}

