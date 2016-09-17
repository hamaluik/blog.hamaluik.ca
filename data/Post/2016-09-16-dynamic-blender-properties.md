---
title: Dynamic Blender Properties
slug: dynamic-blender-properties
summary: "As part of my most recent adventures in game engine programming, I came across a small problem—I needed a way to edit levels (both their geometry, and the entities within the level and their associated components). Writing an editor to do this is a rather daunting task. Thankfully, Blender is a free, open-source 3D application that is 'easily' extended (well, easy-ish). So, instead of writing my own editor, I can write a Blender addon to make it do what I need it to. First up in that, was presenting an interface for editing which components an object has, and setting the values of each components' attributes. I found this to be more difficult that I expected, thanks to the way Blender handles and presents data. I will show you here how I got things working, as there doesn't seem to be documentation on this and I had to wade through a lot of half-expired forum posts to get things working."
author: kenton
published: 2016-09-16
category: programming
tags: Blender, Python
---

As part of my most recent adventures in game engine programming, I came across a small problem&mdash;I needed a way to edit levels (both their geometry, and the entities within the level and their associated components). Writing an editor to do this is a rather daunting task. Thankfully, [Blender](https://www.blender.org/) is a free, open-source 3D application that is "easily" extended (well, easy-_ish_). So, instead of writing my own editor, I can write a Blender addon to make it do what I need it to. First up in that, was presenting an interface for editing which components an object has, and setting the values of each components' attributes. I found this to be more difficult that I expected, thanks to the way Blender handles and presents data. I will show you here how I got things working, as there doesn't seem to be documentation on this and I had to wade through a lot of half-expired forum posts to get things working.

Basically, in order to display any data in a panel in Blender, you must pre-define the property classes that it will render. The following code listing shows a simple panel which will show up in the object section of the properties panel:

<figure>
    <img src="/assets/images/dynamic-blender-properties/sample-predefined-property.png">
    <figcaption>A sample panel displaying the pre-defined properties in the pre-defined `SamplePropertyGroup` class.</figcaption>
</figure>

```python
import bpy
from bpy.props import FloatProperty, PointerProperty
from bpy.types import PropertyGroup

class SamplePropertyGroup(PropertyGroup):
    a = FloatProperty(name="A", default=5.0)
    b = FloatProperty(name="B", default=42.0)

class SamplePanel(bpy.types.Panel):
    bl_label = "Sample Panel"
    bl_space_type = 'PROPERTIES'
    bl_region_type = 'WINDOW'
    bl_context = "object"

    def draw(self, context):
        layout = self.layout
        obj = context.object
        
        sampleProperty = obj.samplePropertyGroup
        col = layout.column(align=True)
        col.prop(sampleProperty, "a")
        col.prop(sampleProperty, "b")

def register():
    bpy.utils.register_class(SamplePropertyGroup)
    bpy.utils.register_class(SamplePanel)
    bpy.types.Object.samplePropertyGroup = PointerProperty(type=SamplePropertyGroup)

def unregister():
    bpy.utils.unregister_class(SamplePropertyGroup)
    bpy.utils.unregister_class(SamplePanel)
    del bpy.types.Object.samplePropertyGroup

if __name__ == "__main__":
    register()
```

A problem arises, however, when you don't know ahead of time what that `SamplePropertyGroup` class will look like. Or even if it will exist (or what custom property groups _will_ exist). For my use case, I want to have each component exist in blender as it's own subclass of the `PropertyGroup` class, with its list of attributes defined as class members (as relevant `PropertyTypes`). Forunately for us, Python is a scripted language, and comes with some tools to dynamically create classes and interact with them via the [`type()`](https://docs.python.org/2/library/functions.html#type), [`setattr()`](https://docs.python.org/2/library/functions.html#setattr), [`getattr()`](https://docs.python.org/2/library/functions.html#getattr), and [`delattr()`](https://docs.python.org/2/library/functions.html#delattr) built-in functions.