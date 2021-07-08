use emojis::{Emoji, Group};
use yew::prelude::*;

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
            },
            EmojiMsg::Emoji(emoji) => {
                self.props.emoji_callback.emit(emoji);
                false
            }
        }
    }

    fn view(&self) -> Html {
        let smilies = emojis::Group::SmileysAndEmotion.emojis().next().unwrap();

        let smilies_callback = self
            .link
            .callback(|_| EmojiMsg::Group(emojis::Group::SmileysAndEmotion));

        let flags = emojis::Group::Flags.emojis().next().unwrap();
        let flags_callback = self
            .link
            .callback(|_| EmojiMsg::Group(emojis::Group::Flags));

        let activities = emojis::Group::Activities.emojis().next().unwrap();
        let activities_callback = self
            .link
            .callback(|_| EmojiMsg::Group(emojis::Group::Activities));

        let animals = emojis::Group::AnimalsAndNature.emojis().next().unwrap();
        let animals_callback = self
            .link
            .callback(|_| EmojiMsg::Group(emojis::Group::AnimalsAndNature));

        let food = emojis::Group::FoodAndDrink.emojis().next().unwrap();
        let food_callback = self
            .link
            .callback(|_| EmojiMsg::Group(emojis::Group::FoodAndDrink));

        let objects = emojis::Group::Objects.emojis().next().unwrap();
        let objects_callback = self
            .link
            .callback(|_| EmojiMsg::Group(emojis::Group::Objects));

        let people = emojis::Group::PeopleAndBody.emojis().next().unwrap();
        let people_callback = self
            .link
            .callback(|_| EmojiMsg::Group(emojis::Group::PeopleAndBody));

        let travel = emojis::Group::TravelAndPlaces.emojis().next().unwrap();
        let travel_callback = self
            .link
            .callback(|_| EmojiMsg::Group(emojis::Group::TravelAndPlaces));

        let symbols = emojis::Group::Symbols.emojis().next().unwrap();
        let symbols_callback = self
            .link
            .callback(|_| EmojiMsg::Group(emojis::Group::Symbols));

        let iter = self.iter_emojis();

        let link = &self.link;

        html! {
            <div style="display: flex; flex-wrap: wrap;">
                <div style="flex: 1 1 100%; display: flex; flex-wrap: wrap;">
                <button style="margin: 1% 2%; flex: 1 1 8%;" class="emoji-header-buttons" onclick=smilies_callback>{ smilies }</button>
                <button style="margin: 1% 2%; flex: 1 1 8%;" class="emoji-header-buttons" onclick=flags_callback>{ flags }</button>
                <button style="margin: 1% 2%; flex: 1 1 8%;" class="emoji-header-buttons" onclick=activities_callback>{ activities }</button>
                <button style="margin: 1% 2%; flex: 1 1 8%;" class="emoji-header-buttons" onclick=animals_callback>{ animals }</button>
                <button style="margin: 1% 2%; flex: 1 1 8%;" class="emoji-header-buttons" onclick=food_callback>{ food }</button>
                <button style="margin: 1% 2%; flex: 1 1 8%;" class="emoji-header-buttons" onclick=objects_callback>{ objects }</button>
                <button style="margin: 1% 2%; flex: 1 1 8%;" class="emoji-header-buttons" onclick=people_callback>{ people }</button>
                <button style="margin: 1% 2%; flex: 1 1 8%;" class="emoji-header-buttons" onclick=travel_callback>{ travel }</button>
                <button style="margin: 1% 2%; flex: 1 1 8%;" class="emoji-header-buttons" onclick=symbols_callback>{ symbols }</button>
                </div>


                { for iter.map(|e| { let e2 = e.clone(); let callback = link.callback(move |_| EmojiMsg::Emoji(e2)); html! { <button onclick=callback style="margin: 1% 2%; flex: 1 1 20%;" class="emoji-buttons"> {e.as_str() } </button> }}) }
            </div>
        }
    }
}
