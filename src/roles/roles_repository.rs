use crate::roles::dto::{CreateRoleDto, UpdateRoleDto};
use crate::roles::entities::Role;
use crate::shared::http_error::HttpError;
use crate::shared::paginated::{PaginatedRequest, PaginatedResponse};

pub struct RolesRepository {
    pool: sqlx::MySqlPool,
}

impl RolesRepository {
    pub fn new(pool: sqlx::MySqlPool) -> Self {
        Self { pool }
    }

    async fn find_all(&self, req: Option<PaginatedRequest>) -> Result<PaginatedResponse<Role>, HttpError> {
        match req {
            Some(req) => {
                let total = sqlx::query!("select count(*) as total from roles")
                    .fetch_one(&self.pool)
                    .await
                    .unwrap()
                    .total;

                let roles = sqlx::query_as!(
                    Role,"select * from roles limit ? offset ?",
                    req.limit, (req.page.unwrap() -1) * req.limit.unwrap()
                )
                    .fetch_all(&self.pool)
                    .await
                    .unwrap();

                match roles.len() {
                    0 => Err(HttpError::NotFound("Role not found")),
                    _ => Ok(PaginatedResponse::new(roles, total as u32, req.page.unwrap(), req.limit.unwrap()))
                }
            }
            None => {
                let roles = sqlx::query_as!(Role,"select * from roles")
                    .fetch_all(&self.pool)
                    .await
                    .unwrap();

                match roles.len() {
                    0 => Err(HttpError::NotFound("Role not found")),
                    _ => Ok(PaginatedResponse::only_item(roles))
                }
            }
        }
    }

    async fn find_one_by_id(&self, id: u32) -> Result<Role, HttpError> {
        let role = sqlx::query_as!(Role,"select * from roles where id = ?",id)
            .fetch_optional(&self.pool)
            .await
            .unwrap();

        match role {
            Some(role) => Ok(role),
            None => Err(HttpError::NotFound("Role not found"))
        }
    }

    async fn create(&self, create_role: CreateRoleDto) -> Result<Role, HttpError> {
        // find existing role in db
        let existed = sqlx::query_as!(
            Role,"select * from roles where name = ?",create_role.name
        )
            .fetch_optional(&self.pool)
            .await
            .unwrap();

        match existed {
            Some(_) => Err(HttpError::BadRequest("Role already exist")),
            None => {
                let new_role = sqlx::query_as!(
                    Role,"insert into roles (name,description) values (?,?)",
                    create_role.name,
                    create_role.description
                )
                    .execute(&self.pool)
                    .await
                    .unwrap();

                match new_role.last_insert_id() {
                    0 => Err(HttpError::BadRequest("Create role not success")),
                    _ => {
                        let role = sqlx::query_as!(
                            Role,"select * from roles where id = ?",new_role.last_insert_id()
                        )
                            .fetch_one(&self.pool)
                            .await
                            .unwrap();

                        Ok(role)
                    }
                }
            }
        }
    }

    async fn update(&self, id: u32, update_role_dto: UpdateRoleDto) -> Result<Role, HttpError> {
        let update_role = sqlx::query("update roles set name = ?,description = ? where id =?")
            .bind(update_role_dto.name)
            .bind(update_role_dto.description)
            .bind(id)
            .execute(&self.pool)
            .await;

        match update_role {
            Err(_) => Err(HttpError::BadRequest("Can't update role")),
            Ok(_) => {
                let role = sqlx::query_as!(Role,"select * from roles where id =?",id)
                    .fetch_one(&self.pool)
                    .await
                    .unwrap();

                Ok(role)
            }
        }
    }

    async fn delete(&self, id: u32) -> Result<(), HttpError> {
        // find existed role
        let role_existed = sqlx::query_as!(Role,"select * from roles where id =?",id)
            .fetch_optional(&self.pool)
            .await
            .unwrap();

        match role_existed {
            Some(_) => Err(HttpError::NotFound("Role not found")),
            None => {
                let role_deleted = sqlx::query("delete from roles where id = ?")
                    .bind(id)
                    .execute(&self.pool)
                    .await
                    .unwrap();

                match role_deleted.rows_affected() {
                    0 => Err(HttpError::BadRequest("Can't delete role")),
                    _ => Ok(())
                }
            }
        }
    }
}