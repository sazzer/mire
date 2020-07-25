use dotenv;

#[actix_rt::main]
async fn main() {
    let _ = dotenv::dotenv();

    mire_lib::main().await
}
