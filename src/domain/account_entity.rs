use chrono::{NaiveDateTime};
use diesel::{AsChangeset, Queryable, Identifiable};
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;
use uuid::{Uuid};

#[derive(Queryable, AsChangeset, Identifiable, Serialize, Deserialize, ToSchema)]
#[diesel(table_name = crate::core::app::schema::account)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AccountEntity{
    id: i32,
    uuid: Uuid,
    name: String,
    mail_address: String,
    created_at: NaiveDateTime
}

impl AccountEntity {
    pub fn new(id: i32,
        uuid: Uuid,
        name: String,
        mail_address: String,
        created_at: NaiveDateTime) -> AccountEntity{
            AccountEntity {id, uuid, name, mail_address, created_at}
    }
}

#[cfg(test)]
mod tests{
    use uuid::Uuid;

    use super::{AccountEntity};
    use chrono::Utc;

    #[test]
    fn create_entity_works(){
        let sub_account: AccountEntity = AccountEntity  {  
                id: 30,
                uuid: Uuid::new_v4(),  
                name: "Customer Name".to_string(), 
                mail_address: "test@mail.org".to_string(), 
                created_at: Utc::now().naive_utc()
            };
        assert_eq!(sub_account.id,30);
        assert_eq!(sub_account.mail_address as String, "test@mail.org");
    }
}