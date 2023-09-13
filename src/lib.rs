use lalrpop_util::lalrpop_mod;
pub mod ast;
pub mod semantic_analysis;
lalrpop_mod!(pub grammar);

#[macro_export]
macro_rules! gen_err {
    ($errs:expr, $err_typ:expr, $err:expr) => {{
        $errs.push($err);
        $err_typ
    }};
}
