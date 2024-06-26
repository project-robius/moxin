use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    import crate::shared::styles::*;
    import crate::chat::chat_panel::ChatPanel;
    import crate::chat::chat_history::ChatHistory;
    import crate::chat::chat_params::ChatParams;

    ChatScreen = {{ChatScreen}} {
        width: Fill,
        height: Fill,
        spacing: 50,

        <View> {
            width: Fit,
            height: Fill,

            chat_history = <ChatHistory> {}
        }

        <View> {
            width: Fill,
            height: Fill,
            align: {x: 0.5},
            padding: {top: 48, bottom: 48 }

            chat_panel = <ChatPanel> {}
        }
        /*
        <View> {
            show_bg: true,
            draw_bg: {
                color: #000
            }
            width: 10,
            height: Fill,
            margin: {top: 48, right: 48, bottom: 48}
        }
         */

        <View> {
            width: Fit,
            height: Fill,
            chat_params = <ChatParams> {}
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct ChatScreen {
    #[deref]
    view: View,
}

impl Widget for ChatScreen {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        // TODO This check is actually copied from Makepad view.rs file
        // It's not clear why it's needed here, but without this line
        // the "View all files" link in Discover section does not work after visiting the chat screen
        if self.visible || !event.requires_visibility() {
            self.view.handle_event(cx, event, scope);
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
