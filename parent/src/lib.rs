

pub struct Postgres;

impl Backend for Postgres {}

pub struct Exists<T>(::std::marker::PhantomData<T>);
pub struct AstPass<DB>(::std::marker::PhantomData<DB>);

pub trait Backend{}
pub trait SupportsDefaultKeyword {}

impl SupportsDefaultKeyword for Postgres {}

pub trait QueryFragment<DB: Backend> {
    fn walk_ast(&self, pass: AstPass<DB>) -> Result<(), ()>;
}



#[derive(Debug, Clone, Copy)]
pub struct BatchInsert<'a, T: 'a, Tab> {
    pub(crate) records: &'a [T],
    _marker: ::std::marker::PhantomData<Tab>,
}

impl<'a, T:'a, Tab, DB> QueryFragment<DB> for BatchInsert<'a, T, Tab>
where DB: SupportsDefaultKeyword + Backend,
{
    fn walk_ast(&self, _pass: AstPass<DB>) -> Result<(), ()> {
        Ok(())
    }
}
