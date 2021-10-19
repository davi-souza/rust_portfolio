use crate::domain::{InternalItem, Item, NewItem, UpdateItem, UpdateValue};
use crate::utils::*;
use chrono::prelude::{DateTime, Utc};
use failure::Error;
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
use uuid::Uuid;

type Connection = PooledConnection<SqliteConnectionManager>;

pub struct ItemRepository<'a> {
    pub conn: &'a Connection,
}

impl<'a> ItemRepository<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        ItemRepository::<'a> { conn }
    }

    pub fn get_all(self) -> Result<Vec<Item>, Error> {
        let mut stmt = self.conn.prepare(
            r#"
					select "id", "text", "number", "created_at" from "item"
					where "revoked_at" is null
					order by "created_at" desc
					"#,
        )?;
        let items: Vec<Item> = stmt
            .query_map([], |row| {
                Ok(Item {
                    id: uuid_from_str(&row.get::<usize, String>(0)?),
                    text: row.get(1)?,
                    number: row.get(2)?,
                    created_at: datetime_from_str(&row.get::<usize, String>(3)?),
                })
            })
            .and_then(Iterator::collect)?;
        Ok(items)
    }

    pub fn get_by_id(&self, id: Uuid) -> Result<Option<Item>, Error> {
        let mut stmt = self.conn.prepare(
            r#"
					select 
							"id",
							"text",
							"number",
							"created_at"
					from "item"
					where "id" = ? and "revoked_at" is null
					order by "created_at" desc
					"#,
        )?;
        match stmt.query_row([stringify_uuid(id)], |row| {
            Ok(Item {
                id: uuid_from_str(&row.get::<usize, String>(0)?),
                text: row.get(1)?,
                number: row.get(2)?,
                created_at: datetime_from_str(&row.get::<usize, String>(3)?),
            })
        }) {
            Ok(item) => Ok(Some(item)),
            Err(_) => Ok(None),
        }
    }

    pub fn get_all_internal(self) -> Result<Vec<InternalItem>, Error> {
        let mut stmt = self.conn.prepare(
            r#"
					select "pk", "id", "text", "number", "created_at", "revoked_at"
					from "item"
					order by "id", "created_at" desc
					"#,
        )?;
        let items: Vec<InternalItem> = stmt
            .query_map([], |row| {
                Ok(InternalItem {
                    pk: uuid_from_str(&row.get::<usize, String>(0)?),
                    id: uuid_from_str(&row.get::<usize, String>(1)?),
                    text: row.get(2)?,
                    number: row.get(3)?,
                    created_at: datetime_from_str(&row.get::<usize, String>(4)?),
                    revoked_at: get_revoked_at(row.get::<usize, Option<String>>(5)?),
                })
            })
            .and_then(Iterator::collect)?;
        Ok(items)
    }

    pub fn get_all_internal_by_id(self, id: Uuid) -> Result<Vec<InternalItem>, Error> {
        let mut stmt = self.conn.prepare(
            r#"
					select "pk", "id", "text", "number", "created_at", "revoked_at"
					from "item"
                    where "id" = ?
					order by "id", "created_at" desc
					"#,
        )?;
        let items: Vec<InternalItem> = stmt
            .query_map([stringify_uuid(id)], |row| {
                Ok(InternalItem {
                    pk: uuid_from_str(&row.get::<usize, String>(0)?),
                    id: uuid_from_str(&row.get::<usize, String>(1)?),
                    text: row.get(2)?,
                    number: row.get(3)?,
                    created_at: datetime_from_str(&row.get::<usize, String>(4)?),
                    revoked_at: get_revoked_at(row.get::<usize, Option<String>>(5)?),
                })
            })
            .and_then(Iterator::collect)?;
        Ok(items)
    }

    pub fn get_internal_item_by_id(&self, id: Uuid) -> Result<Option<InternalItem>, Error> {
        let mut stmt = self.conn.prepare(
            r#"
					select "pk",
							"id",
							"text",
							"number",
							"created_at",
							"revoked_at"
					from "item"
					where "id" = ? and "revoked_at" is null
					order by "created_at" desc
					"#,
        )?;
        match stmt.query_row([stringify_uuid(id)], |row| {
            Ok(InternalItem {
                pk: uuid_from_str(&row.get::<usize, String>(0)?),
                id: uuid_from_str(&row.get::<usize, String>(1)?),
                text: row.get(2)?,
                number: row.get(3)?,
                created_at: datetime_from_str(&row.get::<usize, String>(4)?),
                revoked_at: get_revoked_at(row.get::<usize, Option<String>>(5)?),
            })
        }) {
            Ok(item) => Ok(Some(item)),
            Err(_) => Ok(None),
        }
    }

    pub fn create_one(self, new_item: NewItem) -> Result<Item, Error> {
        let id = Uuid::new_v4();
        let created_at = Utc::now();
        self.conn.execute(
            r#"
					insert into "item" (
							"pk",
							"id",
							"text",
							"number",
							"created_at",
							"revoked_at"
					)
					values (?, ?, ?, ?, ?, null)
					"#,
            [
                stringify_uuid(Uuid::new_v4()),
                stringify_uuid(id),
                new_item.text.clone(),
                new_item.number.to_string(),
                stringify_datetime(created_at),
            ],
        )?;
        Ok(Item {
            id,
            text: new_item.text.clone(),
            number: new_item.number,
            created_at,
        })
    }

    pub fn update_one(self, id: Uuid, update_item: UpdateItem) -> Result<Item, Error> {
        let now = Utc::now();
        let maybe_old_item = self.get_internal_item_by_id(id)?;
        if maybe_old_item.is_none() {
            return Ok(Item {
                id: Uuid::new_v4(),
                text: String::from("qwe"),
                number: 1,
                created_at: now,
            });
        }
        let old_item = maybe_old_item.unwrap();
        self.conn.execute(
            r#"
					update "item" 
					set "revoked_at" = ?
					where "pk" = ?
					"#,
            [
                format!("{}", now.format("%Y-%m-%dT%H:%M:%S%z")),
                format!("{}", old_item.pk),
            ],
        )?;
        self.conn.execute(
            r#"
					insert into "item" (
							"pk",
							"id",
							"text",
							"number",
							"created_at",
							"revoked_at"
					)
					values (?, ?, ?, ?, ?, null)
					"#,
            [
                stringify_uuid(Uuid::new_v4()),
                stringify_uuid(old_item.id),
                match &update_item.text {
                    UpdateValue::Undefined => old_item.text.clone(),
                    UpdateValue::Value(value) => value.clone(),
                },
                match &update_item.number {
                    UpdateValue::Undefined => old_item.number.to_string(),
                    UpdateValue::Value(value) => value.to_string(),
                },
                stringify_datetime(now),
            ],
        )?;
        Ok(Item {
            id,
            text: match &update_item.text {
                UpdateValue::Undefined => old_item.text.clone(),
                UpdateValue::Value(value) => value.clone(),
            },
            number: match update_item.number {
                UpdateValue::Undefined => old_item.number,
                UpdateValue::Value(value) => value,
            },
            created_at: now,
        })
    }
}

fn get_revoked_at(raw: Option<String>) -> Option<DateTime<Utc>> {
    raw.map(|timestamp| datetime_from_str(&timestamp))
}
