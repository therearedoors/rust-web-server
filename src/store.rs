use crate::types::{
    answer::{Answer, AnswerId, NewAnswer},
    question::{Question, QuestionId, NewQuestion},
};
use handle_errors::Error;
use sqlx::postgres::{PgPool, PgPoolOptions, PgRow};
use sqlx::Row;

#[derive(Debug, Clone)]
pub struct Store {
    pub connection: PgPool,
}

impl Store {
    pub async fn new(db_url: &str) -> Self {
        let db_pool = match PgPoolOptions::new()
            .max_connections(5)
            .connect(db_url)
            .await
        {
            Ok(pool) => pool,
            Err(e) => panic!("Error connecting to database: {}", e),
        };
        Store { connection: db_pool }
    }

    pub async fn get_questions(&self, limit: Option<usize>, offset: usize) -> Result<Vec<Question>, sqlx::Error> {
        match sqlx::query("SELECT * FROM questions LIMIT $1 OFFSET $2")
            .bind(limit.unwrap_or(0) as i32)
            .bind(offset as i32)
            .map(|row: PgRow| Question {
                id: QuestionId(row.get("id")),
                title: row.get("title"),
                content: row.get("content"),
                tags: row.get("tags"),
            })
            .fetch_all(&self.connection)
            .await {
                Ok(questions) => Ok(questions),
                Err(e) => {
                    tracing::event!(tracing::Level::ERROR, "Error getting questions: {:?}", e);
                    Err(e)
                }
            }
    }

    pub async fn add_question(
        &self,
        new_question: NewQuestion,
    ) -> Result<Question, sqlx::Error> {
        match sqlx::query(
            "INSERT INTO questions (title, content, tags) VALUES ($1, $2, $3)",
        )
        .bind(new_question.title)
        .bind(new_question.content)
        .bind(new_question.tags)
        .map(|row: PgRow| Question {
            id: QuestionId(row.get("id")),
            title: row.get("title"),
            content: row.get("content"),
            tags: row.get("tags"),
        })
        .fetch_one(&self.connection)
        .await
        {
            Ok(question) => Ok(question),
            Err(e) => Err(e),
        }
    }

    pub async fn update_question(
        &self,
        question: Question,
        question_id: i32
    ) -> Result<Question, sqlx::Error> {
        match sqlx::query(
        "UPDATE questions SET title = $1, content = $2, tags = $3 WHERE id = $4 RETURNING id, title, content, tags"
    )
    .bind(question.title)
    .bind(question.content)
    .bind(question.tags)
    .bind(question_id)
    .map(|row: PgRow| Question {
        id: QuestionId(row.get("id")),
        title: row.get("title"),
        content: row.get("content"),
        tags: row.get("tags"),
    })
    .fetch_one(&self.connection)
    .await {
        Ok(question) => Ok(question),
        Err(e) => Err(e),
        }
    }

    pub async fn delete_question(
        &self, 
        question_id: i32
     ) -> Result<bool, sqlx::Error> {
        match sqlx::query("DELETE FROM questions WHERE id = $1")
        .bind(question_id)
        .execute(&self.connection)
        .await {
            Ok(_) => Ok(true),
            Err(e) => Err(e),
        }
     }

     pub async fn get_answers(&self, limit: Option<usize>, offset: usize) -> Result<Vec<Answer>, sqlx::Error> {
        match sqlx::query("SELECT * FROM answers LIMIT $1 OFFSET $2")
            .bind(limit.unwrap_or(0) as i32)
            .bind(offset as i32)
            .map(|row: PgRow| Answer {
                id: AnswerId(row.get("id")),
                content: row.get("content"),
                question_id: QuestionId(row.get("question_id")),
            })
            .fetch_all(&self.connection)
            .await {
                Ok(answers) => Ok(answers),
                Err(e) => {
                    tracing::event!(tracing::Level::ERROR, "Error getting answers: {:?}", e);
                    Err(e)
                }
            }
    }

     pub async fn add_answer(
        &self,
        new_answer: NewAnswer
        ) -> Result<Answer, Error> {
            match sqlx::query(
            "INSERT INTO answers (content, question_id) VALUES ($1, $2)"
            )
            .bind(new_answer.content)
            .bind(new_answer.question_id.0)
            .map(|row: PgRow| Answer {
                id: AnswerId(row.get("id")),
                content: row.get("content"),
                question_id: QuestionId(row.get("question_id")),
            })
            .fetch_one(&self.connection)
            .await {
                Ok(answer) => Ok(answer),
                Err(e) => {
                    tracing::event!(tracing::Level::ERROR, "{:?}", e);
                    Err(Error::DatabaseQueryError(e))
                },
            }
    }
     

}
