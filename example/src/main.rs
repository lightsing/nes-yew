use nes_yew::*;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
struct ColProps {
    #[prop_or(1)]
    size: usize,
    children: Children,
}

#[function_component(Col)]
fn col(ColProps { size, children }: &ColProps) -> Html {
    html! {
        <div style={format!("flex: {}", size)}>{children.clone()}</div>
    }
}

#[derive(Properties, PartialEq)]
struct RowProps {
    children: Children,
}

#[function_component(Row)]
fn row(RowProps { children }: &RowProps) -> Html {
    html! {
        <div style=r#"display: flex; flex-flow: row wrap;"#>{children.clone()}</div>
    }
}

#[function_component]
fn App() -> Html {
    let radio_options = vec![
        RadioOption {
            value: "yes".into(),
            label: "Yes".into(),
        },
        RadioOption {
            value: "no".into(),
            label: "No".into(),
        },
    ];
    let selected_radio_value = use_state(|| AttrValue::from("yes"));
    let on_radio_value_change = {
        let selected_radio_value = selected_radio_value.clone();
        Callback::from(move |value| selected_radio_value.set(value))
    };

    let box_one_checked = use_state(|| false);
    let box_two_checked = use_state(|| false);
    let toggle_check_box_one = {
        let box_one_checked = box_one_checked.clone();
        Callback::from(move |_| {
            let value = !*box_one_checked;
            box_one_checked.set(value);
        })
    };
    let toggle_check_box_two = {
        let box_two_checked = box_two_checked.clone();
        Callback::from(move |_| {
            let value = !*box_two_checked;
            box_two_checked.set(value);
        })
    };

    let gen_input_callback = |input: &UseStateHandle<AttrValue>| {
        let input = input.clone();
        Callback::from(move |e: HtmlInputElement| input.set(e.value().into()))
    };
    let text_input = use_state(|| AttrValue::from(""));
    let success_input = use_state(|| AttrValue::from(""));
    let warning_input = use_state(|| AttrValue::from(""));
    let error_input = use_state(|| AttrValue::from(""));
    let textarea_input = use_state(|| AttrValue::from(""));

    html! {
        <div id="root">
            <h1>{"nes-yew"}</h1>
            <h4>
              {"A Yew component library based on the awesome "}
              <a href="https://github.com/nostalgic-css/NES.css">{"nes.css"}</a>
            </h4>
            <Container>
              <p>{"Containers"}</p>
              <Row>
                <Col>
                  <Container>{"Regular"}</Container>
                </Col>
                <Col>
                  <Container rounded={true}>{"Rounded"}</Container>
                </Col>
                <Col>
                  <Container title="With Title">{"Title for this one"}</Container>
                </Col>
              </Row>
              <Row>
                <Col>
                  <Container dark={true}>{"Dark"}</Container>
                </Col>
                <Col>
                  <Container centered={true}>{"Centered text for this one!"}</Container>
                </Col>
              </Row>
            </Container>

            <Container title="Buttons">
              <Button>{"Regular"}</Button>
              <Button primary={true}>{"Primary"}</Button>
              <Button success={true}>{"Success"}</Button>
              <Button warning={true}>{"Warning"}</Button>
              <Button error={true}>{"Error"}</Button>
              <Button disabled={true}>{"Disabled"}</Button>
            </Container>

            <Container title="Radios">
              <Radios
                selected_value={(*selected_radio_value).clone()}
                options={radio_options}
                on_value_change={on_radio_value_change}
              />
            </Container>

            <Container title="Checkboxes">
              <Checkbox
                checked={*box_one_checked}
                label="Box One"
                on_select={toggle_check_box_one}
              />

              <Checkbox
                checked={*box_two_checked}
                label="Box Two"
                on_select={toggle_check_box_two}
              />
            </Container>

            <Container title="Text Inputs">
                <TextInput
                    label="Label"
                    placeholder="Text placeholder"
                    value={(*text_input).clone()}
                    on_change={gen_input_callback(&text_input)}
                />
                <TextInput
                    label="Success"
                    label_inline={true}
                    success={true}
                    value={(*success_input).clone()}
                    on_change={gen_input_callback(&success_input)}
                />
                <TextInput
                    label="Warning"
                    label_inline={true}
                    warning={true}
                    value={(*warning_input).clone()}
                    on_change={gen_input_callback(&warning_input)}
                />
                <TextInput
                    label="Error"
                    label_inline={true}
                    error={true}
                    value={(*error_input).clone()}
                    on_change={gen_input_callback(&error_input)}
                />
                <TextArea
                    label="Text Area"
                    value={(*error_input).clone()}
                    on_change={gen_input_callback(&textarea_input)}
                />
            </Container>

            <Container title="Avatars">
                <Avatar src="https://www.gravatar.com/avatar" small={true} />
                <Avatar src="https://www.gravatar.com/avatar" />
                <Avatar src="https://www.gravatar.com/avatar" medium={true} />
                <Avatar src="https://www.gravatar.com/avatar" large={true} />
                <Avatar src="https://www.gravatar.com/avatar" small={true} rounded={true} />
                <Avatar src="https://www.gravatar.com/avatar" rounded={true} />
                <Avatar src="https://www.gravatar.com/avatar" medium={true} rounded={true} />
                <Avatar src="https://www.gravatar.com/avatar" large={true} rounded={true} />
            </Container>

            <Container title="Balloons">
              <Row>
                <Col>
                  <div style="display: flex">
                    <Sprite sprite={SpriteKind::Bcrikko} style="align-self: flex-end" />
                    <Balloon style="margin: 2rem; max-width: 400px;" from_left={true}>
                      {"Here's one from the left!"}
                    </Balloon>
                  </div>
                </Col>
                <Col>
                  <div style="display: flex">
                    <Balloon
                      style="margin: 2rem; max-width: 400px;"
                      from_right={true}
                    >
                      {"And one from the right!"}
                    </Balloon>
                    <Sprite sprite={SpriteKind::Bcrikko} style="align-self: flex-end" />
                  </div>
                </Col>
              </Row>
            </Container>

            <Container title="Lists">
                <p>{"Lists"}</p>
                <List>
                    <li>{"Item 1"}</li>
                    <li>{"Item 2"}</li>
                    <li>{"Item 3"}</li>
                </List>

                <List solid={true}>
                    <li>{"Solid 1"}</li>
                    <li>{"Solid 2"}</li>
                    <li>{"Solid 3"}</li>
                </List>
            </Container>

            <Container title="Tables">
                <Table>
                    <thead>
                      <tr>
                        <th>{"Table"}</th>
                        <th>{"Table"}</th>
                      </tr>
                    </thead>
                    <tbody>
                      <tr>
                        <td>{"Row 1 Cell 1"}</td>
                        <td>{"Row 1 Cell 2"}</td>
                      </tr>

                      <tr>
                        <td>{"Row 2 Cell 1"}</td>
                        <td>{"Row 2 Cell 2"}</td>
                      </tr>
                    </tbody>
                </Table>
                <div style="height: 20px" />
                <Table bordered={true}>
                    <thead>
                      <tr>
                        <th>{"Table bordered"}</th>
                        <th>{"Table bordered"}</th>
                      </tr>
                    </thead>
                    <tbody>
                      <tr>
                        <td>{"Row 1 Cell 1"}</td>
                        <td>{"Row 1 Cell 2"}</td>
                      </tr>

                      <tr>
                        <td>{"Row 2 Cell 1"}</td>
                        <td>{"Row 2 Cell 2"}</td>
                      </tr>
                    </tbody>
                </Table>
                <div style="height: 20px" />
                <Table bordered={true} centered={true} style="width: 700px">
                    <thead>
                      <tr>
                        <th>{"Table centered"}</th>
                        <th>{"Table centered"}</th>
                      </tr>
                    </thead>
                    <tbody>
                      <tr>
                        <td>{"Row 1 Cell 1"}</td>
                        <td>{"Row 1 Cell 2"}</td>
                      </tr>

                      <tr>
                        <td>{"Row 2 Cell 1"}</td>
                        <td>{"Row 2 Cell 2"}</td>
                      </tr>
                    </tbody>
                </Table>
                <div style="height: 20px" />
                <Table bordered={true} centered={true} dark={true} style="width: 700px">
                    <thead>
                      <tr>
                        <th>{"Table dark"}</th>
                        <th>{"Table dark"}</th>
                      </tr>
                    </thead>
                    <tbody>
                      <tr>
                        <td>{"Row 1 Cell 1"}</td>
                        <td>{"Row 1 Cell 2"}</td>
                      </tr>

                      <tr>
                        <td>{"Row 2 Cell 1"}</td>
                        <td>{"Row 2 Cell 2"}</td>
                      </tr>
                    </tbody>
                </Table>
            </Container>

            <Container title="Progress Bars">
              <Progress value={90} max={100} />
              <Progress value={80} max={100} primary={true} />
              <Progress value={70} max={100} success={true} />
              <Progress value={60} max={100} warning={true} />
              <Progress value={50} max={100} error={true} />
              <Progress value={40} max={100} pattern={true} />
            </Container>

            <Container title="Icons">
                <Container title="Size and state">
                    <Icon icon={IconKind::Star} small={true} />
                    <Icon icon={IconKind::Star} />
                    <Icon icon={IconKind::Star} medium={true} />
                    <Icon icon={IconKind::Star} large={true} />
                    <Icon icon={IconKind::Star} half={true} large={true} />
                    <Icon icon={IconKind::Star} empty={true} large={true} />
                    <Icon icon={IconKind::Star} transparent={true} large={true} />
                </Container>
                <Container title="Sprites">
                    {
                        SpriteKind::iter().map(|icon| {
                            html! {
                                <Sprite sprite={icon} style="margin: 5px" />
                            }
                        }).collect::<Html>()
                    }
                </Container>
            </Container>

            <Container title="Controller Icons">
            {
                ControllerKind::iter().map(|controller| {
                    html!{
                        <ControllerIcon controller={controller} style="margin: 5px" />
                    }
                }).collect::<Html>()
            }
            </Container>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
