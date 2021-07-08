use yew_emoji_picker::{EmojiPicker, EmojiPickerProps};

fn main() {
    let props = EmojiPickerProps {
        emoji_callback: yew::Callback::Callback(std::rc::Rc::new(|_| {})),
    };

    yew::start_app_with_props::<EmojiPicker>(props);
}
