
extern crate rustc;
extern crate syntax;
extern crate rustc_driver;
extern crate rustc_trans;

use rustc::session::config;
use rustc::session;
use rustc::middle::ty;
use rustc_driver::driver;
use std::io::fs::PathExtensions;
use rustc_trans::back::link;

use syntax::{ast, ast_map, codemap, diagnostic};

fn main() {
    let path = "src/main.rs";
    let input = config::Input::File(Path::new(path));

    let sessopts = config::Options {
        maybe_sysroot: None,
        crate_types: vec!(config::CrateTypeRlib),
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
    
}
