use crate::{pretty_type_name, pretty_type_name_str};

#[test]
fn shorten_name_basic() {
    assert_eq!(
        pretty_type_name_str("path::to::some::Type"),
        "Type".to_string()
    );
}
#[test]
fn shorten_name_generic() {
    assert_eq!(
        pretty_type_name_str("bevy::ecs::Handle<bevy::render::StandardMaterial>"),
        "Handle<StandardMaterial>".to_string()
    );
}
#[test]
fn shorten_name_nested_generic() {
    assert_eq!(
        pretty_type_name_str("foo::bar::quux<qaax<p::t::b>>"),
        "quux<qaax<b>>".to_string()
    );
}

#[test]
fn tuple() {
    assert_eq!(pretty_type_name_str("(x::a, x::b)"), "(a, b)".to_string());
}

#[test]
fn complex_name() {
    assert_eq!(
            pretty_type_name_str("bevy_inspector_egui::world_inspector::impls::InspectorQuery<(bevy_ecs::core::filter::With<bevy_ui::node::Node>, bevy_ecs::core::filter::Without<bevy_transform::components::parent::Parent>)>"),
            "InspectorQuery<(With<Node>, Without<Parent>)>".to_string());
}

#[test]
fn tuples() {
    assert_eq!(
        pretty_type_name_str("(m::A, (m::B, m::C))"),
        "(A, (B, C))".to_string()
    );
}

#[test]
fn fn_type_name() {
    mod m {
        pub struct T;
    }

    assert_eq!(
        pretty_type_name::<fn(String, m::T)>(),
        "fn(String, T)".to_string()
    );
}
