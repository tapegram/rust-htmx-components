use rscx::{component, html, props};

use htmx_components::server::form::{GridCell, GridLayout, Label, Select, SelectOption, TextInput};

// ### Components ###

#[component]
pub fn FormPlayground() -> String {
    html! {
        <section class="py-8">
            <h2 class="text-xl font-bold">Form Playground</h2>
            <GridLayout class="mt-10">
                <GridCell span=3>
                    <Label for_input="first_name" error=true>
                        First name
                    </Label>
                    <TextInput
                        name="first_name"
                        value="Mary"
                        error=Some("Oops! Name can not be empty.".into())
                    />
                </GridCell>
                <GridCell span=3>
                    <Label for_input="last_name">Last name</Label>
                    <TextInput name="last_name" autocomplete="family-name" value="Sue" />
                </GridCell>
                <GridCell span=3>
                    <Label for_input="color" error=true>Color</Label>
                    <Select error=Some("Blue sucks!".into())>
                        <SelectOption>Red</SelectOption>
                        <SelectOption selected=true>Blue</SelectOption>
                        <SelectOption>Green</SelectOption>
                    </Select>
                </GridCell>
                <GridCell span=3>
                    <Label for_input="notes">Notes</Label>
                    <TextInput input_type="textarea" name="notes" value="Some bogus notes" />
                </GridCell>
            </GridLayout>
        </section>
    }
}
