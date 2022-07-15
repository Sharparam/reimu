initSidebarItems({"constant":[["NUM_POINTER_BUTTONS","Number of pointer buttons supported by egui, i.e. the number of possible states of [`PointerButton`]."]],"enum":[["Align","left/center/right or top/center/bottom alignment for e.g. anchors and layouts."],["CursorIcon","A mouse cursor icon."],["Direction","Layout direction, one of `LeftToRight`, `RightToLeft`, `TopDown`, `BottomUp`."],["Event","An input event generated by the integration."],["FontFamily","Which style of font: [`Monospace`][`FontFamily::Monospace`] or [`Proportional`][`FontFamily::Proportional`]."],["Key","Keyboard keys."],["PointerButton","Mouse button (or similar for touch input)"],["Shape","A paint primitive such as a circle or a piece of text. Coordinates are all screen space points (not physical pixels)."],["TextStyle","One of a few categories of styles of text, e.g. body, button or heading."],["TextureId","What texture to use in a [`Mesh`] mesh."],["TouchPhase","In what phase a touch event is in."],["WidgetText","This is how you specify text for a widget."],["WidgetType","The different types of built-in widgets in egui"]],"fn":[["__run_test_ctx","For use in tests; especially doctests."],["__run_test_ui","For use in tests; especially doctests."],["lerp","Linear interpolation."],["pos2","`pos2(x,y) == Pos2::new(x, y)`"],["remap","Linearly remap a value from one range to another, so that when `x == from.start()` returns `to.start()` and when `x == from.end()` returns `to.end()`."],["remap_clamp","Like [`remap`], but also clamps the value so that the returned value is always in the `to` range."],["vec2","`vec2(x,y) == Vec2::new(x, y)`"],["warn_if_debug_build","Helper function that adds a label when compiling with debug assertions enabled."]],"macro":[["egui_assert","An assert that is only active when `egui` is compiled with the `extra_asserts` feature or with the `extra_debug_asserts` feature in debug builds."],["github_link_file","Create a `Hyperlink` to the current [`file!()`] on github."],["github_link_file_line","Create a `Hyperlink` to the current [`file!()`] (and line) on Github"],["trace","Show debug info on hover when [`Context::set_debug_on_hover`] has been turned on."]],"mod":[["color","Color conversions and types."],["containers","Containers are pieces of the UI which wraps other pieces of UI. Examples: [`Window`], [`ScrollArea`], [`Resize`], [`SidePanel`], etc."],["layers","Handles paint layers, i.e. how things are sometimes painted behind or in front of other things."],["menu","Menu bar functionality (very basic so far)."],["mutex","Helper module that wraps some Mutex types with different implementations."],["output","All the data egui returns to the backend at the end of each frame."],["special_emojis","egui supports around 1216 emojis in total. Here are some of the most useful: ∞⊗⎗⎘⎙⏏⏴⏵⏶⏷ ⏩⏪⏭⏮⏸⏹⏺■▶📾🔀🔁🔃 ☀☁★☆☐☑☜☝☞☟⛃⛶✔ ↺↻⟲⟳⬅➡⬆⬇⬈⬉⬊⬋⬌⬍⮨⮩⮪⮫ ♡ 📅📆 📈📉📊 📋📌📎📤📥🔆 🔈🔉🔊🔍🔎🔗🔘 🕓🖧🖩🖮🖱🖴🖵🖼🗀🗁🗋🗐🗑🗙🚫❓"],["style","egui theme (spacing, colors, etc)."],["text",""],["util","Miscellaneous tools used by the rest of egui."],["widgets","Widgets are pieces of GUI such as [`Label`], [`Button`], [`Slider`] etc."]],"struct":[["Align2","Two-dimension alignment, e.g. [`Align2::LEFT_TOP`]."],["ClippedMesh","A [`Mesh`] within a clip rectangle."],["Color32","This format is used for space-efficient color representation (32 bits)."],["Context","Your handle to egui."],["CtxRef","A wrapper around `Arc``<`[`Context`]`>`. This is how you will normally create and access a [`Context`]."],["DroppedFile","A file dropped into egui."],["FontData","A `.ttf` or `.otf` file and a font face index."],["FontDefinitions","Describes the font data and the sizes to use."],["FontImage","An 8-bit texture containing font data."],["Galley","Text that has been layed out, ready for painting."],["Grid","A simple grid layout."],["HoveredFile","A file about to be dropped into egui."],["Id","egui tracks widgets frame-to-frame using `Id`s."],["InnerResponse","Returned when we wrap some ui-code and want to return both the results of the inner function and the ui as a whole, e.g.:"],["InputState","Input state that egui updates each frame."],["Layout","The layout of a [`Ui`][`crate::Ui`], e.g. “vertical & centered”."],["Memory","The data that egui persists between frames."],["Modifiers","State of the modifier keys. These must be fed to egui."],["MultiTouchInfo","All you probably need to know about a multi-touch gesture."],["Output","What egui emits each frame. The backend should use this."],["Painter","Helper to paint shapes and text to a specific region on a specific layer."],["PointerState","Mouse or touch state."],["Pos2","A position on screen."],["RawInput","What the integrations provides to egui at the start of each frame."],["Rect","A rectangular region of space."],["Response","The result of adding a widget to a [`Ui`]."],["Rgba","0-1 linear space `RGBA` color with premultiplied alpha."],["RichText","Text and optional style choices for it."],["Sense","What sort of interaction is a widget sensitive to?"],["Stroke","Describes the width and color of a line."],["TextFormat",""],["TouchDeviceId","this is a `u64` as values of this kind can always be obtained by hashing"],["TouchId","Unique identification of a touch occurrence (finger or pen or …). A Touch ID is valid until the finger is lifted. A new ID is used for the next touch."],["Ui","This is what you use to place widgets."],["Vec2","A vector has a direction and length. A [`Vec2`] is often used to represent a size."],["WidgetInfo","Describes a widget such as a [`crate::Button`] or a [`crate::TextEdit`]."]],"trait":[["NumExt","Extends `f32`, `Vec2` etc with `at_least` and `at_most` as aliases for `max` and `min`."]],"type":[["IdMap","`IdMap<V>` is a `HashMap<Id, V>` optimized by knowing that `Id` has good entropy, and doesn’t need more hashing."]]});