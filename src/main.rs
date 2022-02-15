mod sort;

use std::sync::Once;
use std::thread::sleep;
use std::time::Duration;
use std::cell::RefCell;

use wasm_bindgen::prelude::*;
use yew::prelude::*;
use wasm_bindgen::JsCast;
use sort::*;


use gloo::{events::EventListener, timers::callback::*, console::log};

use rand::thread_rng;
use rand::seq::SliceRandom;



const DATA_AMOUNT: usize = 50;

/* Inputs {
    None = 0,
    Bubble,
    Quick,
    Merge,
    Insert,
} */
struct App {
    data: Vec<usize>,
    mode: u8,
    running: bool,
    interval: Interval,
    bubble_sort_h: BubbleSort,
    insertion_sort_h: InsertionSort,
}

enum  Msg {
    SetModeBubble,
    SetModeQuick,
    SetModeMerge,
    SetModeInsertion,
    SetRunningTrue,
    SetRunningFalse,
    Tick,
    Pass,
}


impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let tmp: Vec<usize> = (1..=DATA_AMOUNT).collect();

        let callback = ctx.link().callback(|_| Msg::Tick);
        let interval = Interval::new(200, move || callback.emit(()));

        Self {
            data: tmp.clone(),
            mode: 0,
            running: false,
            interval: interval,
            bubble_sort_h: BubbleSort::new(tmp.clone()),
            insertion_sort_h: InsertionSort::new(tmp.clone()),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetModeBubble => {
                self.running = false;
                self.mode = 1;

                let mut tmp: Vec<usize> = (1..=DATA_AMOUNT).collect();
                tmp.shuffle(&mut thread_rng());
                self.data = tmp.clone();
                self.bubble_sort_h = BubbleSort::new(self.data.clone());
                true
            },

            Msg::SetModeQuick => {
                self.running = false;
                self.mode = 2;

                let mut tmp: Vec<usize> = (1..=DATA_AMOUNT).collect();
                tmp.shuffle(&mut thread_rng());
                self.data = tmp.clone();
                self.bubble_sort_h = BubbleSort::new(self.data.clone());
                true
            },

            Msg::SetModeMerge => {
                self.running = false;
                self.mode = 3;

                let mut tmp: Vec<usize> = (1..=DATA_AMOUNT).collect();
                tmp.shuffle(&mut thread_rng());
                self.data = tmp.clone();
                self.bubble_sort_h = BubbleSort::new(self.data.clone());
                true
            },

            Msg::SetModeInsertion => {
                self.running = false;
                self.mode = 4;

                let mut tmp: Vec<usize> = (1..=DATA_AMOUNT).collect();
                tmp.shuffle(&mut thread_rng());
                self.data = tmp.clone();
                self.insertion_sort_h = InsertionSort::new(self.data.clone());
                true
            },

            Msg::SetRunningTrue => {
                self.running = true;
                let callback = ctx.link().callback(|_| Msg::Tick);
                self.interval = Interval::new(10, move || callback.emit(()));
                true
            },

            Msg::SetRunningFalse => {
                self.running = false;
                let callback = ctx.link().callback(|_| Msg::Pass);
                self.interval = Interval::new(10000, move || callback.emit(()));
                true
            }

            Msg::Tick => {
                if self.running {
                    match self.mode {
                        1 => {
                            if self.bubble_sort_h.step() {
                                self.running = false;
                                let callback = ctx.link().callback(|_| Msg::Pass);
                                self.interval = Interval::new(10000, move || callback.emit(()));
                            } else {

                            }
                            self.data = self.bubble_sort_h.vec.clone();
                            true
                        },
                        2 => {
                            true
                        },
                        3 => {
                            true
                        },
                        4 => {
                            if self.insertion_sort_h.step() {
                                self.running = false;
                                let callback = ctx.link().callback(|_| Msg::Pass);
                                self.interval = Interval::new(10000, move || callback.emit(()));
                            } else {

                            }
                            self.data = self.insertion_sort_h.vec.clone();
                            true
                        },
                        _ => true,
                    }
                } else {
                    true
                }
            },
            Msg::Pass => true,
            _ => true
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut items: Vec<Html> = vec!();
        for val in self.data.iter() {
            let mut text = String::new();
            for _i in 0..*val {
                text.push_str("■■■");
            }
            items.push(html!(<div style="display: flex;"><div class="bar">{text}</div></div>));
        }

        html! {
            <div>
                <div class="navbar">
                    <button class="navbar-button" onclick={ctx.link().callback(|_| Msg::SetModeBubble)}>{ "Bubble sort" }</button>
                    <button class="navbar-button" onclick={ctx.link().callback(|_| Msg::SetModeQuick)}>{ "Quick sort" }</button>
                    <button class="navbar-button" onclick={ctx.link().callback(|_| Msg::SetModeMerge)}>{ "Merge sort" }</button>
                    <button class="navbar-button" onclick={ctx.link().callback(|_| Msg::SetModeInsertion)}>{ "Insertion sort" }</button>
                    <button class="navbar-button" onclick={ctx.link().callback(|_| Msg::SetRunningTrue)}>{ "Start" }</button>
                    <button class="navbar-button" onclick={ctx.link().callback(|_| Msg::SetRunningFalse)}>{ "Stop" }</button>
                </div>
                <div class="display-bars">
                    {items}
                </div>
            </div>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {
    }

    fn destroy(&mut self, _ctx: &Context<Self>) {}
}

fn main() {
    yew::start_app::<App>();
}