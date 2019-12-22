extern crate csv;
extern crate rustc_serialize;

use csv::{Reader, Writer};

#[derive(RustcDecodable, RustcEncodable)]
struct Movie {
    title: String,
    bad_guy: String,
    pub_year: usize,
}

fn main() {
    // let dollar_films = vec![
    //     ("A Fistful of Dollars", "Rojo", 1964),
    //     ("For a Few Dollars More", "El Indio", 1965),
    //     ("The Good, the Bad and the Ugly", "Tuco", 1966),
    // ];
    let dollar_films = vec![
        vec!["Title", "Bad Guy", "Year"],
        vec!["A Fistful of Dollars", "Rojo", "1964"],
        vec!["For a Few Dollars More", "El Indio", "1965"],
        vec!["The Good, the Bad and the Ugly", "Tuco", "1966"],
    ];
    let path = "westerns.csv";
    let mut writer = Writer::from_path(path).unwrap();
    for row in dollar_films {
        writer.write_record(&row).expect("CSV writer error");
    }

    let movie = Movie {
        title: "Hang 'Em High".to_string(),
        bad_guy: "Wilson".to_string(),
        pub_year: 1968,
    };
    let movie_record = vec![
        movie.title,
        movie.bad_guy,
        movie.pub_year.to_string(),
    ];
    writer.write_record(&movie_record).expect("CSV writer error");
    writer.flush().expect("Flush error");


    let mut reader = Reader::from_path(path).unwrap();
    for record in reader.records() {
        let row = record.unwrap();
        println!("{:?}", row);
    }

    // Check out the csv crate docs for more
    // You can also change the delimiter for TSV
}
