use heraclitus_compiler::prelude::*;
use crate::utils::metadata::{ParserMetadata, TranslateMetadata};
use crate::modules::expression::expr::Expr;
use crate::translate::module::TranslateModule;
use crate::modules::variable::{
    init::VariableInit,
    set::VariableSet
};
use crate::modules::command::statement::CommandStatement;
use crate::handle_types;

#[derive(Debug)]
enum StatementType {
    Expr(Expr),
    VariableInit(VariableInit),
    VariableSet(VariableSet),
    CommandStatement(CommandStatement)
}

#[derive(Debug)]
pub struct Statement {
    value: Option<StatementType>
}

impl Statement {
    handle_types!(StatementType, [
        // Variables
        VariableInit, VariableSet,
        // Command
        CommandStatement,
        // Expression
        Expr
    ]);

    // Get result out of the provided module and save it in the internal state
    fn get<M,S>(&mut self, meta: &mut M, mut module: S, cb: impl Fn(S) -> StatementType) -> SyntaxResult
    where
        M: Metadata,
        S: SyntaxModule<M>
    {
        match syntax(meta, &mut module) {
            Ok(()) => {
                self.value = Some(cb(module));
                Ok(())    
            }
            Err(details) => Err(details)
        }
    }
}

impl SyntaxModule<ParserMetadata> for Statement {
    syntax_name!("Statement");

    fn new() -> Self {
        Statement {
            value: None
        }
    }

    fn parse(&mut self, meta: &mut ParserMetadata) -> SyntaxResult {
        let mut error = None;
        let statements = self.get_modules();
        for statement in statements {
            match self.parse_match(meta, statement) {
                Ok(()) => return Ok(()),
                Err(details) => error = Some(details)
            }
        }
        Err(error.unwrap())
    }
}

impl TranslateModule for Statement {
    fn translate(&self, meta: &mut TranslateMetadata) -> String {
        let translated = self.translate_match(meta, &self.value.as_ref().unwrap());
        if translated.starts_with("$(") {
            format!("echo {} > /dev/null 2>&1", translated)
        } else { translated }
    }
}