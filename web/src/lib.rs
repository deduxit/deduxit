// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! deduxit for a web page: the library compiled to WebAssembly. Each call
//! takes JSON text and returns JSON or SVG text.
//!
//! Every call takes the documents it works on, such as a session, and the
//! page's settings, and returns a document. Nothing is kept between calls.
//! The settings are the JSON of [`deduxit::Settings`], read over the
//! tab's defaults: an empty text is the defaults, and a key the page gives
//! replaces the default there.
//!
//! A call that fails throws the error's JSON as a string, `{"code",
//! "kind", "message", "setting", "details"}`, `setting` only where a bound
//! refused. The page branches on `code` and `kind` and words the message
//! itself.
//!
//! The module reads no clock and starts no thread. A call ends at the
//! bounds of the settings, the work bound first, or when the page stops
//! its worker.
//!
//! The calls are those of [`calls`], exported under their names, `export`
//! as `export_as`. Those of [`ordinary`] are exported with the prefix
//! `ordinary_`, those of [`rules`], which build a derivation in a
//! calculus given as data, with the prefix `rules_`, and those of
//! [`modal`], which decide modal logic, with the prefix `modal_`, and
//! those of [`lambda`], which check lambda-terms and find a type's
//! terms, with the prefix `lambda_`. [`protocol`] writes
//! down which call of each suite is which of the protocol's, `describe`
//! answering it as JSON. Those of [`workspace`], the linked tabs a page
//! keeps, are exported with the prefix `workspace_`.
//!
//! PNG and PDF are rendered from a drawing's SVG by a module of their
//! own, `deduxit-web-render`, which the page loads on its first such
//! export.

pub mod calls;
pub mod lambda;
pub mod modal;
pub mod ordinary;
pub mod protocol;
pub mod rules;
pub mod tab;
pub mod workspace;

pub use deduxit_web_tab::Failure;

/// The exports of the module, which wrap [`calls`] for JavaScript.
mod exports {
    use crate::Failure;
    use crate::{calls, lambda, modal, ordinary, protocol, rules, workspace};
    use wasm_bindgen::prelude::{JsValue, wasm_bindgen};

    /// Returns a failure as the string a call throws.
    fn thrown(failure: Failure) -> JsValue {
        JsValue::from_str(&failure.0)
    }

    /// Returns a goal or a position the page gives as a number, or the
    /// failure thrown, as [`calls::index`] says.
    fn index(number: f64, what: &str) -> Result<u32, JsValue> {
        calls::index(number, what).map_err(thrown)
    }

    /// See [`calls::about`].
    #[wasm_bindgen]
    pub fn about() -> String {
        calls::about()
    }

    /// See [`calls::defaults`].
    #[wasm_bindgen]
    pub fn defaults() -> String {
        calls::defaults()
    }

    /// See [`protocol::describe`].
    #[wasm_bindgen]
    pub fn describe() -> String {
        protocol::describe()
    }

    /// See [`calls::read`].
    #[wasm_bindgen]
    pub fn read(input: &str, settings: &str) -> Result<String, JsValue> {
        calls::read(input, settings).map_err(thrown)
    }

    /// See [`calls::read_as`].
    #[wasm_bindgen]
    pub fn read_as(input: &str, mode: &str, settings: &str) -> Result<String, JsValue> {
        calls::read_as(input, mode, settings).map_err(thrown)
    }

    /// See [`calls::start`].
    #[wasm_bindgen]
    pub fn start(sequent: &str, mode: &str, settings: &str) -> Result<String, JsValue> {
        calls::start(sequent, mode, settings).map_err(thrown)
    }

    /// See [`calls::view`].
    #[wasm_bindgen]
    pub fn view(session: &str, settings: &str) -> Result<String, JsValue> {
        calls::view(session, settings).map_err(thrown)
    }

    /// See [`calls::candidates`].
    #[wasm_bindgen]
    pub fn candidates(
        session: &str,
        goal: f64,
        position: f64,
        settings: &str,
    ) -> Result<String, JsValue> {
        let (goal, position) = (index(goal, "a goal")?, index(position, "a position")?);
        calls::candidates(session, goal, position, settings).map_err(thrown)
    }

    /// See [`calls::rules`].
    #[wasm_bindgen]
    pub fn rules(
        session: &str,
        goal: f64,
        position: f64,
        settings: &str,
    ) -> Result<String, JsValue> {
        let (goal, position) = (index(goal, "a goal")?, index(position, "a position")?);
        calls::rules(session, goal, position, settings).map_err(thrown)
    }

    /// See [`calls::split_passes`].
    #[wasm_bindgen]
    pub fn split_passes(
        session: &str,
        goal: f64,
        step: &str,
        settings: &str,
    ) -> Result<bool, JsValue> {
        calls::split_passes(session, index(goal, "a goal")?, step, settings).map_err(thrown)
    }

    /// See [`calls::apply`].
    #[wasm_bindgen]
    pub fn apply(session: &str, goal: f64, step: &str, settings: &str) -> Result<String, JsValue> {
        calls::apply(session, index(goal, "a goal")?, step, settings).map_err(thrown)
    }

    /// See [`calls::cut`].
    #[wasm_bindgen]
    pub fn cut(
        session: &str,
        goal: f64,
        formula: &str,
        left: &str,
        settings: &str,
    ) -> Result<String, JsValue> {
        calls::cut(session, index(goal, "a goal")?, formula, left, settings).map_err(thrown)
    }

    /// See [`calls::invert_all`].
    #[wasm_bindgen]
    pub fn invert_all(session: &str, settings: &str) -> Result<String, JsValue> {
        calls::invert_all(session, settings).map_err(thrown)
    }

    /// See [`calls::undo`].
    #[wasm_bindgen]
    pub fn undo(session: &str, settings: &str) -> Result<String, JsValue> {
        calls::undo(session, settings).map_err(thrown)
    }

    /// See [`calls::close`].
    #[wasm_bindgen]
    pub fn close(session: &str, goal: f64, settings: &str) -> Result<String, JsValue> {
        calls::close(session, index(goal, "a goal")?, settings).map_err(thrown)
    }

    /// See [`calls::fill`].
    #[wasm_bindgen]
    pub fn fill(session: &str, goal: f64, source: &str, settings: &str) -> Result<String, JsValue> {
        calls::fill(session, index(goal, "a goal")?, source, settings).map_err(thrown)
    }

    /// See [`calls::close_all`].
    #[wasm_bindgen]
    pub fn close_all(session: &str, settings: &str) -> Result<String, JsValue> {
        calls::close_all(session, settings).map_err(thrown)
    }

    /// See [`calls::hint`].
    #[wasm_bindgen]
    pub fn hint(session: &str, goal: f64, settings: &str) -> Result<String, JsValue> {
        calls::hint(session, index(goal, "a goal")?, settings).map_err(thrown)
    }

    /// See [`calls::proof`].
    #[wasm_bindgen]
    pub fn proof(session: &str, settings: &str) -> Result<String, JsValue> {
        calls::proof(session, settings).map_err(thrown)
    }

    /// See [`calls::term`].
    #[wasm_bindgen]
    pub fn term(session: &str, settings: &str) -> Result<String, JsValue> {
        calls::term(session, settings).map_err(thrown)
    }

    /// See [`calls::prove`].
    #[wasm_bindgen]
    pub fn prove(sequent: &str, mode: &str, settings: &str) -> Result<String, JsValue> {
        calls::prove(sequent, mode, settings).map_err(thrown)
    }

    /// See [`calls::draw`].
    #[wasm_bindgen]
    pub fn draw(session: &str, settings: &str) -> Result<String, JsValue> {
        calls::draw(session, settings).map_err(thrown)
    }

    /// See [`calls::draw_net`].
    #[wasm_bindgen]
    pub fn draw_net(session: &str, settings: &str) -> Result<String, JsValue> {
        calls::draw_net(session, settings).map_err(thrown)
    }

    /// Returns the cut or cut link a page names as a number. `undefined`
    /// or `null` is `None`, the strategy's choice.
    fn at(at: Option<f64>, what: &str) -> Result<Option<u32>, JsValue> {
        at.map(|at| index(at, what)).transpose()
    }

    /// See [`calls::cuts`].
    #[wasm_bindgen]
    pub fn cuts(session: &str, proof: &str, settings: &str) -> Result<String, JsValue> {
        calls::cuts(session, proof, settings).map_err(thrown)
    }

    /// See [`calls::cut_step`]. `at` is a node, or `undefined` for the
    /// strategy's cut.
    #[wasm_bindgen]
    pub fn cut_step(
        session: &str,
        proof: &str,
        at: Option<f64>,
        settings: &str,
    ) -> Result<String, JsValue> {
        calls::cut_step(session, proof, self::at(at, "a cut")?, settings).map_err(thrown)
    }

    /// See [`calls::eliminate`].
    #[wasm_bindgen]
    pub fn eliminate(session: &str, proof: &str, settings: &str) -> Result<String, JsValue> {
        calls::eliminate(session, proof, settings).map_err(thrown)
    }

    /// See [`calls::net_cuts`].
    #[wasm_bindgen]
    pub fn net_cuts(session: &str, net: &str, settings: &str) -> Result<String, JsValue> {
        calls::net_cuts(session, net, settings).map_err(thrown)
    }

    /// See [`calls::net_cut_step`]. `at` is a vertex at either end of a
    /// cut link, or `undefined` for the strategy's.
    #[wasm_bindgen]
    pub fn net_cut_step(
        session: &str,
        net: &str,
        at: Option<f64>,
        settings: &str,
    ) -> Result<String, JsValue> {
        calls::net_cut_step(session, net, self::at(at, "a vertex")?, settings).map_err(thrown)
    }

    /// See [`calls::net_eliminate`].
    #[wasm_bindgen]
    pub fn net_eliminate(session: &str, net: &str, settings: &str) -> Result<String, JsValue> {
        calls::net_eliminate(session, net, settings).map_err(thrown)
    }

    /// See [`calls::export`]. JavaScript exports it as `export_as`:
    /// `export` is a reserved word there, which wasm-bindgen would rename
    /// `_export`.
    #[wasm_bindgen(js_name = "export_as")]
    pub fn export(session: &str, format: &str, settings: &str) -> Result<String, JsValue> {
        calls::export(session, format, settings).map_err(thrown)
    }

    /// See [`calls::certificate`].
    #[wasm_bindgen]
    pub fn certificate(document: &str, settings: &str) -> Result<String, JsValue> {
        calls::certificate(document, settings).map_err(thrown)
    }

    /// See [`ordinary::start`].
    #[wasm_bindgen]
    pub fn ordinary_start(sequent: &str, logic: &str, settings: &str) -> Result<String, JsValue> {
        ordinary::start(sequent, logic, settings).map_err(thrown)
    }

    /// See [`ordinary::view`].
    #[wasm_bindgen]
    pub fn ordinary_view(session: &str, settings: &str) -> Result<String, JsValue> {
        ordinary::view(session, settings).map_err(thrown)
    }

    /// See [`ordinary::candidates`].
    #[wasm_bindgen]
    pub fn ordinary_candidates(
        session: &str,
        goal: f64,
        side: &str,
        position: f64,
        settings: &str,
    ) -> Result<String, JsValue> {
        let (goal, position) = (index(goal, "a goal")?, index(position, "a position")?);
        ordinary::candidates(session, goal, side, position, settings).map_err(thrown)
    }

    /// See [`ordinary::rules`].
    #[wasm_bindgen]
    pub fn ordinary_rules(
        session: &str,
        goal: f64,
        side: &str,
        position: f64,
        settings: &str,
    ) -> Result<String, JsValue> {
        let (goal, position) = (index(goal, "a goal")?, index(position, "a position")?);
        ordinary::rules(session, goal, side, position, settings).map_err(thrown)
    }

    /// See [`ordinary::apply`].
    #[wasm_bindgen]
    pub fn ordinary_apply(
        session: &str,
        goal: f64,
        step: &str,
        settings: &str,
    ) -> Result<String, JsValue> {
        ordinary::apply(session, index(goal, "a goal")?, step, settings).map_err(thrown)
    }

    /// See [`ordinary::undo`].
    #[wasm_bindgen]
    pub fn ordinary_undo(session: &str, settings: &str) -> Result<String, JsValue> {
        ordinary::undo(session, settings).map_err(thrown)
    }

    /// See [`ordinary::close`].
    #[wasm_bindgen]
    pub fn ordinary_close(session: &str, goal: f64, settings: &str) -> Result<String, JsValue> {
        ordinary::close(session, index(goal, "a goal")?, settings).map_err(thrown)
    }

    /// See [`ordinary::hint`].
    #[wasm_bindgen]
    pub fn ordinary_hint(session: &str, goal: f64, settings: &str) -> Result<String, JsValue> {
        ordinary::hint(session, index(goal, "a goal")?, settings).map_err(thrown)
    }

    /// See [`ordinary::term`].
    #[wasm_bindgen]
    pub fn ordinary_term(session: &str, settings: &str) -> Result<String, JsValue> {
        ordinary::term(session, settings).map_err(thrown)
    }

    /// See [`ordinary::model`]: `world` absent for every world.
    #[wasm_bindgen]
    pub fn ordinary_model(
        model: &str,
        sequent: &str,
        logic: &str,
        world: Option<f64>,
        settings: &str,
    ) -> Result<String, JsValue> {
        let world = world.map(|w| index(w, "a world")).transpose()?;
        ordinary::model(model, sequent, logic, world, settings).map_err(thrown)
    }

    /// See [`ordinary::structure`].
    #[wasm_bindgen]
    pub fn ordinary_structure(
        structure: &str,
        sequent: &str,
        logic: &str,
        settings: &str,
    ) -> Result<String, JsValue> {
        ordinary::structure(structure, sequent, logic, settings).map_err(thrown)
    }

    /// See [`ordinary::table`].
    #[wasm_bindgen]
    pub fn ordinary_table(sequent: &str, settings: &str) -> Result<String, JsValue> {
        ordinary::table(sequent, settings).map_err(thrown)
    }

    /// See [`ordinary::tableau`].
    #[wasm_bindgen]
    pub fn ordinary_tableau(sequent: &str, settings: &str) -> Result<String, JsValue> {
        ordinary::tableau(sequent, settings).map_err(thrown)
    }

    /// See [`ordinary::invert_all`].
    #[wasm_bindgen]
    pub fn ordinary_invert_all(session: &str, settings: &str) -> Result<String, JsValue> {
        ordinary::invert_all(session, settings).map_err(thrown)
    }

    /// See [`ordinary::tableau_review`]: `sequent` `""` for none.
    #[wasm_bindgen]
    pub fn ordinary_tableau_review(
        text: &str,
        sequent: &str,
        settings: &str,
    ) -> Result<String, JsValue> {
        ordinary::tableau_review(text, sequent, settings).map_err(thrown)
    }

    /// See [`ordinary::translate`].
    #[wasm_bindgen]
    pub fn ordinary_translate(
        sequent: &str,
        options: &str,
        settings: &str,
    ) -> Result<String, JsValue> {
        ordinary::translate(sequent, options, settings).map_err(thrown)
    }

    /// See [`ordinary::normalize`].
    #[wasm_bindgen]
    pub fn ordinary_normalize(
        sequent: &str,
        options: &str,
        settings: &str,
    ) -> Result<String, JsValue> {
        ordinary::normalize(sequent, options, settings).map_err(thrown)
    }

    /// See [`ordinary::resolve`].
    #[wasm_bindgen]
    pub fn ordinary_resolve(
        sequent: &str,
        options: &str,
        settings: &str,
    ) -> Result<String, JsValue> {
        ordinary::resolve(sequent, options, settings).map_err(thrown)
    }

    /// See [`ordinary::clauses`].
    #[wasm_bindgen]
    pub fn ordinary_clauses(sequent: &str, form: &str, settings: &str) -> Result<String, JsValue> {
        ordinary::clauses(sequent, form, settings).map_err(thrown)
    }

    /// See [`ordinary::resolve_step`].
    #[wasm_bindgen]
    pub fn ordinary_resolve_step(
        refutation: &str,
        first: f64,
        second: f64,
        atom: f64,
        settings: &str,
    ) -> Result<String, JsValue> {
        ordinary::resolve_step(refutation, first, second, atom, settings).map_err(thrown)
    }

    /// See [`ordinary::refutation_check`].
    #[wasm_bindgen]
    pub fn ordinary_refutation_check(refutation: &str, settings: &str) -> Result<String, JsValue> {
        ordinary::refutation_check(refutation, settings).map_err(thrown)
    }

    /// See [`ordinary::interpolate`].
    #[wasm_bindgen]
    pub fn ordinary_interpolate(
        sequent: &str,
        logic: &str,
        partition: &str,
        settings: &str,
    ) -> Result<String, JsValue> {
        ordinary::interpolate(sequent, logic, partition, settings).map_err(thrown)
    }

    /// See [`ordinary::draw`].
    #[wasm_bindgen]
    pub fn ordinary_draw(session: &str, settings: &str) -> Result<String, JsValue> {
        ordinary::draw(session, settings).map_err(thrown)
    }

    /// See [`ordinary::export`].
    #[wasm_bindgen]
    pub fn ordinary_export(session: &str, format: &str, settings: &str) -> Result<String, JsValue> {
        ordinary::export(session, format, settings).map_err(thrown)
    }

    /// See [`ordinary::model_drawing`].
    #[wasm_bindgen]
    pub fn ordinary_model_drawing(outcome: &str, settings: &str) -> Result<String, JsValue> {
        ordinary::model_drawing(outcome, settings).map_err(thrown)
    }

    /// See [`ordinary::read`].
    #[wasm_bindgen]
    pub fn ordinary_read(input: &str, logic: &str, settings: &str) -> Result<String, JsValue> {
        ordinary::read(input, logic, settings).map_err(thrown)
    }

    /// See [`ordinary::certificate`].
    #[wasm_bindgen]
    pub fn ordinary_certificate(outcome: &str, settings: &str) -> Result<String, JsValue> {
        ordinary::certificate(outcome, settings).map_err(thrown)
    }

    /// See [`ordinary::model_certificate`].
    #[wasm_bindgen]
    pub fn ordinary_model_certificate(outcome: &str, settings: &str) -> Result<String, JsValue> {
        ordinary::model_certificate(outcome, settings).map_err(thrown)
    }

    /// See [`rules::calculus`].
    #[wasm_bindgen]
    pub fn rules_calculus(name: &str) -> Result<String, JsValue> {
        rules::calculus(name).map_err(thrown)
    }

    /// See [`rules::read`].
    #[wasm_bindgen]
    pub fn rules_read(calculus: &str, settings: &str) -> Result<String, JsValue> {
        rules::read(calculus, settings).map_err(thrown)
    }

    /// See [`rules::start`].
    #[wasm_bindgen]
    pub fn rules_start(
        calculus: &str,
        goal: &str,
        assumptions: &str,
        settings: &str,
    ) -> Result<String, JsValue> {
        rules::start(calculus, goal, assumptions, settings).map_err(thrown)
    }

    /// See [`rules::view`].
    #[wasm_bindgen]
    pub fn rules_view(session: &str, settings: &str) -> Result<String, JsValue> {
        rules::view(session, settings).map_err(thrown)
    }

    /// See [`rules::rules`].
    #[wasm_bindgen]
    pub fn rules_rules(session: &str, goal: f64, settings: &str) -> Result<String, JsValue> {
        rules::rules(session, index(goal, "a goal")?, settings).map_err(thrown)
    }

    /// See [`rules::candidates`].
    #[wasm_bindgen]
    pub fn rules_candidates(session: &str, goal: f64, settings: &str) -> Result<String, JsValue> {
        rules::candidates(session, index(goal, "a goal")?, settings).map_err(thrown)
    }

    /// See [`rules::apply`].
    #[wasm_bindgen]
    pub fn rules_apply(
        session: &str,
        goal: f64,
        step: &str,
        settings: &str,
    ) -> Result<String, JsValue> {
        rules::apply(session, index(goal, "a goal")?, step, settings).map_err(thrown)
    }

    /// See [`rules::derive`].
    #[wasm_bindgen]
    pub fn rules_derive(
        session: &str,
        goal: f64,
        step: &str,
        settings: &str,
    ) -> Result<String, JsValue> {
        rules::derive(session, index(goal, "a goal")?, step, settings).map_err(thrown)
    }

    /// See [`rules::cite`].
    #[wasm_bindgen]
    pub fn rules_cite(
        session: &str,
        goal: f64,
        line: f64,
        settings: &str,
    ) -> Result<String, JsValue> {
        rules::cite(
            session,
            index(goal, "a goal")?,
            index(line, "a line")?,
            settings,
        )
        .map_err(thrown)
    }

    /// See [`rules::assume`].
    #[wasm_bindgen]
    pub fn rules_assume(session: &str, goal: f64, settings: &str) -> Result<String, JsValue> {
        rules::assume(session, index(goal, "a goal")?, settings).map_err(thrown)
    }

    /// See [`rules::undo`].
    #[wasm_bindgen]
    pub fn rules_undo(session: &str, settings: &str) -> Result<String, JsValue> {
        rules::undo(session, settings).map_err(thrown)
    }

    /// See [`rules::check`].
    #[wasm_bindgen]
    pub fn rules_check(session: &str, settings: &str) -> Result<String, JsValue> {
        rules::check(session, settings).map_err(thrown)
    }

    /// See [`rules::draw`].
    #[wasm_bindgen]
    pub fn rules_draw(session: &str, settings: &str) -> Result<String, JsValue> {
        rules::draw(session, settings).map_err(thrown)
    }

    /// See [`rules::export`].
    #[wasm_bindgen]
    pub fn rules_export(session: &str, format: &str, settings: &str) -> Result<String, JsValue> {
        rules::export(session, format, settings).map_err(thrown)
    }

    /// See [`lambda::read`].
    #[wasm_bindgen]
    pub fn lambda_read(text: &str, calculus: &str, settings: &str) -> Result<String, JsValue> {
        lambda::read(text, calculus, settings).map_err(thrown)
    }

    /// See [`lambda::check`].
    #[wasm_bindgen]
    pub fn lambda_check(term: &str, settings: &str) -> Result<String, JsValue> {
        lambda::check(term, settings).map_err(thrown)
    }

    /// See [`lambda::draw`].
    #[wasm_bindgen]
    pub fn lambda_draw(typing: &str, settings: &str) -> Result<String, JsValue> {
        lambda::draw(typing, settings).map_err(thrown)
    }

    /// See [`lambda::export`].
    #[wasm_bindgen]
    pub fn lambda_export(typing: &str, format: &str, settings: &str) -> Result<String, JsValue> {
        lambda::export(typing, format, settings).map_err(thrown)
    }

    /// See [`lambda::read_type`].
    #[wasm_bindgen]
    pub fn lambda_read_type(text: &str, settings: &str) -> Result<String, JsValue> {
        lambda::read_type(text, settings).map_err(thrown)
    }

    /// See [`lambda::inhabit`].
    #[wasm_bindgen]
    pub fn lambda_inhabit(ty: &str, settings: &str) -> Result<String, JsValue> {
        lambda::inhabit(ty, settings).map_err(thrown)
    }

    /// See [`lambda::enumerate`].
    #[wasm_bindgen]
    pub fn lambda_enumerate(ty: &str, options: &str, settings: &str) -> Result<String, JsValue> {
        lambda::enumerate(ty, options, settings).map_err(thrown)
    }

    /// See [`modal::read`].
    #[wasm_bindgen]
    pub fn modal_read(text: &str, system: &str, settings: &str) -> Result<String, JsValue> {
        modal::read(text, system, settings).map_err(thrown)
    }

    /// See [`modal::decide`].
    #[wasm_bindgen]
    pub fn modal_decide(statement: &str, settings: &str) -> Result<String, JsValue> {
        modal::decide(statement, settings).map_err(thrown)
    }

    /// See [`modal::countermodel`].
    #[wasm_bindgen]
    pub fn modal_countermodel(outcome: &str, settings: &str) -> Result<String, JsValue> {
        modal::countermodel(outcome, settings).map_err(thrown)
    }

    /// See [`modal::evaluate`]; a `world` of `undefined` checks every
    /// world.
    #[wasm_bindgen]
    pub fn modal_evaluate(
        statement: &str,
        model: &str,
        world: Option<f64>,
        settings: &str,
    ) -> Result<String, JsValue> {
        let world = world.map(|w| index(w, "world")).transpose()?;
        modal::evaluate(statement, model, world, settings).map_err(thrown)
    }

    /// See [`modal::draw`].
    #[wasm_bindgen]
    pub fn modal_draw(outcome: &str, settings: &str) -> Result<String, JsValue> {
        modal::draw(outcome, settings).map_err(thrown)
    }

    /// See [`modal::export`].
    #[wasm_bindgen]
    pub fn modal_export(outcome: &str, format: &str, settings: &str) -> Result<String, JsValue> {
        modal::export(outcome, format, settings).map_err(thrown)
    }

    /// See [`modal::calculus`].
    #[wasm_bindgen]
    pub fn modal_calculus(statement: &str, settings: &str) -> Result<String, JsValue> {
        modal::calculus(statement, settings).map_err(thrown)
    }

    /// See [`modal::model_drawing`].
    #[wasm_bindgen]
    pub fn modal_model_drawing(
        statement: &str,
        model: &str,
        world: f64,
        settings: &str,
    ) -> Result<String, JsValue> {
        modal::model_drawing(statement, model, index(world, "world")?, settings).map_err(thrown)
    }

    /// See [`workspace::statement`].
    #[wasm_bindgen]
    pub fn workspace_statement(
        suite: &str,
        text: &str,
        parameter: &str,
        settings: &str,
    ) -> Result<String, JsValue> {
        workspace::statement(suite, text, parameter, settings).map_err(thrown)
    }

    /// See [`workspace::open`].
    #[wasm_bindgen]
    pub fn workspace_open(document: &str, settings: &str) -> Result<String, JsValue> {
        workspace::open(document, settings).map_err(thrown)
    }

    /// See [`workspace::edit`].
    #[wasm_bindgen]
    pub fn workspace_edit(
        document: &str,
        action: &str,
        work: &str,
        settings: &str,
    ) -> Result<String, JsValue> {
        workspace::edit(document, action, work, settings).map_err(thrown)
    }

    /// See [`workspace::undo`].
    #[wasm_bindgen]
    pub fn workspace_undo(document: &str, work: &str, settings: &str) -> Result<String, JsValue> {
        workspace::undo(document, work, settings, false).map_err(thrown)
    }

    /// See [`workspace::undo`], forward.
    #[wasm_bindgen]
    pub fn workspace_redo(document: &str, work: &str, settings: &str) -> Result<String, JsValue> {
        workspace::undo(document, work, settings, true).map_err(thrown)
    }

    /// See [`workspace::compute`].
    #[wasm_bindgen]
    pub fn workspace_compute(
        document: &str,
        node: f64,
        what: &str,
        work: &str,
        settings: &str,
    ) -> Result<String, JsValue> {
        workspace::compute(document, index(node, "node")?, what, work, settings).map_err(thrown)
    }

    /// See [`workspace::accept`].
    #[wasm_bindgen]
    pub fn workspace_accept(
        document: &str,
        computed: &str,
        work: &str,
        settings: &str,
    ) -> Result<String, JsValue> {
        workspace::accept(document, computed, work, settings).map_err(thrown)
    }

    /// See [`workspace::run`].
    #[wasm_bindgen]
    pub fn workspace_run(
        document: &str,
        nodes: &str,
        what: &str,
        work: &str,
        settings: &str,
    ) -> Result<String, JsValue> {
        workspace::run(document, nodes, what, work, settings).map_err(thrown)
    }

    /// See [`workspace::view`].
    #[wasm_bindgen]
    pub fn workspace_view(document: &str, work: &str, settings: &str) -> Result<String, JsValue> {
        workspace::view(document, work, settings).map_err(thrown)
    }

    /// See [`workspace::pack`].
    #[wasm_bindgen]
    pub fn workspace_pack(
        document: &str,
        nodes: &str,
        work: &str,
        settings: &str,
    ) -> Result<String, JsValue> {
        workspace::pack(document, nodes, work, settings).map_err(thrown)
    }
}
