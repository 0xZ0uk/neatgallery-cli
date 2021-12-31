use std::error::Error;
use image::{GenericImage, GenericImageView};

mod twitter_api;

use crate::twitter_api::{get_user, get_follower_count};

fn process_image() -> Result<(), Box<dyn Error>> {
    let mut img = image::open("tests/images/jpg/wall_surface.jpg").unwrap();
    let frame = image::open("tests/images/png/frame.png").unwrap();
    println!("dimensions {:?}", img.dimensions());
    println!("{:?}", img.color());

    img.copy_from(&frame, 100, 100);
    img.save("tests/images/png/test.png").unwrap();

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>  {
    let user = get_user("0xz0uk").await?;
    let follower_count = get_follower_count(&user.id).await?;

    println!("UserID: {:?}", user);
    println!("Follower Count: {:?}", follower_count);

    process_image();

    Ok(())
}