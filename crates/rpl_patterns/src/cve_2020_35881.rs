#[allow(non_snake_case)]
pub mod const_const_Transmute_ver {
    use rpl_context::PatCtxt;
    use rpl_mir::pat::MirPatternBuilder;
    use rustc_hir as hir;
    use rustc_hir::def_id::LocalDefId;
    use rustc_hir::intravisit::{self, Visitor};
    use rustc_middle::hir::nested_filter::All;
    use rustc_middle::ty::TyCtxt;
    use rustc_span::Span;

    use rpl_mir::{pat, CheckMirCtxt};

    #[instrument(level = "info", skip(tcx, pcx))]
    pub fn check_item(tcx: TyCtxt<'_>, pcx: PatCtxt<'_>, item_id: hir::ItemId) {
        let item = tcx.hir().item(item_id);
        // let def_id = item_id.owner_id.def_id;
        let mut check_ctxt = CheckFnCtxt { tcx, pcx };
        check_ctxt.visit_item(item);
    }

    struct CheckFnCtxt<'pcx, 'tcx> {
        tcx: TyCtxt<'tcx>,
        pcx: PatCtxt<'pcx>,
    }

    impl<'tcx> Visitor<'tcx> for CheckFnCtxt<'_, 'tcx> {
        type NestedFilter = All;
        fn nested_visit_map(&mut self) -> Self::Map {
            self.tcx.hir()
        }

        #[instrument(level = "debug", skip_all, fields(?item.owner_id))]
        fn visit_item(&mut self, item: &'tcx hir::Item<'tcx>) -> Self::Result {
            match item.kind {
                hir::ItemKind::Trait(hir::IsAuto::No, ..) | hir::ItemKind::Impl(_) | hir::ItemKind::Fn(..) => {},
                _ => return,
            }
            intravisit::walk_item(self, item);
        }

        fn visit_fn(
            &mut self,
            _kind: intravisit::FnKind<'tcx>,
            _decl: &'tcx hir::FnDecl<'tcx>,
            _body_id: hir::BodyId,
            _span: Span,
            def_id: LocalDefId,
        ) -> Self::Result {
            if self.tcx.visibility(def_id).is_public() && self.tcx.is_mir_available(def_id) {
                let body = self.tcx.optimized_mir(def_id);
                #[allow(irrefutable_let_patterns)]
                if let mut patterns = MirPatternBuilder::new(self.pcx)
                    && let pattern = pattern_wrong_assumption_of_fat_pointer_layout(&mut patterns)
                    && let Some(matches) = CheckMirCtxt::new(self.tcx, body, &patterns.build()).check()
                    && let Some(ptr_transmute) = matches[pattern.ptr_transmute]
                    && let span1 = ptr_transmute.span_no_inline(body)
                    && let Some(data_ptr_get) = matches[pattern.data_ptr_get]
                    && let span2 = data_ptr_get.span_no_inline(body)
                {
                    self.tcx
                        .dcx()
                        .emit_err(crate::errors::WrongAssumptionOfFatPointerLayout {
                            ptr_transmute: span1,
                            data_ptr_get: span2,
                        });
                }
            }
        }
    }

    struct WrongAssumptionOfFatPointerLayout {
        ptr_transmute: pat::Location,
        data_ptr_get: pat::Location,
    }

    #[rpl_macros::mir_pattern]
    fn pattern_wrong_assumption_of_fat_pointer_layout(
        patterns: &mut pat::MirPatternBuilder<'_>,
    ) -> WrongAssumptionOfFatPointerLayout {
        mir! {
            meta!{$T:ty}

            let ptr: *const $T = _;
            // _4 = &_1;
            let ref_to_ptr: &*const $T = &ptr;
            // _3 = &raw const (*_4);
            let ptr_to_ptr_t: *const *const $T = &raw const (*ref_to_ptr);
            // _2 = move _3 as *const *const () (Transmute);
            let ptr_to_ptr: *const *const() = move ptr_to_ptr_t as *const *const () (Transmute);
            // _0 = copy (*_2);
            let data_ptr: *const () = _;
        }

        WrongAssumptionOfFatPointerLayout {
            ptr_transmute: ptr_to_ptr_stmt,
            data_ptr_get: data_ptr_stmt,
        }
    }
}

#[allow(non_snake_case)]
pub mod mut_mut_Transmute_ver {
    use rpl_context::PatCtxt;
    use rpl_mir::pat::MirPatternBuilder;
    use rustc_hir as hir;
    use rustc_hir::def_id::LocalDefId;
    use rustc_hir::intravisit::{self, Visitor};
    use rustc_middle::hir::nested_filter::All;
    use rustc_middle::ty::TyCtxt;
    use rustc_span::Span;

    use rpl_mir::{pat, CheckMirCtxt};

    #[instrument(level = "info", skip(tcx, pcx))]
    pub fn check_item(tcx: TyCtxt<'_>, pcx: PatCtxt<'_>, item_id: hir::ItemId) {
        let item = tcx.hir().item(item_id);
        // let def_id = item_id.owner_id.def_id;
        let mut check_ctxt = CheckFnCtxt { tcx, pcx };
        check_ctxt.visit_item(item);
    }

    struct CheckFnCtxt<'pcx, 'tcx> {
        tcx: TyCtxt<'tcx>,
        pcx: PatCtxt<'pcx>,
    }

    impl<'tcx> Visitor<'tcx> for CheckFnCtxt<'_, 'tcx> {
        type NestedFilter = All;
        fn nested_visit_map(&mut self) -> Self::Map {
            self.tcx.hir()
        }

        #[instrument(level = "debug", skip_all, fields(?item.owner_id))]
        fn visit_item(&mut self, item: &'tcx hir::Item<'tcx>) -> Self::Result {
            match item.kind {
                hir::ItemKind::Trait(hir::IsAuto::No, ..) | hir::ItemKind::Impl(_) | hir::ItemKind::Fn(..) => {},
                _ => return,
            }
            intravisit::walk_item(self, item);
        }

        fn visit_fn(
            &mut self,
            _kind: intravisit::FnKind<'tcx>,
            _decl: &'tcx hir::FnDecl<'tcx>,
            _body_id: hir::BodyId,
            _span: Span,
            def_id: LocalDefId,
        ) -> Self::Result {
            if self.tcx.visibility(def_id).is_public() && self.tcx.is_mir_available(def_id) {
                let body = self.tcx.optimized_mir(def_id);
                #[allow(irrefutable_let_patterns)]
                if let mut patterns = MirPatternBuilder::new(self.pcx)
                    && let pattern = pattern_wrong_assumption_of_fat_pointer_layout(&mut patterns)
                    && let Some(matches) = CheckMirCtxt::new(self.tcx, body, &patterns.build()).check()
                    && let Some(ptr_transmute) = matches[pattern.ptr_transmute]
                    && let span1 = ptr_transmute.span_no_inline(body)
                    && let Some(data_ptr_get) = matches[pattern.data_ptr_get]
                    && let span2 = data_ptr_get.span_no_inline(body)
                {
                    self.tcx
                        .dcx()
                        .emit_err(crate::errors::WrongAssumptionOfFatPointerLayout {
                            ptr_transmute: span1,
                            data_ptr_get: span2,
                        });
                }
            }
        }
    }

    struct WrongAssumptionOfFatPointerLayout {
        ptr_transmute: pat::Location,
        data_ptr_get: pat::Location,
    }

    #[rpl_macros::mir_pattern]
    fn pattern_wrong_assumption_of_fat_pointer_layout(
        patterns: &mut pat::MirPatternBuilder<'_>,
    ) -> WrongAssumptionOfFatPointerLayout {
        mir! {
            meta!{$T:ty}

            let ptr: *mut $T = _;
            // _4 = &mut _1;
            let ref_to_ptr: &mut *mut $T = &mut ptr;
            // _3 = &raw mut (*_4);
            let ptr_to_ptr_t: *mut *mut $T = &raw mut (*ref_to_ptr);
            // _2 = move _3 as *mut *mut () (Transmute);
            let ptr_to_ptr: *mut *mut() = move ptr_to_ptr_t as *mut *mut () (Transmute);
            // _0 = copy (*_2);
            let data_ptr: *mut () = _;
        }

        WrongAssumptionOfFatPointerLayout {
            ptr_transmute: ptr_to_ptr_stmt,
            data_ptr_get: data_ptr_stmt,
        }
    }
}

#[allow(non_snake_case)]
pub mod mut_const_PtrToPtr_ver {
    use rpl_context::PatCtxt;
    use rpl_mir::pat::MirPatternBuilder;
    use rustc_hir as hir;
    use rustc_hir::def_id::LocalDefId;
    use rustc_hir::intravisit::{self, Visitor};
    use rustc_middle::hir::nested_filter::All;
    use rustc_middle::ty::TyCtxt;
    use rustc_span::Span;

    use rpl_mir::{pat, CheckMirCtxt};

    #[instrument(level = "info", skip(tcx, pcx))]
    pub fn check_item(tcx: TyCtxt<'_>, pcx: PatCtxt<'_>, item_id: hir::ItemId) {
        let item = tcx.hir().item(item_id);
        // let def_id = item_id.owner_id.def_id;
        let mut check_ctxt = CheckFnCtxt { tcx, pcx };
        check_ctxt.visit_item(item);
    }

    struct CheckFnCtxt<'pcx, 'tcx> {
        tcx: TyCtxt<'tcx>,
        pcx: PatCtxt<'pcx>,
    }

    impl<'tcx> Visitor<'tcx> for CheckFnCtxt<'_, 'tcx> {
        type NestedFilter = All;
        fn nested_visit_map(&mut self) -> Self::Map {
            self.tcx.hir()
        }

        #[instrument(level = "debug", skip_all, fields(?item.owner_id))]
        fn visit_item(&mut self, item: &'tcx hir::Item<'tcx>) -> Self::Result {
            match item.kind {
                hir::ItemKind::Trait(hir::IsAuto::No, ..) | hir::ItemKind::Impl(_) | hir::ItemKind::Fn(..) => {},
                _ => return,
            }
            intravisit::walk_item(self, item);
        }

        fn visit_fn(
            &mut self,
            _kind: intravisit::FnKind<'tcx>,
            _decl: &'tcx hir::FnDecl<'tcx>,
            _body_id: hir::BodyId,
            _span: Span,
            def_id: LocalDefId,
        ) -> Self::Result {
            if self.tcx.visibility(def_id).is_public() && self.tcx.is_mir_available(def_id) {
                let body = self.tcx.optimized_mir(def_id);
                #[allow(irrefutable_let_patterns)]
                if let mut patterns = MirPatternBuilder::new(self.pcx)
                    && let pattern = pattern_wrong_assumption_of_fat_pointer_layout(&mut patterns)
                    && let Some(matches) = CheckMirCtxt::new(self.tcx, body, &patterns.build()).check()
                    && let Some(ptr_transmute) = matches[pattern.ptr_transmute]
                    && let span1 = ptr_transmute.span_no_inline(body)
                    && let Some(data_ptr_get) = matches[pattern.data_ptr_get]
                    && let span2 = data_ptr_get.span_no_inline(body)
                {
                    self.tcx
                        .dcx()
                        .emit_err(crate::errors::WrongAssumptionOfFatPointerLayout {
                            ptr_transmute: span1,
                            data_ptr_get: span2,
                        });
                }
            }
        }
    }

    struct WrongAssumptionOfFatPointerLayout {
        ptr_transmute: pat::Location,
        data_ptr_get: pat::Location,
    }

    #[rpl_macros::mir_pattern]
    fn pattern_wrong_assumption_of_fat_pointer_layout(
        patterns: &mut pat::MirPatternBuilder<'_>,
    ) -> WrongAssumptionOfFatPointerLayout {
        mir! {
            meta!{$T:ty}

            let ptr: *const $T = _;
            let ref_to_ptr: &mut *const $T = &mut ptr;
            let ptr_to_ptr_t: *mut *const $T = &raw mut (*ref_to_ptr);
            let ptr_to_ptr: *mut *mut () = move ptr_to_ptr_t as *mut *mut () (PtrToPtr);
            // _0 = copy (*_2);
            let data_ptr: *mut () = _;
        }

        WrongAssumptionOfFatPointerLayout {
            ptr_transmute: ptr_to_ptr_stmt,
            data_ptr_get: data_ptr_stmt,
        }
    }
}