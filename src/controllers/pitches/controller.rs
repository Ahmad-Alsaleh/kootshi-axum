use crate::{
    controllers::pitches::errors::PitchControllerError,
    models::{ModelManager, tables::Pitch},
};
use uuid::Uuid;

pub struct PitchController;

impl PitchController {
    pub async fn get_all_pitches(
        model_manager: &ModelManager,
    ) -> Result<Vec<Pitch>, PitchControllerError> {
        sqlx::query_as(
            "
            SELECT
                id, owner_id, display_name, sport
            FROM
                pitches
            ",
        )
        .fetch_all(model_manager.db())
        .await
        .map_err(PitchControllerError::Sqlx)
    }

    pub async fn get_pitches_by_business_id(
        business_id: Uuid,
        model_manager: &ModelManager,
    ) -> Result<Vec<Pitch>, PitchControllerError> {
        sqlx::query_as(
            "
            SELECT
                id, owner_id, display_name, sport
            FROM
                pitches
            WHERE
                owner_id = $1
            ",
        )
        .bind(business_id)
        .fetch_all(model_manager.db())
        .await
        .map_err(PitchControllerError::Sqlx)
    }
}
