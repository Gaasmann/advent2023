mod entry;

pub use entry::MapEntry;

#[derive(Debug)]
pub struct Map {
    entries: Vec<MapEntry>,
    sorted: bool,
}

impl Map {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            sorted: true,
        }
    }

    pub fn add_entry(&mut self, entry: MapEntry) {
        self.entries.push(entry);
        self.sorted = false;
    }

    pub fn sort(&mut self) {
        self.entries
            .sort_unstable_by(|a, b| a.source_start.partial_cmp(&b.source_start).unwrap());
        self.sorted = true;
    }

    pub fn convert(&self, source: u64) -> Result<u64, MapError> {
        if !self.sorted {
            return Err(MapError::ConvertOnUnsortedMap);
        }
        let mut chosen_entry = None;
        for entry in &self.entries {
            if entry.source_start > source {
                break;
            } else {
                chosen_entry = Some(entry)
            }
        }
        match chosen_entry {
            Some(entry) => Ok(entry.convert(source)),
            None => Ok(source),
        }
    }
}

impl Default for Map {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(PartialEq, Debug)]
pub enum MapError {
    ConvertOnUnsortedMap,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_add_entry() {
        let entry = MapEntry {
            source_start: 10,
            target_start: 20,
            range_size: 5,
        };
        let mut map = Map::new();
        assert_eq!(map.entries.len(), 0);
        map.add_entry(entry);
        assert_eq!(map.entries.len(), 1);
    }

    fn create_map() -> Map {
        let entry1 = MapEntry {
            source_start: 10,
            target_start: 20,
            range_size: 5,
        };
        let entry2 = MapEntry {
            source_start: 5,
            target_start: 10,
            range_size: 5,
        };
        let entry3 = MapEntry {
            source_start: 40,
            target_start: 60,
            range_size: 5,
        };
        let mut map = Map::new();
        map.add_entry(entry1);
        map.add_entry(entry2);
        map.add_entry(entry3);
        map
    }

    #[test]
    fn map_sort() {
        let mut map = create_map();
        assert_eq!(map.sorted, false);
        map.sort();
        assert_eq!(map.sorted, true);
        assert_eq!(map.entries[0].source_start, 5);
        assert_eq!(map.entries[1].source_start, 10);
        assert_eq!(map.entries[2].source_start, 40);
    }

    #[test]
    fn map_convert_on_unsorted() {
        let mut map = create_map();
        let mut result = map.convert(42);
        assert_eq!(result, Err(MapError::ConvertOnUnsortedMap));
        map.sort();
        result = map.convert(12);
        assert_eq!(result, Ok(22));
    }
}
