use diesel::prelude::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};

use crate::infrastructure::postgres::schema::quest_adventurer_junction;

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, Queryable, AsChangeset)]
#[diesel(belongs_to(QuestEntity, foreign_key = quest_id))]
#[diesel(belongs_to(AdventurerEntity, foreign_key = adventurer_id))]
#[diesel(table_name = quest_adventurer_junction)]
pub struct QuestAdventurerJunctionModel {
    pub adventurer_id: i32,
    pub quest_id: i32,
}
