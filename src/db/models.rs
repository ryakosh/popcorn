use crate::chrono::NaiveDate;
use super::schema::users;

#[table_name = "users"]
#[derive(Queryable, Insertable)]
pub struct User<'u> {
  pub id: &'u str,
  pub email: &'u str,
  pub pwd: &'u str,
}

#[derive(Queryable)]
pub struct Artist<'a> {
  pub id: i32,
  pub first_name: &'a str,
  pub last_name: &'a str,
  pub gender: &'a str,
}

#[derive(Queryable)]
pub struct Director<'d> {
  pub id: i32,
  pub first_name: &'d str,
  pub last_name: &'d str,
  pub gender: &'d str,
}

#[derive(Queryable)]
pub struct Writer<'w> {
  pub id: i32,
  pub first_name: &'w str,
  pub last_name: &'w str,
  pub gender: &'w str,
}

#[derive(Queryable)]
pub struct Movie<'m> {
  id: i32,
  title: &'m str,
  description: &'m str,
  genres: Vec<&'m str>,
  languages: Vec<&'m str>,
  release_country: &'m str,
  release_date: NaiveDate,
  duration: i16,
  directors: Vec<i32>,
  wirters: Vec<i32>,
  stars: Vec<i32>,
}