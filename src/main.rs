
mod synth;

use leptos::*;
use web_sys::{window, HtmlDivElement};
fn main() {
    console_error_panic_hook::set_once();
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
    let (main_synth, set_main_synth) = create_signal(synth::Synth::new());

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
            <main class="flex flex-col items-center my-5">
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
                                            <div class="flex flex-col ">
                                                <Card label={label} checked=checked handler_on_click={|_|{}} />
                                                {
                                                    checked.then(||{
                                                        view!{
                                                            <p class="text-6xl text-center py-5">{"|"}</p>
                                                            <Card label=(scale_pattern.iter().position(|r|*r==i).unwrap()+1).to_string() checked={false} handler_on_click={|_|{}} />
                                                        }
                                                    })
                                                }
                                            </div>
                                        }
                                    }
                                />
                            </div>
                        }
                    })
                }
            </main>
            <section class="flex flex-col items-center my-5">
                {
                    move || key_note_number.get().is_some().then(|| {
                        let key_note_number = key_note_number.get().unwrap();

                        view!{
                            <h2>{"Chords"}</h2>
                            <div class="flex">
                                <div class="flex flex-col">
                                    <For
                                        each=||{0..7}
                                        key=|&i|{i}
                                        children=move|i|{
                                            view!{
                                                <div class="h-24 aspect-square flex items-center">
                                                    <p class="text-center w-full text-6xl font-thin">{get_roman_number_musical(i+1)}</p>
                                                </div>
                                            }
                                        }
                                    />
                                </div>
                                <ChordCardList differences=&[0, 2, 4] label_fn=|d|get_roman_number_musical(*d.first().unwrap()).to_string() /> // Major / Minor
                                <ChordCardList differences=&[0, 2, 4, 6]  label_fn=|d|format!("{} 7", get_roman_number_musical(*d.first().unwrap())) /> // 7th
                                <ChordCardList differences=&[0, 2, 4, 8]  label_fn=|d|format!("{} add9", get_roman_number_musical(*d.first().unwrap())) /> // add9
                                <ChordCardList differences=&[0, 2, 4, 6, 8]  label_fn=|d|format!("{} 9", get_roman_number_musical(*d.first().unwrap())) /> // 9th
                                <ChordCardList differences=&[0, 3, 4]  label_fn=|d|format!("{} sus4", get_roman_number_musical(*d.first().unwrap()))/> // sus4
                                <ChordCardList differences=&[0, 2, 4, 5]  label_fn=|d|format!("{} 6", get_roman_number_musical(*d.first().unwrap()))/> // 6th
                            </div>
                        }
                    })
                }
            </section>
        </div>
    }
}

#[component]
fn Card(label: String, checked: bool, handler_on_click: impl Fn(leptos::ev::MouseEvent) + 'static ) -> impl IntoView {
    let class = format!("block shadow-md rounded-lg h-24 aspect-[3/4] flex items-center justify-center {}", if checked {"bg-slate-300 text-black"} else {"bg-white text-black"});
    view! {
        <div class={class} on:click=handler_on_click>
            <p class="text-2xl h-min w-min">{label}</p>
        </div>
    }
}

#[component]
fn ChordCard(label: String, caption: String, handler_on_click: impl Fn(leptos::ev::MouseEvent) + 'static ) -> impl IntoView {
    view! {
        <div class="h-24 w-48 flex items-center p-2">
            <button on:click=handler_on_click class="block w-full h-full text-center bg-white rounded-lg font-thin">
                <span class="text-4xl">{label}</span><br />
                <span class="text-2xl">{caption}</span>
            </button>
        </div>
    }
}

#[component]
fn ChordCardList<T>(differences: &'static [u8], label_fn: T) -> impl IntoView  where T: Fn(Vec<u8>) -> String + 'static{
    view! {
        <div class="flex flex-col">
            <For
                each=||{0..7}
                key=|&i|{i}
                children=move|i|{
                    let numbers = differences.iter().map(|d| (i + d) % 7 + 1).collect::<Vec<_>>();
                    let caption = numbers.iter().map(|x|x.to_string()).collect::<Vec<_>>().join("-");
                    let label = label_fn(numbers);
                    view!{
                        <ChordCard label={label} caption={caption} handler_on_click={|_|{
                            let synth = synth::Synth::new();
                            synth.play(440.);
                        }} />
                    }
                }
            />
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

fn get_roman_number_musical(number: u8) -> &'static str{
    match number {
        1 => "I",
        2 => "II",
        3 => "III",
        4 => "IV",
        5 => "V",
        6 => "VI",
        7 => "VII",
        _ => panic!("Invalid number"),
    }
}