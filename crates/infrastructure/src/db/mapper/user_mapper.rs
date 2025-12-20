use domain_users::models::user::User as DomainUser;
use crate::db::entities::user::Model as DbUser;

pub struct UserMapper;

impl UserMapper {
    pub fn to_domain(db_user: DbUser) -> DomainUser {
        DomainUser {
            id: db_user.id,
            name: db_user.name,
            email: db_user.email,
            phone_number: db_user.phone_number,
            password_hash: db_user.password_hash,
        }
    }

    pub fn to_db(domain_user: DomainUser) -> DbUser {
        DbUser {
            id: domain_user.id,
            name: domain_user.name,
            email: domain_user.email,
            phone_number: domain_user.phone_number,
            password_hash: domain_user.password_hash,
        }
    }
}
