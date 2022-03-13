
pub mod page_pool;
pub mod page_template;
pub mod page;


pub type PagePool = page_pool::PagePool;
pub type PageKind<'a> = page_pool::PageKind<'a>;
