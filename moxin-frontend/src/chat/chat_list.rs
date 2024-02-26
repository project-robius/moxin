use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    import crate::shared::styles::*;
    import crate::shared::widgets::*;
    import crate::chat::shared::*;

    Divider = <VerticalFiller> {
        height: 1,
        show_bg: true,
        draw_bg: {
            color: #ccc
        }
    }

    ChatMessage = <View> {
        width: Fill,
        height: Fit,
        flow: Down,
        spacing: 10,

        main = <View> {
            width: Fill,
            height: Fit,

            flow: Right,
            spacing: 10,

            role = <ChatRole> {}
            content = <View> {
                width: Fill,
                height: Fill,

                align: {x: 0.0, y: 0.5},
                padding: 7,

                label = <Label> {
                    
                    draw_text:{
                        text_style: <REGULAR_FONT>{},
                        color: #000
                    }
                }
            }
        }

        <Divider> {
            margin: {bottom: 10}
        }
    }

    ChatList = <View> {
        width: Fill,
        height: Fill,
        flow: Down,
        spacing: 5,
        padding: 20

        show_bg: true,
        draw_bg: {
            color: #eee8,
        }

        <ChatMessage> {
            main = { content = {
                label = {
                    text: "blah blah blah"
                }
            }}
        }
        <ChatMessage> {
            main = {
                role = {
                    label = {
                        text: "AI"
                    }
                }
                content = {
                    label = {
                        text: "blah blah blah"
                    }
                }
            }
        }
    }
}