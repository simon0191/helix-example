#[macro_use]
extern crate helix;
extern crate csv;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::collections::HashMap;
use std::fs::File;

#[derive(Debug,Serialize,Deserialize)]
struct Record {
    symbol: String,
    date: String,
    close: Option<f64>,
    volume: Option<f64>,
    open: Option<f64>,
    high: Option<f64>,
    low: Option<f64>,
}

ruby! {
    class TopStocks {
        def hello() {
            println!("Hello from top_stocks!");
        }

        def import(path: String) -> String {
            let mut data: HashMap<String, Vec<Record>> = HashMap::new();

            let file = File::open(path).unwrap();

            let mut rdr = csv::Reader::from_reader(file);
            for result in rdr.deserialize() {

                let record: Record = result.unwrap();

                let mut symbol_data = data.entry(record.symbol.clone()).or_insert(Vec::new());
                symbol_data.push(record);
            }

            for arr in data.values_mut() {
                arr.sort_by(|a,b|
                    b.close.partial_cmp(&a.close).unwrap()
                );
                arr.truncate(10);
            }

            let json = serde_json::to_string(&data).unwrap();

            return json;
        }

    }
}
