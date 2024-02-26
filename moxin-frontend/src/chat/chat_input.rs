use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    import crate::shared::styles::*;
    import crate::chat::shared::*;

    ChatInput = <View> {
        width: Fill,
        height: Fit,

        spacing: 5,

        <ChatRole> {}
        <RoundedView> {
            width: Fill,
            height: Fit,

            align: {x: 0.5, y: 0.5},
            padding: 7,

            show_bg: true,
            draw_bg: {
                color: #ddd,
                border_color: #000,
                border_width: 1.0,
            }

            <TextInput> {
                width: Fill,
                height: Fit,

                empty_message: "Enter a user message..."
                draw_text: {
                    text_style:<REGULAR_FONT>{},

                    fn get_color(self) -> vec4 {
                        return
                        mix(
                            mix(
                                mix(
                                    #x22222255,
                                    #x22222288,
                                    self.hover
                                ),
                                #x222222CC,
                                self.focus
                            ),
                            #3,
                            self.is_empty
                        )
                    }
                }
                draw_cursor: {
                    color: #0000
                    color_focus: #aaa
                }
                draw_select: {
                    color: #0000
                    color_focus: #ccc
                }
            }
        }
    }
}