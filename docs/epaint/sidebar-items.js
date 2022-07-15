initSidebarItems({"constant":[["WHITE_UV","The UV coordinate of a white region of the texture mesh. The default egui texture has the top-left corner pixel fully white. You need need use a clamping texture sampler for this to work (so it doesn’t do bilinear blending with bottom right corner)."]],"enum":[["Shape","A paint primitive such as a circle or a piece of text. Coordinates are all screen space points (not physical pixels)."],["TextStyle","One of a few categories of styles of text, e.g. body, button or heading."],["TextureId","What texture to use in a [`Mesh`] mesh."]],"fn":[["pos2","`pos2(x,y) == Pos2::new(x, y)`"],["vec2","`vec2(x,y) == Vec2::new(x, y)`"]],"macro":[["epaint_assert","An assert that is only active when `epaint` is compiled with the `extra_asserts` feature or with the `extra_debug_asserts` feature in debug builds."]],"mod":[["color","Color conversions and types."],["mutex","Helper module that wraps some Mutex types with different implementations."],["shape_transform",""],["stats","Collect statistics about what is being painted."],["tessellator","Converts graphics primitives into textured triangles."],["text","Everything related to text, fonts, text layout, cursors etc."],["util",""]],"struct":[["CircleShape","How to paint a circle."],["ClippedMesh","A [`Mesh`] within a clip rectangle."],["ClippedShape","A [`Shape`] within a clip rectangle."],["FontImage","An 8-bit texture containing font data."],["Fonts","The collection of fonts used by `epaint`."],["Galley","Text that has been layed out, ready for painting."],["Mesh","Textured triangles in two dimensions."],["Mesh16","A version of [`Mesh`] that uses 16-bit indices."],["PathShape","A path which can be stroked and/or filled (if closed)."],["Pos2","A position on screen."],["Rect","A rectangular region of space."],["RectShape","How to paint a rectangle."],["Shadow","The color and fuzziness of a fuzzy shape. Can be used for a rectangular shadow with a soft penumbra."],["Stroke","Describes the width and color of a line."],["TextShape","How to paint some text on screen."],["TextureAtlas","Contains font data in an atlas, where each character occupied a small rectangle."],["Vec2","A vector has a direction and length. A [`Vec2`] is often used to represent a size."],["Vertex","The 2D vertex type."]]});