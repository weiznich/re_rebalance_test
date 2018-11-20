#![feature(re_rebalance_coherence)]
extern crate parent;

use parent::*;

pub struct Oracle;
impl Backend for Oracle {}

pub struct OracleFoo;


impl QueryFragment<Oracle> for OracleFoo {
    fn walk_ast(&self, _pass: AstPass<Oracle>) -> Result<(), ()>{
        Ok(())
    }
}

impl<'a, T:'a, Tab> QueryFragment<Oracle> for BatchInsert<'a, T, Tab> {
    fn walk_ast(&self, _pass: AstPass<Oracle>) -> Result<(), ()> {
        Ok(())
    }
}

// impl<T> QueryFragment<Oracle> for T {
//     fn walk_ast(&self, _pass: AstPass<Oracle>) -> Result<(), ()> {
//         Ok(())
//     }
//  }
