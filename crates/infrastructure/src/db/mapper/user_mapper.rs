use crate::db::entities::user::Model as DbUser;
use domain_users::models::user::User as DomainUser;

pub struct UserMapper;

impl UserMapper {
    pub fn to_domain(db_user: DbUser) -> DomainUser {
        DomainUser {
            id: db_user.id,
            first_name: db_user.first_name,
            last_name: db_user.last_name,
            email: db_user.email,
            phone_number: db_user.phone_number,
            password_hash: db_user.password_hash,
            birth_date: db_user.birth_date,
            is_email_verified: db_user.is_email_verified,
            role: db_user.role.into(),
            deleted_at: db_user.deleted_at.map(|dt| dt.into()),
        }
    }

    pub fn to_db(domain_user: DomainUser) -> DbUser {
        DbUser {
            id: domain_user.id,
            first_name: domain_user.first_name,
            last_name: domain_user.last_name,
            email: domain_user.email,
            phone_number: domain_user.phone_number,
            password_hash: domain_user.password_hash,
            birth_date: domain_user.birth_date,
            is_email_verified: domain_user.is_email_verified,
            role: domain_user.role.to_string(),
            deleted_at: domain_user.deleted_at.map(|dt| dt.into()),
        }
    }
}
