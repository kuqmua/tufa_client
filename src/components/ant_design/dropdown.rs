use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct DropdownProps {
    pub disabled: bool, //	Whether the dropdown menu is disabled	boolean	-
                        //pub get_popup_container:  //	To set the container of the dropdown menu. The default is to create a div element in body, but you can reset it to the scrolling area and make a relative reposition. Example on CodePen.	Function(triggerNode)	() => document.body
                        //pub overlay: 	//The dropdown menu	Menu | () => Menu	-
                        //pub overlayClassName: //	Class name of the dropdown root element	string	-	3.11.0
                        //pub overlayStyle: 	Style of the dropdown root element	object	-	3.11.0
                        //pub placement:	Placement of popup menu: bottomLeft, bottomCenter, bottomRight, topLeft, topCenter or topRight	String	bottomLeft
                        //pub trigger:	The trigger mode which executes the dropdown action. Note that hover can't be used on touchscreens.	Array<click|hover|contextMenu>	['hover']
                        //pub visible:	Whether the dropdown menu is currently visible	boolean	-
                        //pub onVisibleChange:	Called when the visible state is changed.	Function(visible)
}

#[function_component(Dropdown)]
pub fn dropdown(props: &DropdownProps) -> Html {
    html! {}
}
