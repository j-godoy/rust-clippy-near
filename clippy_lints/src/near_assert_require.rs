// use rustc_lint::{LateContext, LateLintPass};
// use clippy_utils::macros::root_macro_call_first_node;
// use rustc_hir::Expr;
// use rustc_session::{declare_lint_pass, declare_tool_lint};
// use rustc_span::sym;

use clippy_utils::macros::{root_macro_call_first_node};
use clippy_utils::{diagnostics::span_lint_and_note};
use rustc_hir::{Expr};
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::{declare_lint_pass, declare_tool_lint};

declare_clippy_lint! {
    /// ### What it does
    /// This lint warns about using assert macros from std rust.
    /// 
    /// ### Why is this bad?
    /// In Near contracts is better to use the require! macro
    /// 
    /// ### Example
    /// ```rust
    /// // assert!(x > 5);
    /// ```
    /// Use instead:
    /// ```rust
    /// require(x > 5);
    /// ```
    #[clippy::version = "1.64.0"]
    pub NEAR_ASSERT_REQUIRE,
    pedantic,
    "Please use require instead of assert! in NEAR projects"
}

declare_lint_pass!(NearAssertRequire => [NEAR_ASSERT_REQUIRE]);

impl<'tcx> LateLintPass<'tcx> for NearAssertRequire {
    fn check_expr(&mut self, cx: &LateContext<'_>, e: &'tcx Expr<'_>) {
        let Some(macro_call) = root_macro_call_first_node(cx, e) else { return };
        // let is_assert_macro = match cx.tcx.get_diagnostic_name(macro_call.def_id) {
        //     Some(sym::debug_assert_macro) => false,
        //     Some(sym::assert_macro) => true,
        //     _ => return,
        // };
        // if !(is_assert_macro) {
        //     return;
        // }
        let macro_name = cx.tcx.item_name(macro_call.def_id);
        if !matches!( macro_name.as_str(), "assert") {
            return;
        }
        let macro_name = macro_name.as_str();
        print!("EL NOMBRE DE LA MACRO ES: {}.",macro_name);
        span_lint_and_note(
            cx,
            NEAR_ASSERT_REQUIRE,
            macro_call.span,
            &format!("used `{}!` from standard rust", macro_name),
            Some(macro_call.span),
            "use `require` for near contracts",
        );
    }
}
