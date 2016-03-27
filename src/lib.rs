#![feature(custom_derive, custom_attribute, plugin)]
#![plugin(diesel_codegen)]

#[macro_use]
extern crate diesel;

infer_schema!("postgres://localhost/enum_type_test");

fn main() {
}
