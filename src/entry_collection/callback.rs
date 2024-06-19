use crate::utils;
use log::debug;
use rustc_driver::Compilation;
use rustc_interface::interface;
use rustc_interface::Queries;
use rustc_middle::ty::TyCtxt;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub struct EntryCollectorCallbacks {
    // If we are compiling a dependency crate, only collect FFI functions
    // If we are compiling a top crate, collect both FFI functions and public functions
    is_dependency: bool,
}

impl EntryCollectorCallbacks {
    pub fn new() -> Self {
        if std::env::var_os("FFI_CHECKER_IS_DEPS").is_some() {
            Self {
                is_dependency: true,
            }
        } else {
            Self {
                is_dependency: false,
            }
        }
    }
}

impl rustc_driver::Callbacks for EntryCollectorCallbacks {
    /// Called after analysis. Return value instructs the compiler whether to
    /// continue the compilation afterwards (defaults to `Compilation::Continue`)
    fn after_analysis<'compiler, 'tcx>(
        &mut self,
        compiler: &'compiler interface::Compiler,
        queries: &'tcx Queries<'tcx>,
    ) -> Compilation {
        queries
            .global_ctxt()
            .unwrap()
            .enter(|tcx| self.run_analysis(compiler, tcx));
        Compilation::Continue
    }
}


impl EntryCollectorCallbacks {
    fn run_analysis<'tcx, 'compiler>(
        &mut self,
        _compiler: &'compiler interface::Compiler,
        tcx: TyCtxt<'tcx>,
    ) {
        // Skip some crates that we are not interested in
        let crate_name =
            utils::get_arg_flag_value("--crate-name").expect("Argument --crate-name not found");
        let should_skip = vec!["build_script_build"];
        if should_skip.contains(&crate_name.as_str()) {
            return;
        }

        // Public functions and FFI functions are globally visible, so their names should be unique
        let mut pub_funcs = HashSet::new();
        let mut ffi_funcs = HashSet::new();

        // If the crate is a binary, add the entry function
        if let Some((entry_def_id, _)) = tcx.entry_fn(()) {
            let item_name = tcx.item_name(entry_def_id).to_ident_string();
            pub_funcs.insert(item_name);
        }

        // Initialize global analysis context
        let hir = tcx.hir();
        for item_id in hir.items() {
        let item = hir.item(item_id);
            // If it is a top crate, collect all the public functions/methods
        if !self.is_dependency {
            match &item.kind {
                rustc_hir::ItemKind::Fn { .. } => {
                    if tcx.visibility(item.owner_id).is_public() {
                        debug!("Public Fn: {:?}, {:?}", item.owner_id, tcx.def_path_str(item.owner_id));
                        pub_funcs.insert(tcx.def_path_str(item.owner_id));
                    }
                }
                rustc_hir::ItemKind::Impl(impl_inner) => {
                    for item_ref in impl_inner.items {
                        if matches!(item_ref.kind, rustc_hir::AssocItemKind::Fn { .. }) {
                            // 通过 id 获取 ImplItem 的可见性
                            let impl_item_id = item_ref.id;
                            let impl_item = tcx.hir().impl_item(impl_item_id);
                            if tcx.visibility(impl_item.owner_id).is_public() {
                                let defpath = tcx.def_path(impl_item.owner_id.def_id.to_def_id());
                                debug!(
                                    "Public Impl Fn: {:?}, {:?}, {}",
                                    item_ref.id,
                                    impl_item.ident,
                                    defpath.to_filename_friendly_no_crate()
                                );
                                pub_funcs.insert(tcx.def_path_str(impl_item.owner_id));
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        if let rustc_hir::ItemKind::ForeignMod { abi: _, items } = hir.item(item_id).kind {
            for itemref in items {
                debug!("FFI: {:?}, {:?}", itemref.id, itemref.ident);
                ffi_funcs.insert(String::from(&*(itemref.ident.as_str())));
        
                // The visibility of a foreign function is stored in `ForeignItem`, so we get it through its id
                let foreign_item_id = itemref.id;
                let foreign_item = hir.foreign_item(foreign_item_id);
                if tcx.visibility(foreign_item.owner_id).is_public() {
                    debug!("Public FFI Fn: {:?}, {:?}", itemref.id, itemref.ident);
                    pub_funcs.insert(String::from(&*(itemref.ident.as_str())));
                }
            }
        }
        
        }

        // If we collect some entry points and FFI functions, write them to files
        // Note that to get more results, we only consider whether entry points are found,
        // even if there is no FFI called, we still continue the analysis
        if !pub_funcs.is_empty() {
            // Create directory `entry_points` if not exists
            if !Path::new("target/entry_points").exists() {
                std::fs::create_dir_all("target/entry_points")
                    .expect("Failed to create `entry_points` directory");
            }

            let file_path = Path::new("target/entry_points").join(crate_name);

            if !file_path.exists() {
                let mut file = File::create(file_path).expect("Failed to create file");
                for entry in pub_funcs {
                    file.write_all(format!("Entry: {}\n", entry).as_bytes())
                        .unwrap();
                }
                for ffi in ffi_funcs {
                    file.write_all(format!("FFI: {}\n", ffi).as_bytes())
                        .unwrap();
                }
            }
        }
    }
}
