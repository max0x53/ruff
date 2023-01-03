use rustpython_ast::{Constant, Expr, ExprKind, Stmt, StmtKind};

use crate::ast::types::Range;
use crate::autofix::Fix;
use crate::checkers::ast::Checker;
use crate::registry::{Check, CheckCode, CheckKind};

/// PIE790
pub fn no_unnecessary_pass(checker: &mut Checker, body: &[Stmt]) {
    if body.len() > 1 {
        // This only catches the case in which a docstring makes a `pass` statement redundant.
        // Consider removing all `pass` statements instead.
        let docstring_stmt = &body[0];
        let pass_stmt = &body[1];
        let StmtKind::Expr { value } = &docstring_stmt.node else {
            return;
        };
        if matches!(
            value.node,
            ExprKind::Constant {
                value: Constant::Str(..),
                ..
            }
        ) {
            if matches!(pass_stmt.node, StmtKind::Pass) {
                let mut check =
                    Check::new(CheckKind::NoUnnecessaryPass, Range::from_located(pass_stmt));
                if checker.patch(&CheckCode::PIE790) {
                    check.amend(Fix::deletion(
                        pass_stmt.location,
                        pass_stmt.end_location.unwrap(),
                    ));
                }
                checker.add_check(check);
            }
        }
    }
}

/// PIE807
pub fn prefer_list_builtin(checker: &mut Checker, expr: &Expr) {
    let ExprKind::Lambda { args, body } = &expr.node else {
        unreachable!("Expected ExprKind::Lambda");
    };
    if args.args.is_empty() {
        if let ExprKind::List { elts, .. } = &body.node {
            if elts.is_empty() {
                let mut check = Check::new(CheckKind::PreferListBuiltin, Range::from_located(expr));
                if checker.patch(&CheckCode::PIE807) {
                    check.amend(Fix::replacement(
                        "list".to_string(),
                        expr.location,
                        expr.end_location.unwrap(),
                    ));
                }
                checker.add_check(check);
            }
        }
    }
}