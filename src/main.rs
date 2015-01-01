
extern crate rustc;
extern crate syntax;
extern crate rustc_driver;
extern crate rustc_trans;

use rustc::session::search_paths::SearchPaths;
use rustc::session::config;
use rustc::session;
use rustc::middle::ty;
use rustc_driver::driver;
use std::io::fs::PathExtensions;
use rustc_trans::back::link;
use std::collections::HashMap;
use rustc::util::ppaux;

use syntax::{ast, ast_map, codemap, diagnostic, visit};

struct PrintVisitor<'a>{
    ty_cx: &'a ty::ctxt<'a>,
}

impl<'ast> visit::Visitor<'ast> for PrintVisitor<'ast> {
    fn visit_ty(&mut self, ty: &ast::Ty) {
        //println!("ty is {} span {} to {}", ty.id, ty.span.lo, ty.span.hi);
        //visit::walk_ty(self, ty);
    }
    fn visit_expr(&mut self, expr: &'ast ast::Expr) {
        let ty = ty::node_id_to_type(self.ty_cx, expr.id);
        println!("ty is {} span {} to {}", ppaux::ty_to_string(self.ty_cx, ty),
                 expr.span.lo, expr.span.hi);
        visit::walk_expr(self, expr);
    }
}

fn main() {
    let path = "src/main.rs";
    let input = config::Input::File(Path::new(path));
    let mut search_paths = SearchPaths::new();

    let sessopts = config::Options {
        maybe_sysroot: None,
        crate_types: vec!(config::CrateTypeRlib),
        search_paths: search_paths,
//        target_triple: triple.unwrap_or(config::host_triple().to_string()),
        ..config::basic_options().clone()
    };

    let codemap = codemap::CodeMap::new();
    let diagnostic_handler = diagnostic::default_handler(diagnostic::Auto, None);
    let span_diagnostic_handler =
        diagnostic::mk_span_handler(diagnostic_handler, codemap);
    let sess = session::build_session_(sessopts,
                                       Some(Path::new(path)),
									   span_diagnostic_handler);
	let cfg = config::build_configuration(&sess);
	let krate = driver::phase_1_parse_input(&sess, cfg, &input);
    let name = link::find_crate_name(Some(&sess), krate.attrs.as_slice(),
                                     &input);
    let krate = driver::phase_2_configure_and_expand(&sess, krate, name.as_slice(), None).unwrap();

    let mut forest = ast_map::Forest::new(krate);
    let ast_map = driver::assign_node_ids_and_map(&sess, &mut forest);

    let arenas = ty::CtxtArenas::new();
    let ty::CrateAnalysis {
        exported_items, public_items, ty_cx, ..
    } = driver::phase_3_run_analysis_passes(sess, ast_map, &arenas, name);
    let mut vis = PrintVisitor{ty_cx: &ty_cx};
    visit::walk_crate(&mut vis, ty_cx.map.krate());
}
