use leptos::*;

#[component] // 모든 컴포넌트 정의와 마찬가지로, `#[component]` 매크로로 시작함.
fn App() -> impl IntoView {
    // 모든 타입의 파라미터를 0개 이상 받음 / Leptos `view`에서 반환할 수 있는 모든 것을 포함하는 불투명 타입인 `impl IntoView`를 반환함
    let (count, set_count) = create_signal(0); // React의 state 개념=== Leptos의 signal 개념. (getter, setter) 반환.

    // JSX-like 포맷인 `view` macro를 통해 UI를 정의함.
    view! {
        <button
            on:click=move |_| { set_count(count() + 1) }
            class=("w-20", move || count() % 2 == 1)
        >
            "Click me: "
            {count}
        </button>
    }
    // count라는 현재 signal을 받아올 때,  nightly Rust에서는 count()로도 사용 가능. 원래는 count.get()임.
    // nightly Rust에서는 {move || count.get()} 대신에 {count}만으로도 현재 signal을 불러와서 사용할 수 있음.

    // class:red=move || count() % 2 == 1 -> count가 홀수일 경우 class에 red를 추가.
    // `class:` 어트리뷰트는
    // 1. class name을 콜론 뒤에 받고
    // 2. 그 뒤에는 bool value, 혹은 bool을 리턴하는 함수를 받는다.
    // class=("red", move || count() % 2 == 1) 와 같이 튜플 형태로도 사용할 수 있음.
}

#[component]
fn ButtonWithProgressBar() -> impl IntoView {
    let (x, set_x) = create_signal::<i32>(0);
    let double_x = move || x() * 2;

    view! {
        <progress max="50" value=double_x></progress>
        <br/>
        <p>"Double Count: " {double_x}</p>
        <br/>
        <button
            on:click=move |_| {
                set_x.update(|x| *x += 1);
            }

            style="position: absolute"
            style:background-color=move || format!("rgb({},{},100", x(), 100)
            style:max-width="400px"
            style=("--columns", x)
        >
            "click to move"
        </button>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <ButtonWithProgressBar/> })
}
