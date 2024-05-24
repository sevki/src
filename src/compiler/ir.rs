#![allow(clippy::needless_borrow)]

use salsa::*;
use std::{
    array::IntoIter,
    collections::BTreeMap,
    path::Iter,
    sync::{Arc, Mutex},
};

use crate::{parser::ast};

#[salsa::tracked]
pub struct Program {
    #[return_ref]
    pub modul: Vec<Function>,
    #[return_ref]
    pub symbols: BTreeMap<Mangled, Symbol>,
}

#[salsa::tracked]
pub struct Function {
    #[return_ref]
    pub name: String,

    #[return_ref]
    pub body: Vec<Box<Function>>,

    #[return_ref]
    pub effects: Vec<InternedEffect>,
}

#[salsa::interned]
pub struct InternedEffect {
    pub effect: String,
}

#[salsa::interned]
pub struct Symbol {
    #[return_ref]
    pub symbol: Mangled,
}

#[salsa::tracked]
pub struct EffectDef {
    #[return_ref]
    pub effect: ast::EffectDef,
}

#[salsa::tracked]
pub struct Import {
    #[return_ref]
    pub imports: Vec<String>,
    #[return_ref]
    pub module: String,
}

#[salsa::interned]
pub struct Mangled {
    #[return_ref]
    pub mangled: String,
}

