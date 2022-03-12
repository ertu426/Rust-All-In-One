use actix_web::web;
use actix_web::web::Json;
use serde::{Deserialize, Serialize};
use crate::errors::MyError;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Teacher {
    pub id: i32,
    pub name: String,
    pub picture_url: String,
    pub profile: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CreateTeacher {
    pub name: String,
    pub picture_url: String,
    pub profile: String,
}

impl TryFrom<web::Json<CreateTeacher>> for CreateTeacher {
    type Error = MyError;

    fn try_from(teacher: Json<CreateTeacher>
    ) -> Result<Self, Self::Error> {
        Ok(CreateTeacher {
            name: teacher.name.clone(),
            picture_url: teacher.picture_url.clone(),
            profile: teacher.profile.clone()
        })
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct UpdateTeacher {
    pub name: Option<String>,
    pub picture_url: Option<String>,
    pub profile: Option<String>,
}

impl TryFrom<web::Json<UpdateTeacher>> for UpdateTeacher {
    type Error = MyError;

    fn try_from(teacher: Json<UpdateTeacher>
    ) -> Result<Self, Self::Error> {
        Ok(UpdateTeacher {
            name: teacher.name.clone(),
            picture_url: teacher.picture_url.clone(),
            profile: teacher.profile.clone()
        })
    }
}