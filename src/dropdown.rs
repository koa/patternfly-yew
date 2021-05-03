use crate::{AsClasses, Avatar, Button, Divider, Icon, Variant};
use yew::html::ChildrenRenderer;
use yew::prelude::*;
use yew::virtual_dom::{VChild, VComp};

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Position {
    Left,
    Right,
    Top,
}

impl Default for Position {
    fn default() -> Self {
        Self::Left
    }
}

impl AsClasses for Position {
    fn as_classes(&self) -> Classes {
        match self {
            Self::Left => Classes::new(),
            Self::Right => Classes::from(&["pf-m-right"][..]),
            Self::Top => Classes::from(&["pf-m-top"][..]),
        }
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub position: Position,
    #[prop_or_default]
    pub icon: Option<Icon>,
    #[prop_or_default]
    pub text: String,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub plain: bool,

    pub toggle: Html,
    #[prop_or_default]
    pub toggle_style: Option<String>,

    #[prop_or_default]
    pub children: ChildrenRenderer<DropdownChildVariant>,
}

pub struct Dropdown {
    props: Props,
    link: ComponentLink<Self>,

    expanded: bool,
}

pub enum Msg {
    Toggle,
}

impl Component for Dropdown {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            expanded: false,
            props,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Toggle => {
                self.expanded = !self.expanded;
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let mut classes = Classes::from("pf-c-dropdown");

        classes = classes.extend(self.props.position.as_classes());

        if self.expanded {
            classes.push("pf-m-expanded");
        }

        let onclick = self.link.callback(|_| Msg::Toggle);

        let variant = match self.props.plain {
            true => Variant::Plain,
            false => Variant::None,
        };

        return html! {
            <div class=classes>
                <Button
                    class="pf-c-dropdown__toggle"
                    style=self.props.toggle_style.clone()
                    variant=variant
                    r#type="button"
                    disabled=self.props.disabled
                    onclick=onclick
                    >
                    { self.props.toggle.clone() }
                </Button>
                <div class="pf-c-dropdown__menu" hidden=!self.expanded>
                    <ul>
                    { for self.props.children.iter() }
                    </ul>
                </div>
            </div>
        };
    }
}

// toggle

#[derive(Clone, PartialEq, Properties)]
pub struct ToggleProps {
    #[prop_or_default]
    pub image: Option<Html>,
    #[prop_or_default]
    pub text: String,
    #[prop_or_default]
    pub icon: Option<Icon>,
}

pub struct DropdownToggle {
    props: ToggleProps,
}

impl Component for DropdownToggle {
    type Message = ();
    type Properties = ToggleProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        return html! {
            <>
                { if let Some(ref image) = self.props.image {html!{
                    <span class="pf-c-dropdown__toggle-image">
                        { image.clone() }
                    </span>
                }} else { html!{} }}
                <span class="pf-c-dropdown__toggle-text">
                    { &self.props.text }
                </span>
                <span class="pf-c-dropdown__toggle-icon">
                    { if let Some(icon) = self.props.icon {html!{
                        icon
                    }} else { html!{
                        Icon::CaretDown
                    }} }
                </span>
            </>
        };
    }
}

// child

#[derive(Clone, PartialEq)]
pub enum DropdownChild {
    Item(<DropdownItem as Component>::Properties),
    Divider(<Divider as Component>::Properties),
    Group(<DropdownItemGroup as Component>::Properties),
    Text(<DropdownItemText as Component>::Properties),
}

impl From<DropdownItemProps> for DropdownChild {
    fn from(props: DropdownItemProps) -> Self {
        DropdownChild::Item(props)
    }
}

impl From<()> for DropdownChild {
    fn from(_: ()) -> Self {
        DropdownChild::Divider(())
    }
}

impl From<DropdownItemGroupProps> for DropdownChild {
    fn from(props: DropdownItemGroupProps) -> Self {
        DropdownChild::Group(props)
    }
}

impl From<DropdownItemTextProps> for DropdownChild {
    fn from(props: DropdownItemTextProps) -> Self {
        DropdownChild::Text(props)
    }
}

// variant

#[derive(PartialEq, Clone)]
pub struct DropdownChildVariant {
    props: DropdownChild,
}

impl<CHILD> From<VChild<CHILD>> for DropdownChildVariant
where
    CHILD: Component,
    CHILD::Properties: Into<DropdownChild>,
{
    fn from(vchild: VChild<CHILD>) -> Self {
        Self {
            props: vchild.props.into(),
        }
    }
}

impl Into<Html> for DropdownChildVariant {
    fn into(self) -> Html {
        match self.props {
            DropdownChild::Item(props) => {
                VComp::new::<DropdownItem>(props, NodeRef::default(), None).into()
            }
            DropdownChild::Divider(props) => {
                VComp::new::<Divider>(props, NodeRef::default(), None).into()
            }
            DropdownChild::Group(props) => {
                VComp::new::<DropdownItemGroup>(props, NodeRef::default(), None).into()
            }
            DropdownChild::Text(props) => {
                VComp::new::<DropdownItemText>(props, NodeRef::default(), None).into()
            }
        }
    }
}

// Item

#[derive(Clone, PartialEq, Properties)]
pub struct DropdownItemProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub href: String,
    #[prop_or_default]
    pub onclick: Option<Callback<()>>,
}

#[derive(Clone, PartialEq)]
pub struct DropdownItem {
    props: DropdownItemProps,
}

impl Component for DropdownItem {
    type Message = ();
    type Properties = DropdownItemProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let action = if let Some(onclick) = &self.props.onclick {
            html! {
                <Button
                    class="pf-c-dropdown__menu-item"
                    onclick=onclick.clone().reform(|_|{})
                    >
                    { for self.props.children.iter() }
                </Button>
            }
        } else {
            html! {
                <a
                    class="pf-c-dropdown__menu-item"
                    href=self.props.href.clone()>{ for self.props.children.iter() }</a>
            }
        };

        return html! {
            <li>{action}</li>
        };
    }
}

// Group

#[derive(Clone, PartialEq, Properties)]
pub struct DropdownItemGroupProps {
    #[prop_or_default]
    pub children: ChildrenRenderer<DropdownChildVariant>,
}

#[derive(Clone, PartialEq)]
pub struct DropdownItemGroup {
    props: DropdownItemGroupProps,
}

impl Component for DropdownItemGroup {
    type Message = ();
    type Properties = DropdownItemGroupProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        return html! {
            <>
            { for self.props.children.iter().map(|c|{
                html!{
                    <section class="pf-c-dropdown__group">
                        { c }
                    </section>
                }
            })}
            </>
        };
    }
}

// Text

#[derive(Clone, PartialEq, Properties)]
pub struct DropdownItemTextProps {
    #[prop_or_default]
    pub children: Children,
}

#[derive(Clone, PartialEq)]
pub struct DropdownItemText {
    props: DropdownItemTextProps,
}

impl Component for DropdownItemText {
    type Message = ();
    type Properties = DropdownItemTextProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        return html! {
            <div class="pf-c-dropdown__menu-item pf-m-text">
            { for self.props.children.iter() }
            </div>
        };
    }
}

// kebab toggle

pub struct KebabToggle {}

impl Component for KebabToggle {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        return html! {
            <DropdownToggle icon=Icon::EllipsisV/>
        };
    }
}

// user toggle

#[derive(Clone, PartialEq, Properties)]
pub struct UserToggleProps {
    #[prop_or_default]
    pub name: String,
    #[prop_or_default]
    pub src: String,
}

pub struct UserToggle {
    props: UserToggleProps,
}

impl Component for UserToggle {
    type Message = ();
    type Properties = UserToggleProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        return html! {
            <DropdownToggle
                image=html!{
                    <Avatar src=self.props.src.clone()/>
                }
                text=&self.props.name
                />
        };
    }
}
