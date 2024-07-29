pub mod mpesa;


use async_graphql::MergedObject;
use mpesa::PurchasePhoto;



#[derive(MergedObject, Default)]
pub struct MpesaMut(pub PurchasePhoto);
