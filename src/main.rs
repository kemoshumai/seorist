
mod synth;

use leptos::*;
use web_sys::HtmlDivElement;
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

    let (key_note_number, set_key_note_number) = create_signal(None);
    let (scale, set_scale) = create_signal(Scale::Major);
    let main_synth = store_value(synth::Synth::new());

    let horizontal_scroll_handler = |event: leptos::ev::WheelEvent| {

        // マウスパッドでの横スクロールを許容する
        if event.delta_y().abs() < event.delta_x().abs() {
            return;
        }
    
        let target: HtmlDivElement = event_target(&event);
        target.set_scroll_left(target.scroll_left() + event.delta_y() as i32);
        
    };

    view! {
        <div class="overflow-none w-screen max-w-screen">
            <header class="overflow-none">
                <p class="text-8xl p-5">{"Seorist"}</p>
                <div class="overflow-x-scroll  py-3">
                    <div class="flex gap-2 cursor-pointer" on:wheel=horizontal_scroll_handler>               
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
                </div>
            </header>
            <div class="w-screen overflow-x-scroll">
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
            </div>
            <section class="flex flex-col items-center my-5">
                {

                    let synthesizer = main_synth;

                    move || match key_note_number.get() {
                        Some(key_note_number) => {
                            let scale_pattern = match scale.get() {
                                Scale::Major => vec![0, 2, 4, 5, 7, 9, 11],
                                Scale::Minor => vec![0, 2, 3, 5, 7, 8, 10],
                            };
                            let note_numbers_in_scale = scale_pattern.iter().map(|r|(key_note_number+r)).collect::<Vec<u8>>();
    
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
                                    <ChordCardList 
                                        differences=&[0, 2, 4] 
                                        label_fn=|d|get_roman_number_musical(*d.first().unwrap()).to_string() 
                                        synthesizer={synthesizer} 
                                        note_numbers_in_scale={note_numbers_in_scale.clone()}
                                    /> // Major / Minor
                                    <ChordCardList 
                                        differences=&[0, 2, 4, 6]  
                                        label_fn=|d|format!("{} 7", get_roman_number_musical(*d.first().unwrap())) 
                                        synthesizer={synthesizer} 
                                        note_numbers_in_scale={note_numbers_in_scale.clone()}
                                    /> // 7th
                                    <ChordCardList 
                                        differences=&[0, 2, 4, 8]  
                                        label_fn=|d|format!("{} add9", get_roman_number_musical(*d.first().unwrap())) 
                                        synthesizer={synthesizer} 
                                        note_numbers_in_scale={note_numbers_in_scale.clone()}
                                    /> // add9
                                    <ChordCardList 
                                        differences=&[0, 2, 4, 6, 8]  
                                        label_fn=|d|format!("{} 9", get_roman_number_musical(*d.first().unwrap())) 
                                        synthesizer={synthesizer} 
                                        note_numbers_in_scale={note_numbers_in_scale.clone()}
                                    /> // 9th
                                    <ChordCardList 
                                        differences=&[0, 3, 4]  
                                        label_fn=|d|format!("{} sus4", get_roman_number_musical(*d.first().unwrap())) 
                                        synthesizer={synthesizer} 
                                        note_numbers_in_scale={note_numbers_in_scale.clone()}
                                    /> // sus4
                                    <ChordCardList 
                                        differences=&[0, 2, 4, 5]  
                                        label_fn=|d|format!("{} 6", get_roman_number_musical(*d.first().unwrap())) 
                                        synthesizer={synthesizer} 
                                        note_numbers_in_scale={note_numbers_in_scale.clone()}
                                    /> // 6th
                                    <div class="flex flex-col">
                                        <For
                                            each=||{0..7}
                                            key=|&i|{i}
                                            children=move|i|{
                                                let label = format!("{} aug", get_roman_number_musical(i+1));
                                                let caption = format!("{}-{}-{}#", (i%7)+1, (i+2)%7 + 1, (i+4)%7 + 1);
                                                let note_numbers: Vec<u8> = [
                                                    *(note_numbers_in_scale.get((i%7) as usize).unwrap()),
                                                    *(note_numbers_in_scale.get(((i+2)%7) as usize).unwrap()),
                                                    note_numbers_in_scale.get(((i+4)%7) as usize).unwrap()+1,
                                                ].to_vec();
                                                view!{
                                                    <ChordCard label={label} caption={caption} synthesizer=synthesizer note_numbers=note_numbers />
                                                }
                                            }
                                        />
                                    </div>// aug
                                </div>
                            }.into_view()
                        },
                        None => {
                            view!{
                                <div>
                                    <p>{"Please select a key note."}</p>
                                </div>
                            }.into_view()
                        }
                    }
                }
            </section>
        </div>
    }
}

#[component]
fn Card(label: String, checked: bool, handler_on_click: impl Fn(leptos::ev::MouseEvent) + 'static ) -> impl IntoView {
    let class = format!("block shadow-md rounded-lg h-32 w-24 text-center flex items-center justify-center {}", if checked {"bg-slate-300 text-black"} else {"bg-white text-black"});
    view! {
        <div class={class} on:click=handler_on_click>
            <p class="text-2xl h-min w-24">{label}</p>
        </div>
    }
}

#[component]
fn ChordCard(label: String, caption: String, synthesizer: StoredValue<synth::Synth>, note_numbers: Vec<u8> ) -> impl IntoView {
    let handler_on_click = move |_|{
        synthesizer.with_value(|synthesizer|{
            synthesizer.play(&note_numbers.iter().map(|n|note_number_to_frequency(*n)).collect::<Vec<_>>());
        });
    };
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
fn ChordCardList<T>(differences: &'static [u8], label_fn: T, synthesizer: StoredValue<synth::Synth>, note_numbers_in_scale: Vec<u8>) -> impl IntoView  where T: Fn(Vec<u8>) -> String + 'static{
    view! {
        <div class="flex flex-col">
            <For
                each=||{0..7}
                key=|&i|{i}
                children=move|i|{
                    let numbers = differences.iter().map(|d| (i + d) % 7 + 1).collect::<Vec<_>>();
                    let caption = numbers.iter().map(|x|x.to_string()).collect::<Vec<_>>().join("-");
                    let label = label_fn(numbers.clone());
                    let note_numbers = numbers.iter().map(|n|*note_numbers_in_scale.get(*n as usize - 1).unwrap()).collect::<Vec<_>>();
                    view!{
                        <ChordCard label={label} caption={caption} synthesizer=synthesizer note_numbers=note_numbers />
                    }
                }
            />
        </div>
    }
}


// MIDIノート番号から周波数を計算する
fn note_number_to_frequency(note_number: u8) -> f32 {
    440. * 2_f32.powf((note_number as f32 - 69.) / 12.)
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