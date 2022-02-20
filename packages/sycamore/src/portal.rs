//! Portal API.

use std::any::Any;

use wasm_bindgen::prelude::*;

use crate::component::Children;
use crate::prelude::*;

/// Props for [`Portal`].
#[derive(Prop)]
pub struct PortalProps<'a, G>
where
    G: GenericNode,
{
    pub children: Children<'a, G>,
    pub selector: &'static str,
}

/// A portal into another part of the DOM.
#[component]
pub fn Portal<'a, G: Html>(
    ctx: ScopeRef<'a>,
    props: PortalProps<'a, G>,
) -> View<G> {
    let PortalProps { children, selector } = props;

    if G::IS_BROWSER {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let container = document
            .query_selector(selector)
            .unwrap()
            .expect_throw("could not find element matching selector");

        let children = children.call(ctx).flatten();

        for child in &children {
            container
                .append_child(
                    &<dyn Any>::downcast_ref::<DomNode>(child)
                        .unwrap()
                        .inner_element(),
                )
                .unwrap();
        }

        ctx.on_cleanup(move || {
            for child in &children {
                container
                    .remove_child(
                        &<dyn Any>::downcast_ref::<DomNode>(child)
                            .unwrap()
                            .inner_element(),
                    )
                    .unwrap();
            }
        });
    } else {
        // TODO: Support for other types of nodes.
    }

    view! { ctx, }
}
