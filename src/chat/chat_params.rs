use makepad_widgets::*;

use crate::data::store::Store;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    import crate::shared::styles::*;
    import crate::shared::widgets::FadeView;
    import crate::shared::widgets::MoxinButton;
    import crate::shared::widgets::MoxinSlider;
    import crate::shared::widgets::MoxinSwitch;
    import crate::shared::widgets::MoxinTextInput;
    import makepad_draw::shader::std::*;


    // TODO: Use proper icons
    ICON_CLOSE_PANEL = dep("crate://self/resources/icons/open_left_panel.svg")
    ICON_OPEN_PANEL = dep("crate://self/resources/icons/close_left_panel.svg")

    ChatParamsActions = <View> {
        height: Fit
        flow: Right

        <View> {
            width: Fill
            height: Fit
        }


        close_panel_button = <MoxinButton> {
            width: Fit,
            height: Fit,
            icon_walk: {width: 20, height: 20},
            draw_icon: {
                svg_file: (ICON_CLOSE_PANEL),
                fn get_color(self) -> vec4 {
                    return #475467;
                }
            }
        }

        open_panel_button = <MoxinButton> {
            width: Fit,
            height: Fit,
            visible: false,
            icon_walk: {width: 20, height: 20},
            draw_icon: {
                svg_file: (ICON_OPEN_PANEL),
                fn get_color(self) -> vec4 {
                    return #475467;
                }
            }
        }
    }

    ChatParams = {{ChatParams}} {
        flow: Overlay,
        width: Fit,
        height: Fill,

        main_content = <FadeView> {
            width: 300
            height: Fill
            <View> {
                width: Fill
                height: Fill
                padding: {top: 70, left: 25.0, right: 25.0}
                spacing: 35
                flow: Down
                show_bg: true
                draw_bg: {
                    //color: #F2F4F7
                    color: #67e8f9
                }

                <Label> {
                    draw_text: {
                        text_style: <BOLD_FONT>{font_size: 12}
                        color: #000
                    }
                    text: "Inference Parameters"
                }

                <View> {
                    flow: Down
                    spacing: 24

                    temperature = <MoxinSlider> {
                        default: 1.0
                        text: "Temperature"
                        min: 0.0
                        max: 2.0
                    }

                    top_p = <MoxinSlider> {
                        text: "Top P"
                        min: 0.0
                        max: 1.0
                    }

                    <View> {
                        flow: Right
                        height: Fit
                        width: Fill
                        align: {y: 0.5}
                        padding: {left: 4}
                        <Label> {
                            draw_text: {
                                text_style: <BOLD_FONT>{font_size: 10},
                                color: #000
                            }
                            text: "Stream"
                        }
                        <Filler> {}
                        stream = <MoxinSwitch> {}
                    }

                    max_tokens = <MoxinSlider> {
                        text: "Max Tokens"
                        min: 100.0
                        max: 2048.0
                        step: 1.0
                    }

                    <View> {
                        flow: Down
                        height: Fit
                        width: Fill
                        spacing: 12
                        padding: {left: 4}
                        <Label> {
                            draw_text: {
                                text_style: <BOLD_FONT>{font_size: 10},
                                color: #000
                            }
                            text: "Stop"
                        }
                        stop = <MoxinTextInput> {
                            width: Fill,
                            height: 65,

                            empty_message: ""
                            draw_bg: {
                                radius: 5.0
                                color: #fff
                            }
                            draw_text: {
                                text_style: {font_size: 10},
                            }
                        }
                    }

                    frequency_penalty = <MoxinSlider> {
                        text: "Frequency Penalty"
                        min: 0.0
                        max: 1.0
                    }

                    presence_penalty = <MoxinSlider> {
                        text: "Presence Penalty"
                        min: 0.0
                        max: 1.0
                    }
                }
            }
        }

        <ChatParamsActions> {
            padding: {top: 58, left: 25, right: 25}
        }

        animator: {
            panel = {
                default: show,
                show = {
                    redraw: true,
                    from: {all: Forward {duration: 0.3}}
                    ease: ExpDecay {d1: 0.80, d2: 0.97}
                    apply: {main_content = { width: 300, draw_bg: {opacity: 1.0} }}
                }
                hide = {
                    redraw: true,
                    from: {all: Forward {duration: 0.3}}
                    ease: ExpDecay {d1: 0.80, d2: 0.97}
                    apply: {main_content = { width: 110, draw_bg: {opacity: 0.0} }}
                }
            }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct ChatParams {
    #[deref]
    view: View,

    #[animator]
    animator: Animator,
}

impl Widget for ChatParams {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);

        if self.animator_handle_event(cx, event).must_redraw() {
            self.redraw(cx);
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let store = scope.data.get::<Store>().unwrap();
        let ip = &store.chats.inferences_params;

        let temperature = self.slider(id!(temperature));
        let top_p = self.slider(id!(top_p));
        let max_tokens = self.slider(id!(max_tokens));
        let frequency_penalty = self.slider(id!(frequency_penalty));
        let presence_penalty = self.slider(id!(presence_penalty));

        temperature.set_value(ip.temperature.into());
        top_p.set_value(ip.top_p.into());
        max_tokens.set_value(ip.max_tokens.into());
        frequency_penalty.set_value(ip.frequency_penalty.into());
        presence_penalty.set_value(ip.presence_penalty.into());

        self.view.draw_walk(cx, scope, walk)
    }
}

impl WidgetMatchEvent for ChatParams {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, scope: &mut Scope) {
        let store = scope.data.get_mut::<Store>().unwrap();
        let ip = &mut store.chats.inferences_params;

        let close = self.button(id!(close_panel_button));
        let open = self.button(id!(open_panel_button));

        if close.clicked(&actions) {
            close.set_visible(false);
            open.set_visible(true);
            self.animator_play(cx, id!(panel.hide));
        }

        if open.clicked(&actions) {
            open.set_visible(false);
            close.set_visible(true);
            self.animator_play(cx, id!(panel.show));
        }

        if let Some(value) = self.slider(id!(temperature)).slided(&actions) {
            ip.temperature = value as f32;
        }

        if let Some(value) = self.slider(id!(top_p)).slided(&actions) {
            ip.top_p = value as f32;
        }

        if let Some(value) = self.slider(id!(max_tokens)).slided(&actions) {
            ip.max_tokens = value as u32;
        }

        if let Some(value) = self.slider(id!(frequency_penalty)).slided(&actions) {
            ip.frequency_penalty = value as f32;
        }

        if let Some(value) = self.slider(id!(presence_penalty)).slided(&actions) {
            ip.presence_penalty = value as f32;
        }
    }
}
