pub struct LocatoinRenderingConfig {
    // All location rendering
    render_all_location_vec: Rc<RefCell<bool>>,
    render_location_vec: Rc<RefCell<Vec<Rc<RefCell<WorldLocation>>>>>,

    // Location focused on
    render_only_focused_location: Rc<RefCell<bool>>,
    focused_location: Option<Rc<RefCell<WorldLocation>>>,


}