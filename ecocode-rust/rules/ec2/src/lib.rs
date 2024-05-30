#![feature(rustc_private)]
#![feature(let_chains)]
#![warn(unused_extern_crates)]

extern crate rustc_arena;
extern crate rustc_ast;
extern crate rustc_ast_pretty;
extern crate rustc_attr;
extern crate rustc_data_structures;
extern crate rustc_errors;
extern crate rustc_hir;
extern crate rustc_hir_pretty;
extern crate rustc_index;
extern crate rustc_infer;
extern crate rustc_lexer;
extern crate rustc_middle;
extern crate rustc_mir_dataflow;
extern crate rustc_parse;
extern crate rustc_span;
extern crate rustc_target;
extern crate rustc_trait_selection;

use rustc_hir::{Expr, ExprKind, Block, Stmt, StmtKind};
use rustc_lint::{LateContext, LateLintPass};
use clippy_utils::diagnostics::span_lint_and_help;

dylint_linting::declare_late_lint! {
    /// ### What it does
    ///
    /// ### Why is this bad?
    ///
    /// ### Known problems
    /// Remove if none.
    ///
    /// ### Example
    /// ```rust
    /// // example code where a warning is issued
    /// ```
    /// Use instead:
    /// ```rust
    /// // example code that does not raise a warning
    /// ```
    pub EC2,
    Warn,
    "Avoid multiple if-else statement"
}

impl<'tcx> LateLintPass<'tcx> for Ec2 {

    fn check_block(&mut self, cx: &LateContext<'tcx>, block: &'tcx Block<'tcx>) {
        for stmt in block.stmts {
            self.check_stmt(cx, stmt);
        }
    }

    fn check_stmt(&mut self, cx: &LateContext<'tcx>, stmt: &'tcx Stmt<'tcx>) {
        if let StmtKind::Expr(expr) | StmtKind::Semi(expr) = stmt.kind {
            self.check_expr(cx, expr);
        }
    }
    
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if let ExprKind::If(_, _, Some(else_expr)) = &expr.kind {
            if self.is_nested_if_else(else_expr) {
                span_lint_and_help(
                    cx,
                    EC2,
                    expr.span,
                    "ECOCODE : Avoid multiple if-else statements",
                    None,
                    "Consider refactoring to a match statement or another construct"
                );
            }
        }
    }
}

impl<'tcx> Ec2 {
    fn is_nested_if_else(&self, expr: &'tcx Expr<'tcx>) -> bool {
        match expr.kind {
            ExprKind::If(_, _, Some(else_expr)) => {
                if let ExprKind::If(_, _, _) = else_expr.kind {
                    true
                } else {
                    self.is_nested_if_else(else_expr)
                }
            },
            _ => false,
        }
    }
}

#[test]
fn ui() {
    dylint_testing::ui_test(
        env!("CARGO_PKG_NAME"),
        &std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("ui"),
    );
}
