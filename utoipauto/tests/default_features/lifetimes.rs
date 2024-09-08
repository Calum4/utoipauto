use utoipa::ToSchema;

#[derive(ToSchema)]
pub struct LifetimeStructSchema<'a> {
    foo: &'a str,
}

#[derive(ToSchema)]
pub enum LifetimeEnumSchema<'a> {
    Foo(&'a str),
}
