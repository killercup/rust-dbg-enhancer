use crate::parser::Rule;
use pest_ast::FromPest;

#[derive(Debug, FromPest, Clone, PartialEq)]
#[pest_ast(rule(Rule::source))]
pub struct Source<'src> {
    pub item: Item<'src>,
    eoi: EOI,
}

#[derive(Debug, FromPest, Clone, PartialEq)]
#[pest_ast(rule(Rule::item))]
pub enum Item<'src> {
    Array(Array<'src>),
    Structure(Structure<'src>),
    String(StringContent<'src>),
    Ident(Ident<'src>),
}

#[derive(Debug, FromPest, Clone, PartialEq)]
#[pest_ast(rule(Rule::array))]
pub struct Array<'src> {
    pub items: Vec<Item<'src>>,
}

#[derive(Debug, FromPest, Clone, PartialEq)]
#[pest_ast(rule(Rule::structure))]
pub struct Structure<'src> {
    pub name: Ident<'src>,
    pub fields: Vec<StructureField<'src>>,
}

#[derive(Debug, FromPest, Clone, PartialEq)]
#[pest_ast(rule(Rule::structure_field))]
pub struct StructureField<'src> {
    pub name: Ident<'src>,
    pub value: Item<'src>,
}

#[derive(Debug, FromPest, Clone, PartialEq)]
#[pest_ast(rule(Rule::ident))]
pub struct Ident<'src> {
    #[pest_ast(outer(with(span_into_str)))]
    pub content: &'src str,
}

#[derive(Debug, FromPest, Clone, PartialEq)]
#[pest_ast(rule(Rule::string_content))]
pub struct StringContent<'src> {
    #[pest_ast(outer(with(span_into_str)))]
    pub content: &'src str,
}

#[derive(Debug, FromPest, Clone, PartialEq)]
#[pest_ast(rule(Rule::EOI))]
pub(crate) struct EOI;

pub(crate) fn span_into_str(span: pest::Span) -> &str {
    span.as_str()
}
