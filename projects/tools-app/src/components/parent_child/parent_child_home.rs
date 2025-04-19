use leptos::*;
use leptos_router::*;

#[component]
pub fn ParentChildHome() -> impl IntoView {
    view! {
        <div>
            <h2>"Parent Child Home"</h2>
            <ul>
                <li>
                    <A href="/parent-child/write-signal">"Write Signal"</A>
                    <A href="/parent-child/callback">"Callback"</A>
                </li>
            </ul>
            <Outlet />
        </div>
    }
}
