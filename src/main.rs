mod content;

use content::catalog::Catalog;
use content::media::Media;

fn main() {
    let audiobook = Media::Audiobook {
        title: String::from("Flipper"),
    };

    let good_movie = Media::Movie {
        title: String::from("The Great Wall"),
        director: String::from("Matt Damon"),
    };

    let sea_book = Media::Book {
        title: String::from("Mobius Dick"),
        author: String::from("Moby"),
    };

    let funny_book = Media::Book {
        title: String::from("Captain Underpaints"),
        author: String::from("Pants"),
    };
    let podcast = Media::Podcast(10);
    let placeholder = Media::Placeholder;

    println!("{}", audiobook.description());
    println!("{}", good_movie.description());
    println!("{}", funny_book.description());

    let mut catalog = Catalog::new();
    catalog.add(audiobook);
    catalog.add(good_movie);

    catalog.add(funny_book);
    catalog.add(sea_book);
    catalog.add(podcast);
    catalog.add(placeholder);

    match catalog.items.get(100) {
        Option::Some(value) => {
            println!("Item: {:#?}", value);
        }
        Option::None => {
            println!("Nothing at that index")
        }
    }

    println!("{:#?}", catalog);
}
