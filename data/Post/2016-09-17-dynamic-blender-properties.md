---
title: Dynamic Blender Properties
slug: dynamic-blender-properties
author: kenton
published: 2016-09-17 12:58:22
category: programming
tags: Blender, Python
preview_image: /assets/images/dynamic-blender-properties/sample-finished-result.png
preview_summary: "As part of my most recent adventures in game engine programming, I came across a small problem—I needed a way to edit levels (both their geometry, and the entities within the level and their associated components). Writing an editor to do this is a rather daunting task. Thankfully, Blender is a free, open-source 3D application that is 'easily' extended (well, easy-ish). So, instead of writing my own editor, I can write a Blender addon to make it do what I need it to. First up in that, was presenting an interface for editing which components an object has, and setting the values of each components' attributes. I found this to be more difficult that I expected, thanks to the way Blender handles and presents data. I will show you here how I got things working, as there doesn't seem to be documentation on this and I had to wade through a lot of half-expired forum posts to get things working."
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

Simply put, we can use `type()` to go from this:

```python
class SamplePropertyGroup(PropertyGroup):
    a = FloatProperty(name="A", default=5.0)
    b = FloatProperty(name="B", default=42.0)
```

to this:

```python
SamplePropertyGroup = type(
    "SamplePropertyGroup",
    (PropertyGroup,),
    {
        "a": FloatProperty(name="A", default=5.0),
        "b": FloatProperty(name="B", default=42.0)
    })
```

We can easily script this to be more dynamic, such that given a dictionary (which we can parse from a JSON file at runtime) that looks like this:

```python
propertyGroupLayouts = {
    "Health": [
        { "name": "current", "type": "float" },
        { "name": "max", "type": "float" }
    ],
    "Character": [
        { "name": "first_name", "type": "string" },
        { "name": "last_name", "type": "string" }
    ]
}
```

We can dynamically create our property groups and register them with Blender by simply iterating the dictionary and constructing classes as we go:

```python
# iterate over our list of property groups
for groupName, attributeDefinitions in propertyGroupLayouts.items():
    # build the attribute dictionary for this group
    attributes = {}
    for attributeDefinition in attributeDefinitions:
        attType = attributeDefinition['type']
        attName = attributeDefinition['name']
        if attType == 'float':
            attributes[attName] = FloatProperty(name=attName.title())
        elif attType == 'string':
            attributes[attName] = StringProperty(name=attName.title())
        else:
            raise TypeError('Unsupported type (%s) for %s on %s!' % (attType, attName, groupName))

    # now build the property group class
    propertyGroupClass = type(groupName, (PropertyGroup,), attributes)

    # register it with Blender
    bpy.utils.register_class(propertyGroupClass)

    # apply it to all Objects
    setattr(bpy.types.Object, groupName, PointerProperty(type=propertyGroupClass))
```

Now, whenever we register we just call the above code, and our `PropertyGroup` classes will be defined and registered in Blender as properties of all objects! Don't forget to clean up the registered objects in `unregister()`!

Finally, to actually show our properties in a Blender panel is relatively straightforward:

```python
class SamplePanel(bpy.types.Panel):
    bl_label = "Sample Panel"
    bl_space_type = 'PROPERTIES'
    bl_region_type = 'WINDOW'
    bl_context = "object"

    def draw(self, context):
        layout = self.layout
        obj = context.object
        
        # use our layout definition to dynamically create our panel items
        for groupName, attributeDefinitions in propertyGroupLayouts.items():
            # get the instance of our group
            # dynamic equivalent of `obj.samplePropertyGroup` from before
            propertyGroup = getattr(obj, groupName)

            # start laying this group out
            col = layout.column()
            col.label(groupName)

            # loop through all the attributes and show them
            for attributeDefinition in attributeDefinitions:
                col.prop(propertyGroup, attributeDefinition["name"])

            # draw a separation between groups
            layout.separator()
```

And there you have it, dynamic properties (created at run-time, not write-time) in Blender!

<figure>
    <img src="/assets/images/dynamic-blender-properties/sample-finished-result.png">
    <figcaption>What the finished result will look like.</figcaption>
</figure>

Here is the full source code for the above example:

```python
import bpy
from bpy.props import FloatProperty, StringProperty, PointerProperty
from bpy.types import PropertyGroup

# TODO: load dynamically at runtime from a JSON file!
bpy.propertyGroupLayouts = {
    "Health": [
        { "name": "current", "type": "float" },
        { "name": "max", "type": "float" }
    ],
    "Character": [
        { "name": "first_name", "type": "string" },
        { "name": "last_name", "type": "string" }
    ]
}
bpy.samplePropertyGroups = {}

class SamplePanel(bpy.types.Panel):
    bl_label = "Sample Panel"
    bl_space_type = 'PROPERTIES'
    bl_region_type = 'WINDOW'
    bl_context = "object"

    def draw(self, context):
        layout = self.layout
        obj = context.object
        
        # use our layout definition to dynamically create our panel items
        for groupName, attributeDefinitions in bpy.propertyGroupLayouts.items():
            # get the instance of our group
            # dynamic equivalent of `obj.samplePropertyGroup` from before
            propertyGroup = getattr(obj, groupName)

            # start laying this group out
            col = layout.column()
            col.label(groupName)

            # loop through all the attributes and show them
            for attributeDefinition in attributeDefinitions:
                col.prop(propertyGroup, attributeDefinition["name"])

            # draw a separation between groups
            layout.separator()

def register():
    # register the panel class
    bpy.utils.register_class(SamplePanel)
    
    # iterate over our list of property groups
    for groupName, attributeDefinitions in bpy.propertyGroupLayouts.items():
        # build the attribute dictionary for this group
        attributes = {}
        for attributeDefinition in attributeDefinitions:
            attType = attributeDefinition['type']
            attName = attributeDefinition['name']
            if attType == 'float':
                attributes[attName] = FloatProperty(name=attName.title())
            elif attType == 'string':
                attributes[attName] = StringProperty(name=attName.title())
            else:
                raise TypeError('Unsupported type (%s) for %s on %s!' % (attType, attName, groupName))

        # now build the property group class
        propertyGroupClass = type(groupName, (PropertyGroup,), attributes)

        # register it with Blender
        bpy.utils.register_class(propertyGroupClass)

        # apply it to all Objects
        setattr(bpy.types.Object, groupName, PointerProperty(type=propertyGroupClass))
        
        # store it for later
        bpy.samplePropertyGroups[groupName] = propertyGroupClass

def unregister():
    # unregister the panel class
    bpy.utils.unregister_class(SamplePanel)

    # unregister our components
    try:
        for key, value in bpy.samplePropertyGroups.items():
            delattr(bpy.types.Object, key)
            bpy.utils.unregister_class(value)
    except UnboundLocalError:
        pass
    bpy.samplePropertyGroups = {}

if __name__ == "__main__":
    register()
```

There are of course many more things that need to be done to make this a fully working system, but hopefully this can help you get started. I will continue working on my engine tools addon and release it when it's ready (you can follow progress at [BlazingMammothGames/mammoth_blender_tools](https://github.com/BlazingMammothGames/mammoth_blender_tools) if you really like). The tool, when done, will hopefully server as a solid example for adapting Blender to your own needs in the future. For now, if you run into any trouble, don't hesitate to ask for help in the comments!