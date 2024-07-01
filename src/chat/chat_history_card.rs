use crate::data::{chats::chat::ChatID, store::Store};
use chrono::{DateTime, Local, NaiveDateTime, TimeZone};

use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    import crate::shared::styles::*;
    import crate::shared::widgets::MoxinButton;
    import crate::chat::shared::ChatAgentAvatar;

    ICON_DELETE = dep("crate://self/resources/icons/delete.svg")

    ChatHistoryCard = {{ChatHistoryCard}} {
        content = <RoundedView> {
            flow: Down
            width: 248
            height: Fit
            padding: 20
            spacing: 12

            cursor: Hand

            draw_bg: {
                color: #fff
                border_width: 1
            }

            <View> {
                width: Fill
                height: Fit
                flow: Right
                spacing: 10
                padding: { top: 4, bottom: 4 }
                margin: 0

                title = <Label> {
                    width: Fill,
                    height: Fit,
                    draw_text:{
                        text_style: <BOLD_FONT>{font_size: 10},
                        color: #000,
                    }
                    text: ""
                }

                delete_chat = <MoxinButton> {
                    width: Fit
                    height: Fit
                    padding: 4
                    margin: { top: -4}
                    icon_walk: {width: 12, height: 12}
                    draw_icon: {
                        svg_file: (ICON_DELETE),
                        fn get_color(self) -> vec4 {
                            return #B42318;
                        }
                    }
                }
            }

            <View> {
                width: Fill
                height: Fit
                align: {y: 1}

                avatar = <ChatAgentAvatar> {
                    width: 30
                    height: 30

                    draw_bg: {
                        radius: 8
                    }
                    avatar_label = {
                        text: ""
                    }
                }

                filler = <View> {width: Fill}

                date = <Label> {
                    width: Fit,
                    height: Fit,
                    draw_text:{
                        text_style: <REGULAR_FONT>{font_size: 10},
                        color: #667085,
                    }
                    text: "5:29 PM, 5/12/24"
                }
            }


        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct ChatHistoryCard {
    #[deref]
    view: View,
    #[rust]
    chat_id: ChatID,
}

impl Widget for ChatHistoryCard {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let store = scope.data.get_mut::<Store>().unwrap();
        let chat = store
            .chats
            .saved_chats
            .iter()
            .find(|c| c.borrow().id == self.chat_id)
            .unwrap();

        if let Some(current_chat_id) = store.chats.get_current_chat_id() {
            let content_view = self.view(id!(content));

            if current_chat_id == self.chat_id {
                let active_border_color = vec3(0.082, 0.522, 0.604);
                content_view.apply_over(
                    cx,
                    live! {
                        draw_bg: {border_color: (active_border_color)}
                    },
                );
            } else {
                let border_color = vec3(0.918, 0.925, 0.941);
                content_view.apply_over(
                    cx,
                    live! {
                        draw_bg: {border_color: (border_color)}
                    },
                );
            }
        }

        let title_label = self.view.label(id!(title));
        title_label.set_text(chat.borrow_mut().get_title());

        let initial_letter = store.get_last_used_file_initial_letter(self.chat_id)
            .unwrap_or('A')
            .to_uppercase()
            .to_string();

        let avatar_label = self.view.label(id!(avatar.avatar_label));
        avatar_label.set_text(&initial_letter);

        let date_label = self.view.label(id!(date));

        // Format date.
        // TODO: Feels wrong to asume the id will always be the date, do smth about this.
        let naive_datetime = NaiveDateTime::from_timestamp_millis(chat.borrow().id as i64)
            .expect("Invalid timestamp");
        let datetime: DateTime<Local> = Local.from_utc_datetime(&naive_datetime);
        let formatted_date = datetime.format("%-I:%M %p, %-d/%m/%y").to_string();

        date_label.set_text(&formatted_date);

        self.view.draw_walk(cx, scope, walk)
    }
}

impl WidgetMatchEvent for ChatHistoryCard {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, scope: &mut Scope) {
        let store = scope.data.get_mut::<Store>().unwrap();
        let widget_uid = self.widget_uid();

        if self.button(id!(delete_chat)).clicked(actions) {
            store.delete_chat(self.chat_id);
            self.redraw(cx);
            return;
        }

        if let Some(fe) = self.view(id!(content)).finger_down(actions) {
            if fe.tap_count == 1 {
                cx.widget_action(
                    widget_uid,
                    &scope.path,
                    ChatHistoryCardAction::ChatSelected(self.chat_id),
                );
                store.select_chat(self.chat_id);
                self.redraw(cx);
            }
        }
    }
}

impl ChatHistoryCard {
    pub fn set_chat_id(&mut self, id: ChatID) {
        self.chat_id = id;
    }
}

impl ChatHistoryCardRef {
    pub fn set_chat_id(&mut self, id: ChatID) -> Result<(), &'static str> {
        let Some(mut inner) = self.borrow_mut() else {
            return Err("Widget not found in the document");
        };

        inner.set_chat_id(id);
        Ok(())
    }
}

#[derive(Clone, DefaultNone, Eq, Hash, PartialEq, Debug)]
pub enum ChatHistoryCardAction {
    None,
    ChatSelected(ChatID),
}
