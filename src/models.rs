use wither::mongodb::oid::ObjectId;
use wither::mongodb::coll::options::IndexModel;

#[derive(Model, Serialize, Deserialize)]
pub struct Guild {
    #[serde(rename="_id", skip_serializing_if="Option::is_none")]
    pub id: Option<ObjectId>,

    #[model(index(index="asc", unique="true"))]
    pub guild_id: u64,
}