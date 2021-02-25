use std::collections::HashMap;

#[derive(Default)]
struct SymbolTable {
    map: HashMap<String, u32>
}

impl SymbolTable {
    fn addEntry(&mut self, symbol: &str, address: u32) {
        self.map.insert(symbol.clone().to_string(), address);
    }

    fn contains(&self, symbol: &str) -> bool {
        self.map.contains_key(symbol)
    }

    fn getAddress(&self, symbol: &str) -> Option<&u32> {
        self.map.get(symbol)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_work() {
        let mut symbol_table: SymbolTable = Default::default();
        symbol_table.addEntry("symbol1", 16);
        symbol_table.addEntry("symbol2", 17);

        assert_eq!(symbol_table.contains("symbol1"), true);
        assert_eq!(symbol_table.contains("symbol2"), true);
        assert_eq!(symbol_table.contains("symbol3"), false);

        assert_eq!(symbol_table.getAddress("symbol1"), Some(&16));
        assert_eq!(symbol_table.getAddress("symbol2"), Some(&17));
        assert_eq!(symbol_table.getAddress("symbol3"), None);
    }
}