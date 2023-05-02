use self::models::*;
use diesel::prelude::*;
use app08::*;

// cargo run --bin show_posts
fn main() {
    use self::schema::posts::dsl::*;

    let connection = &mut establish_connection();
    let results = posts
        .filter(published.eq(true))
        .limit(5)
        .load::<Post>(connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        print!("{} -- {} \n", post.title, post.body);
    }
}