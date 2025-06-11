/// Application層での商品購入コマンド
/// HTTPの詳細には依存しない
#[derive(Debug)]
pub struct BuyProductCommand {
    pub quantity: u32,
}
