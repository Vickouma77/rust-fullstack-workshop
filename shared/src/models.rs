pub struct Film {
    // uuids as ids
    pub id: uuid::Uuid,
    pub title: String,
    pub director: String,
    pub year: u16,

    //url of the poster
    pub poster: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}
