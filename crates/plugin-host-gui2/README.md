# Plugin Host GUI
A test plugin-host for VSTs.

## Stack & crates
The host uses the following libraries to host VSTs:

* `cpal`
* `rust-vst`

It uses `iced` for GUI.

## Organization of the `iced` GUI

* All GUI specific code should b eunder `src/ui`
* Each view should have its own module
* When a view is stateful, there should be a presentation `view.rs` module, and a parent `mod.rs` which acts as a
  controller and wires the model/updates onto the presentation module
* Whenever possible views should define examples on `view::story` and get them added to `examples/storybook`