#[macro_use]
extern crate helix;
extern crate csv;

use std::collections::HashMap;
use std::fs::File;

ruby! {
    class TopStocks {
        def hello() {
            println!("Hello from top_stocks!");
        }

        def import(path: String) -> String {
            println!("{}", path);

            let mut data: HashMap<String, Vec<HashMap<String,String> > > = HashMap::new();

            let file = File::open(path).unwrap();

            let mut rdr = csv::Reader::from_reader(file);
            for result in rdr.records() {
                let record = result.unwrap();
                let mut row = HashMap::new();
                row.insert(String::from("symbol"), String::from(&record[0]));
                row.insert(String::from("date"), String::from(&record[1]));
                row.insert(String::from("close"), String::from(&record[2]));
                row.insert(String::from("volume"), String::from(&record[3]));
                row.insert(String::from("open"), String::from(&record[4]));
                row.insert(String::from("high"), String::from(&record[5]));
                row.insert(String::from("low"), String::from(&record[6]));

                let mut symbol_data = data.entry(String::from(&record[0])).or_insert(Vec::new());
                symbol_data.push(row);
            }

            for arr in data.values_mut() {
                arr.sort_by(|a,b|
                    b.get("close").unwrap().parse::<f32>().unwrap().partial_cmp(&a.get("close").unwrap().parse::<f32>().unwrap()).unwrap()
                );
            }

            println!("{}", data.len());

            return String::new();
        }

    }
}
