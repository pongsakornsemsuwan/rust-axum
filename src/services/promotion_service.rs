use std::time::SystemTime;
use crate::models::promotion_model::Promotion;


pub async fn get_promotion(id: String) -> Promotion {
  let promotion = Promotion {
    name: "Single Promotion".to_owned(),
    created_at: SystemTime::now(),
  };
  promotion
}

pub async fn get_all_promotions() -> Vec<Promotion> {
  let promotion1 = Promotion {
    name: "Promo 1".to_owned(),
    created_at: SystemTime::now(),
  };
  let promotion2 = Promotion {
    name: "Promo 2".to_owned(),
    created_at: SystemTime::now(),
  };
  let mut promotions = Vec::new();
  promotions.push(promotion1);
  promotions.push(promotion2);

  promotions
}