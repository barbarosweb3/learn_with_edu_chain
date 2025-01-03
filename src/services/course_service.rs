use crate::blockchain::BlockchainService;
use crate::models::{course::Course, certificate::Certificate};
use sqlx::PgPool;
use uuid::Uuid;
use std::error::Error;
use chrono::Utc;
use ethers::core::types::U256;

pub struct CourseService {
    blockchain: BlockchainService,
    db: PgPool,
}

impl CourseService {
    pub fn new(blockchain: BlockchainService, db: PgPool) -> Self {
        Self { blockchain, db }
    }

    pub async fn create_course(
        &self,
        title: String,
        metadata: String,
        price: i64,
        instructor_address: String,
    ) -> Result<Course, Box<dyn Error>> {
        let blockchain_id = Uuid::new_v4().to_string();

        let price_u256: U256 = price.try_into()?;
        self.blockchain
            .create_course(blockchain_id.clone(), metadata.clone(), price_u256)
            .await?;

        let course = Course {
            id: Uuid::new_v4(),
            blockchain_id,
            instructor_address,
            title,
            metadata,
            price,
            is_active: true,
            created_at: Utc::now(),
        };

        sqlx::query!(
            r#"
            INSERT INTO courses
                (id, blockchain_id, instructor_address, title, metadata, price, is_active, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
            course.id,
            course.blockchain_id,
            course.instructor_address,
            course.title,
            course.metadata,
            course.price,
            course.is_active,
            course.created_at
        )
            .execute(&self.db)
            .await?;

        Ok(course)
    }

    pub async fn issue_certificate(
        &self,
        course_id: Uuid,
        student_address: String,
    ) -> Result<Certificate, Box<dyn Error>> {
        let course = sqlx::query_as!(
            Course,
            r#"
            SELECT id, blockchain_id, instructor_address, title, metadata,
                   price, is_active, created_at as "created_at!"
            FROM courses WHERE id = $1
            "#,
            course_id
        )
            .fetch_one(&self.db)
            .await?;

        let tx = self.blockchain
            .issue_certificate(
                student_address.parse()?,
                course.blockchain_id,
            )
            .await?;

        let certificate = Certificate {
            id: Uuid::new_v4(),
            student_address,
            course_id,
            blockchain_hash: tx.transaction_hash.to_string(),
            issued_at: Utc::now(),
        };

        sqlx::query!(
            r#"
            INSERT INTO certificates
                (id, student_address, course_id, blockchain_hash, issued_at)
            VALUES ($1, $2, $3, $4, $5)
            "#,
            certificate.id,
            certificate.student_address,
            certificate.course_id,
            certificate.blockchain_hash,
            certificate.issued_at,
        )
            .execute(&self.db)
            .await?;

        Ok(certificate)
    }
}