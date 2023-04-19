#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct Authorization {
    actor: String,
    permission: String,
}