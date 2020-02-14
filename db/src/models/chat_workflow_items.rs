use chrono::NaiveDateTime;
use diesel::expression::dsl;
use diesel::prelude::*;
use models::*;
use schema::{chat_workflow_items, chat_workflow_responses};
use utils::errors::*;
use uuid::Uuid;

pub const DEFAULT_RESPONSE_WAIT_IN_SECONDS: i32 = 10;

#[derive(Clone, Queryable, Identifiable, Insertable, Serialize, Deserialize, PartialEq, Debug)]
#[table_name = "chat_workflow_items"]
pub struct ChatWorkflowItem {
    pub id: Uuid,
    pub chat_workflow_id: Uuid,
    pub item_type: ChatWorkflowItemType,
    pub message: Option<String>,
    pub render_type: Option<ChatWorkflowItemRenderType>,
    pub response_wait: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize, PartialEq, Debug)]
#[table_name = "chat_workflow_items"]
pub struct NewChatWorkflowItem {
    pub chat_workflow_id: Uuid,
    pub item_type: ChatWorkflowItemType,
    pub message: Option<String>,
    pub render_type: Option<ChatWorkflowItemRenderType>,
    pub response_wait: i32,
}

#[derive(AsChangeset, Default, Deserialize, Debug)]
#[table_name = "chat_workflow_items"]
pub struct ChatWorkflowItemEditableAttributes {
    #[serde(default, deserialize_with = "double_option_deserialize_unless_blank")]
    pub message: Option<Option<String>>,
    pub render_type: Option<Option<ChatWorkflowItemRenderType>>,
}

pub static MESSAGE_REPLACEMENT_HELPERS: &'static [&str] = &["last_input", "error_message"];

impl ChatWorkflowItem {
    pub fn remaining_response_types(
        &self,
        conn: &PgConnection,
    ) -> Result<Vec<ChatWorkflowResponseType>, DatabaseError> {
        let mut available_response_types = self.available_response_types();

        // Answer allows multiple, all other types can only be included once
        for response_type in self.response_types(conn)? {
            match response_type {
                ChatWorkflowResponseType::Answer => (),
                _ => available_response_types.retain(|&x| x != response_type),
            }
        }

        Ok(available_response_types)
    }

    pub fn chat_workflow(&self, conn: &PgConnection) -> Result<ChatWorkflow, DatabaseError> {
        ChatWorkflow::find(self.chat_workflow_id, conn)
    }

    pub fn response_valid(
        &self,
        chat_workflow_response: &ChatWorkflowResponse,
        conn: &PgConnection,
    ) -> Result<bool, DatabaseError> {
        Ok(self
            .response_types(conn)?
            .contains(&chat_workflow_response.response_type))
    }

    pub fn message(&self, chat_session: &ChatSession) -> Option<String> {
        let message = self.message.clone();
        if let Some(mut message) = message {
            for text in MESSAGE_REPLACEMENT_HELPERS {
                message = message.replace(
                    &format!("{{{}}}", text),
                    chat_session.context[text].as_str().unwrap_or(""),
                );
            }

            return Some(message);
        }

        None
    }

    pub fn response_types(&self, conn: &PgConnection) -> Result<Vec<ChatWorkflowResponseType>, DatabaseError> {
        Ok(self.responses(conn)?.into_iter().map(|r| r.response_type).collect())
    }

    pub fn responses(&self, conn: &PgConnection) -> Result<Vec<ChatWorkflowResponse>, DatabaseError> {
        chat_workflow_responses::table
            .filter(chat_workflow_responses::chat_workflow_item_id.eq(self.id))
            .select(chat_workflow_responses::all_columns)
            .order_by(chat_workflow_responses::rank)
            .get_results(conn)
            .to_db_error(ErrorCode::QueryError, "Could not get responses for workflow item")
    }

    pub fn available_response_types(&self) -> Vec<ChatWorkflowResponseType> {
        return match self.item_type {
            ChatWorkflowItemType::Question => vec![ChatWorkflowResponseType::Answer],
            ChatWorkflowItemType::Render | ChatWorkflowItemType::Message => vec![ChatWorkflowResponseType::Noop],
            ChatWorkflowItemType::Done => Vec::new(),
        };
    }

    pub fn create(
        chat_workflow_id: Uuid,
        item_type: ChatWorkflowItemType,
        message: Option<String>,
        render_type: Option<ChatWorkflowItemRenderType>,
        response_wait: Option<i32>,
    ) -> NewChatWorkflowItem {
        NewChatWorkflowItem {
            chat_workflow_id,
            item_type,
            message,
            render_type,
            response_wait: response_wait.unwrap_or(DEFAULT_RESPONSE_WAIT_IN_SECONDS),
        }
    }

    pub fn find(id: Uuid, conn: &PgConnection) -> Result<ChatWorkflowItem, DatabaseError> {
        chat_workflow_items::table
            .filter(chat_workflow_items::id.eq(id))
            .get_result(conn)
            .to_db_error(ErrorCode::QueryError, "Unable to load chat workflow item")
    }

    pub fn find_chat_workflow_response_by_response_type(
        &self,
        response_type: ChatWorkflowResponseType,
        conn: &PgConnection,
    ) -> Result<ChatWorkflowResponse, DatabaseError> {
        match self
            .responses(conn)?
            .into_iter()
            .find(|r| r.response_type == response_type)
        {
            Some(response) => Ok(response),
            None => DatabaseError::no_results("Could not find a valid response for this response type"),
        }
    }

    pub fn destroy(&self, conn: &PgConnection) -> Result<(), DatabaseError> {
        diesel::delete(self)
            .execute(conn)
            .to_db_error(ErrorCode::DeleteError, "Failed to destroy workflow item")?;

        Ok(())
    }

    pub fn update(
        &self,
        attributes: ChatWorkflowItemEditableAttributes,
        conn: &PgConnection,
    ) -> Result<ChatWorkflowItem, DatabaseError> {
        diesel::update(self)
            .set((attributes, chat_workflow_items::updated_at.eq(dsl::now)))
            .get_result(conn)
            .to_db_error(ErrorCode::UpdateError, "Could not update chat workflow item")
    }
}

impl NewChatWorkflowItem {
    pub fn commit(&self, conn: &PgConnection) -> Result<ChatWorkflowItem, DatabaseError> {
        let chat_workflow_item: ChatWorkflowItem = diesel::insert_into(chat_workflow_items::table)
            .values(self)
            .get_result(conn)
            .to_db_error(ErrorCode::InsertError, "Could not create chat workflow item")?;

        // For message and render types, create initial response automatically to avoid repetitive frontend work
        if chat_workflow_item.item_type == ChatWorkflowItemType::Message
            || chat_workflow_item.item_type == ChatWorkflowItemType::Render
        {
            ChatWorkflowResponse::create(
                chat_workflow_item.id,
                ChatWorkflowResponseType::Noop,
                None,
                None,
                None,
                1,
            )
            .commit(conn)?;
        }

        Ok(chat_workflow_item)
    }
}
