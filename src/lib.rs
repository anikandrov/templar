#[macro_use]
extern crate lazy_static;

pub(crate) use error::*;
use std::{collections::HashMap, sync::Arc};
pub use {
    self::{
        context::{Context, SharedContext, StandardContext},
        extensions::{Filter, Function, TemplarResult},
        nodes::Node,
        templar::*,
        template::Template,
    },
    unstructured::Document,
};

pub mod error;

#[cfg(test)]
mod test;

mod context;
mod extensions;
mod nodes;
mod parser;
mod templar;
mod template;
