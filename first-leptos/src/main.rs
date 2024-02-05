use leptos::*;

#[component]
fn ProgressBar<F>(#[prop(default = 100)] max: u16, progress: F) -> impl IntoView
where
    F: Fn() -> i32 + 'static, // static한 lifetime을 갖기 위해 제네릭의 타입에 `+ 'static` 처리를 해줘야 함.
{
    // 여기서 T가 아니라 ReadSignal<T>여야 한다. UI가 변경에 반응하도록 지시하는 방법은, signal 타입을 전달하는 것이다.
    // optional props로 전달하려면 prop 앞에 #[prop(optional)]를 붙여야 한다. 너무 길다..
    view! { <progress max=max value=progress></progress> }
}

#[component] // 모든 컴포넌트 정의와 마찬가지로, `#[component]` 매크로로 시작함.
fn App() -> impl IntoView {
    // 모든 타입의 파라미터를 0개 이상 받음 / Leptos `view`에서 반환할 수 있는 모든 것을 포함하는 불투명 타입인 `impl IntoView`를 반환함
    let (count, set_count) = create_signal(0); // React의 state 개념=== Leptos의 signal 개념. (getter, setter) 반환.
    let double_count = move || count() * 2;

    // JSX-like 포맷인 `view` macro를 통해 UI를 정의함.
    view! {
        <button
            on:click=move |_| { set_count(count() + 1) }
            // 클래스의 적용 여부를 아래와 같이 `move || 조건`의 형태로 설정할 수 있다.
            // class=("w-20", move || count() % 2 == 1)
            class="w-[200px]"
        >
            "Click me: "
            {count}
        </button>
        <br/>
        <ProgressBar progress=count/>
        <br/>
        // 그냥 2배를 해주기만 한건데, 이렇게 하면 컴파일이 안된다. 그 이유는 double_count가 ReadSignal<i32> 타입이 아니기 때문.
        // 이 문제를 해결하기 위해서는, ProgressBar의 progress를 제네릭 형태로 받으면 된다.
        <ProgressBar progress=double_count/>
    }

    // count라는 현재 signal을 받아올 때,  nightly Rust에서는 count()로도 사용 가능. 원래는 count.get()임.
    // nightly Rust에서는 {move || count.get()} 대신에 {count}만으로도 현재 signal을 불러와서 사용할 수 있음.

    // class:red=move || count() % 2 == 1 -> count가 홀수일 경우 class에 red를 추가.
    // `class:` 어트리뷰트는
    // 1. class name을 콜론 뒤에 받고
    // 2. 그 뒤에는 bool value, 혹은 bool을 리턴하는 함수를 받는다.
    // class=("red", move || count() % 2 == 1) 와 같이 튜플 형태로도 사용할 수 있음.
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> })
}
