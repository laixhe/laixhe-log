use tide::Request;
use tide::prelude::*;

#[derive(Debug, Deserialize)]
struct PostAnimal {
    name: String,
    legs: u8,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    app.at("/orders/shoes").post(order_shoes);
    app.listen("127.0.0.1:5050").await?;
    Ok(())
}

async fn order_shoes(mut req: Request<()>) -> tide::Result {
    let PostAnimal { name, legs } = req.body_json().await?;
    Ok(format!("Hello, {}! I've put in an order for {} shoes", name, legs).into())
}
