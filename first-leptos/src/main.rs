use leptos::*;

#[component] // 모든 컴포넌트 정의와 마찬가지로, `#[component]` 매크로로 시작함.
fn App() -> impl IntoView {
    // 모든 타입의 파라미터를 0개 이상 받음 / Leptos `view`에서 반환할 수 있는 모든 것을 포함하는 불투명 타입인 `impl IntoView`를 반환함
    let (count, set_count) = create_signal(0); // React의 state 개념=== Leptos의 signal 개념. (getter, setter) 반환.

    // JSX-like 포맷인 `view` macro를 통해 UI를 정의함.
    view! { <button on:click=move |_| { set_count(count() + 1) }>"Click me: " {count}</button> }
    // count라는 현재 signal을 받아올 때,  nightly Rust에서는 count()로도 사용 가능. 원래는 count.get()임.
    // nightly Rust에서는 {move || count.get()} 대신에 {count}만으로도 현재 signal을 불러와서 사용할 수 있음.
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> })
}
