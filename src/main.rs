use leptos::*;
use web_sys::HtmlDivElement;
fn main() {
    mount_to_body(|| view! { <App /> } );
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Scale {
    Major,
    Minor,
}

#[component]
fn App() -> impl IntoView {

    let (key_note_number, set_key_note_number) = create_signal(Some(60));
    let (scale, set_scale) = create_signal(Scale::Major);

    let horizontal_scroll_handler = |event: leptos::ev::WheelEvent| {

        // マウスパッドでの横スクロールを許容する
        if event.delta_y().abs() < event.delta_x().abs() {
            return;
        }
    
        let target: HtmlDivElement = event_target(&event);
        target.set_scroll_left(target.scroll_left() + event.delta_y() as i32);
        
    };

    view! {
        <div class="m-5">
            <header>
                <p class="text-8xl">{"Seorist"}</p>
                <div class="flex gap-2 overflow-x-scroll cursor-pointer py-3" on:wheel=horizontal_scroll_handler>               
                    <For 
                        each = move || {(0..=127-12).map(move|i|(i, key_note_number.get().is_some_and(|t|t==i) ))}
                        key = |&i|{i}
                        children = move |(i, checked)| {
                            let note_name = get_note_name(i);
                            let label = format!("{}{}", note_name, i as i8/12-1);
                            view! {
                                <Card label={label} checked=checked handler_on_click={move|_|{set_key_note_number.set(Some(i))}} />
                            }
                        }
                    />
                </div>
            </header>
            <main class="flex flex-col items-center">
                {
                    move || key_note_number.get().is_some().then(|| {
                        let key_note_number = key_note_number.get().unwrap();

                        let major_span_class = format!(
                            "px-2 cursor-pointer {}",
                            if scale.get() == Scale::Major {"font-medium"} else { "" }
                        );

                        let minor_span_class = format!(
                            "px-2 cursor-pointer {}",
                            if scale.get() == Scale::Minor {"font-medium"} else { "" }
                        );

                        let scale_pattern = match scale.get() {
                            Scale::Major => vec![0, 2, 4, 5, 7, 9, 11],
                            Scale::Minor => vec![0, 2, 3, 5, 7, 8, 10],
                        };
                        
                        view! {
                            <h2>{"Choose the scale."}</h2>
                            <p class="my-4">
                                <span class={major_span_class} on:click=move|_|set_scale.set(Scale::Major) >major</span>
                                <span>{"/"}</span>
                                <span class={minor_span_class} on:click=move|_|set_scale.set(Scale::Minor) >minor</span>
                            </p>
                            <div class="flex gap-2">
                                <For 
                                    each=||{0..12}
                                    key=|&i|{i}
                                    children=move|i|{
                                        let note_name = get_note_name(key_note_number+i);
                                        let label = format!("{}{}", note_name, (key_note_number+i) as i8/12-1);
                                        let checked = scale_pattern.contains(&(i%12));
                                        view! {
                                            <div>
                                                <Card label={label} checked=checked handler_on_click={|_|{}} />
                                            </div>
                                        }
                                    }
                                />
                            </div>
                        }
                    })
                }
            </main>
        </div>
    }
}

#[component]
fn Card(label: String, checked: bool, handler_on_click: impl Fn(leptos::ev::MouseEvent) + 'static ) -> impl IntoView {
    let class = format!("block shadow-md rounded-lg h-24 aspect-[3/4] flex items-center justify-center {}", if checked {"bg-black text-white"} else {"bg-white text-black"});
    view! {
        <div class={class} on:click=handler_on_click>
            <p class="text-2xl h-min w-min">{label}</p>
        </div>
    }
}

fn get_note_name(key_number: u8) -> &'static str{
    match key_number%12 {
        0 => "C",
        1 => "C#",
        2 => "D",
        3 => "D#",
        4 => "E",
        5 => "F",
        6 => "F#",
        7 => "G",
        8 => "G#",
        9 => "A",
        10 => "A#",
        11 => "B",
        _ => panic!("Invalid key number"),
    }
}