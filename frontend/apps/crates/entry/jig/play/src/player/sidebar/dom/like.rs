use std::rc::Rc;

use dominator::{clone, html, Dom};

use shared::{
    api::{endpoints::jig, ApiEndpoint},
    domain::jig::JigResponse,
    error::EmptyError,
};

use utils::{events, prelude::api_with_auth_empty};

use super::super::state::State;

pub fn render(state: Rc<State>, jig: &JigResponse) -> Dom {
    html!("jig-play-sidebar-action", {
        .property("slot", "actions")
        .property("kind", "like")
        .property_signal("active", state.player_state.jig_liked.signal_ref(|jig_liked| jig_liked.unwrap_or(false)))
        // TODO Render active or not active
        .event(clone!(state, jig => move |_: events::Click| {
            // If jig_liked is None, we don't want to do anything because the request to fetch
            // whether the user liked this jig may not have resolved yet.
            if let Some(jig_liked) = state.player_state.jig_liked.get() {
                state.loader.load(clone!(state => async move {
                    // Immediately update the liked status so that it renders the correct icon to
                    // the user. If the like/unlike request fails, we reset it to its original
                    // state.
                    state.player_state.jig_liked.set(Some(!jig_liked));

                    let response = if jig_liked {
                        // Unlike the JIG
                        let path = jig::Unlike::PATH.replace("{id}", &jig.id.0.to_string());
                        api_with_auth_empty::<EmptyError, ()>(
                            &path,
                            jig::Unlike::METHOD,
                            None
                        )
                        .await
                    } else {
                        // Like the JIG
                        let path = jig::Like::PATH.replace("{id}", &jig.id.0.to_string());
                        api_with_auth_empty::<EmptyError, ()>(
                            &path,
                            jig::Like::METHOD,
                            None
                        )
                        .await
                    };

                    if response.is_err() {
                        state.player_state.jig_liked.set(Some(jig_liked));
                    }
                }));
            }
        }))
    })
}
