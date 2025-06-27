#![feature(rustc_private)]
#![feature(let_chains)]
#![warn(unused_extern_crates)]

extern crate rustc_hir;

use rustc_lint::LateLintPass;
use rustc_hir::{Expr, ExprKind};
use clippy_utils::diagnostics::span_lint_and_help;

dylint_linting::declare_late_lint! {
    /// ### What it does
    /// Detects multiple if-else statements that could impact energy consumption
    ///
    /// ### Why is this bad?
    /// Multiple if-else statements can lead to increased CPU usage and energy consumption,
    /// especially when they could be simplified using pattern matching or lookup tables.
    ///
    /// ### Example
    ///
    /// ```rust
    /// fn handle_value(x: i32) -> String {
    ///     if x == 1 {
    ///         "one".to_string()
    ///     } else if x == 2 {
    ///         "two".to_string()
    ///     } else if x == 3 {
    ///         "three".to_string()
    ///     } else {
    ///         "other".to_string()
    ///     }
    /// }
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust
    /// fn handle_value(x: i32) -> String {
    ///     match x {
    ///         1 => "one".to_string(),
    ///         2 => "two".to_string(),
    ///         3 => "three".to_string(),
    ///         _ => "other".to_string(),
    ///     }
    /// }
    /// ```
    pub GCI2,
    Warn,
    "avoid multiple if-else statement to reduce energy consumption"
}

impl<'tcx> LateLintPass<'tcx> for Gci2 {
    fn check_expr(&mut self, cx: &rustc_lint::LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if let ExprKind::If(_, _, Some(else_expr)) = expr.kind {
            // Count the number of chained if-else statements
            let else_if_count = count_else_if_chain(else_expr);
            
            // Trigger lint if we have 2 or more else-if statements
            if else_if_count >= 2 {
                span_lint_and_help(
                    cx,
                    GCI2,
                    expr.span,
                    format!("found {} chained if-else statements", else_if_count + 1),
                    None,
                    "consider using pattern matching (match) or lookup tables to improve energy efficiency",
                );
            }
        }
    }
}

/// Count the number of chained else-if statements
fn count_else_if_chain(expr: &Expr<'_>) -> usize {
    match expr.kind {
        ExprKind::If(_, _, Some(else_expr)) => 1 + count_else_if_chain(else_expr),
        ExprKind::If(_, _, None) => 1,
        _ => 0,
    }
}

#[test]
fn ui() {
    dylint_testing::ui_test(env!("CARGO_PKG_NAME"), "ui");
}
