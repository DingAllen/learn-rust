#![allow(dead_code)]

mod example;
mod homework;

use crate::example::linked_list::List;
use crate::example::vector::{iterate_vector_example, read_vector_item_example};

fn main() {
    read_vector_item_example();
    iterate_vector_example();
}