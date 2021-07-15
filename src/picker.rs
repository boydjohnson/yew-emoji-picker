use emojis::{Emoji, Group};
use yew::prelude::*;

const SMILIES: &str = "😀";
const FLAGS: &str = "🏁";
const ACTIVITIES: &str = "🎃";
const ANIMALS: &str = "🐵";
const FOOD: &str = "🍇";
const OBJECTS: &str = "👓";
const PEOPLE: &str = "👋";
const TRAVEL: &str = "🌍";
const SYMBOLS: &str = "🏧";

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct EmojiPickerProps {
    pub emoji_callback: Callback<Emoji>,
}

#[derive(Debug)]
pub enum EmojiMsg {
    Group(Group),
    Emoji(Emoji),
}

#[derive(Debug)]
pub struct EmojiPicker {
    search: Option<String>,
    link: ComponentLink<Self>,
    props: EmojiPickerProps,
    selected_group: Group,
}

impl EmojiPicker {
    fn iter_emojis(&self) -> impl Iterator<Item = &'static Emoji> {
        self.selected_group.emojis()
    }
}

impl Component for EmojiPicker {
    type Message = EmojiMsg;
    type Properties = EmojiPickerProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        EmojiPicker {
            search: None,
            link,
            props,
            selected_group: Group::SmileysAndEmotion,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            return true;
        }
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            EmojiMsg::Group(group) => {
                self.selected_group = group;
                true
            }
            EmojiMsg::Emoji(emoji) => {
                self.props.emoji_callback.emit(emoji);
                false
            }
        }
    }

    fn view(&self) -> Html {
        let smilies_callback = self
            .link
            .callback(|_| EmojiMsg::Group(emojis::Group::SmileysAndEmotion));

        let flags_callback = self
            .link
            .callback(|_| EmojiMsg::Group(emojis::Group::Flags));

        let activities_callback = self
            .link
            .callback(|_| EmojiMsg::Group(emojis::Group::Activities));

        let animals_callback = self
            .link
            .callback(|_| EmojiMsg::Group(emojis::Group::AnimalsAndNature));

        let food_callback = self
            .link
            .callback(|_| EmojiMsg::Group(emojis::Group::FoodAndDrink));

        let objects_callback = self
            .link
            .callback(|_| EmojiMsg::Group(emojis::Group::Objects));

        let people_callback = self
            .link
            .callback(|_| EmojiMsg::Group(emojis::Group::PeopleAndBody));

        let travel_callback = self
            .link
            .callback(|_| EmojiMsg::Group(emojis::Group::TravelAndPlaces));

        let symbols_callback = self
            .link
            .callback(|_| EmojiMsg::Group(emojis::Group::Symbols));

        let iter = self.iter_emojis();

        let link = &self.link;

        html! {
            <div style="display: flex; flex-wrap: wrap;">
                <div style="flex: 1 1 100%; display: flex; flex-wrap: wrap;">
                <button style="margin: 1% 2%; flex: 1 1 8%;" class="emoji-header-buttons" onclick=smilies_callback>{ SMILIES }</button>
                <button style="margin: 1% 2%; flex: 1 1 8%;" class="emoji-header-buttons" onclick=flags_callback>{ FLAGS }</button>
                <button style="margin: 1% 2%; flex: 1 1 8%;" class="emoji-header-buttons" onclick=activities_callback>{ ACTIVITIES }</button>
                <button style="margin: 1% 2%; flex: 1 1 8%;" class="emoji-header-buttons" onclick=animals_callback>{ ANIMALS }</button>
                <button style="margin: 1% 2%; flex: 1 1 8%;" class="emoji-header-buttons" onclick=food_callback>{ FOOD }</button>
                <button style="margin: 1% 2%; flex: 1 1 8%;" class="emoji-header-buttons" onclick=objects_callback>{ OBJECTS }</button>
                <button style="margin: 1% 2%; flex: 1 1 8%;" class="emoji-header-buttons" onclick=people_callback>{ PEOPLE }</button>
                <button style="margin: 1% 2%; flex: 1 1 8%;" class="emoji-header-buttons" onclick=travel_callback>{ TRAVEL }</button>
                <button style="margin: 1% 2%; flex: 1 1 8%;" class="emoji-header-buttons" onclick=symbols_callback>{ SYMBOLS }</button>
                </div>


                { for iter.map(|e| { let e2 = e.clone(); let callback = link.callback(move |_| EmojiMsg::Emoji(e2)); html! { <button onclick=callback style="margin: 1% 2%; flex: 1 1 20%;" class="emoji-buttons"> {e.as_str() } </button> }}) }
            </div>
        }
    }
}
