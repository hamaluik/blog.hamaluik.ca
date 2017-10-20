---
title: "Creating Custom Unity Timeline Tracks"
slug: creating-custom-unity-timeline-tracks
author: kenton
tags: [Unity]
#published: 2017-10-20
#meta-image: /assets/images/collision-engine-3d-detection/meta-preview.jpg
large-meta-image: true
#preview-image: /assets/images/collision-engine-3d-detection/meta-preview.jpg
preview-summary: "Timelines are a new feature of Unity 2017.1+, and they are incredibly powerful! Unfortunately, the documentation behind them is a little slack—especially on how to create your own timeline tracks + clips for properties than the built-in Unity timeline tracks can't handle."
---

Timelines are a new feature of Unity 2017.1+, and they are incredibly powerful! Unfortunately, the documentation behind them is a little slack—especially on how to create your own timeline tracks + clips for properties than the built-in Unity timeline tracks can't handle.

I recently had to create a custom timeline track to swap out the mesh of an object. Basically, I wanted an object to use mesh A for the first half of an animation, then switch to mesh B for the second half. This can be accomplished with scripting or other various hacks, but it provides a simple demonstration for creating a custom track to be used in the timeline which could be used wherever, pretty flexibly.

<figure>
    <figcaption>The result: controlling the mesh of a <code>MeshFilter</code> using clips on the timeline!</figcaption>
</figure>

The most confusing thing I found when I started this process is that there are **4** classes / scripts you're required to make to get anything working with the timeline. The reasoning for this can be explained by how timeline _actually_ works—timelines are really just graphs which mix data to produce a result:

<figure>
    <figcaption>Timelines are graphs!</figcaption>
</figure>

...talk about the timeline<=>graph relationship...

With that in mind, here are the four classes we'll have to create to get things working:

1. Clip(which gets dragged around on the timeline)
2. Data(each clip has some data associated with it—in our case it will be which mesh we want to be using while that clip is active)
3. Mixer(there can be multiple clips playing at the same time, the mixer is responsible for taking all of those and producing a single sane output)
4. Track(mostly boilerplate for linking everything up and displaying it all in the timeline editor)

These four classes are all fairly tightly-coupled, but it generally makes sense to start with the data.

## MeshChangerData.cs

The data for our example is going to be rather straightforward: we just need to store a reference to the Mesh we want our eventual `MeshFilter` to use:

```csharp
using System;
using UnityEngine;
using UnityEngine.Playables;

[Serializable]
public class MeshChangerData : PlayableBehaviour {
    public Mesh TargetMesh;
}
```

It's important to note two things here:

1. The class must be `[Serializable]` as we need Unity to save the data for each clip in the timeline asset
2. We inherit from [`PlayableBehaviour`](https://docs.unity3d.com/ScriptReference/Playables.PlayableBehaviour.html)(which gives us access to a bunch of handy timeline-related functions if we need).

## MeshChangerClip.cs

```csharp
using System;
using UnityEngine;
using UnityEngine.Playables;
using UnityEngine.Timeline;

[Serializable]
public class MeshChangerClip : PlayableAsset, ITimelineClipAsset {
    public MeshChangerData template = new MeshChangerData();

    public ClipCaps clipCaps {
        get { return ClipCaps.None; }
    }

    public override Playable CreatePlayable(PlayableGraph graph, GameObject owner) {
        var playable = ScriptPlayable<MeshChangerData>.Create(graph, template);
        return playable;
    }
}

```

## MeshChangerMixer.cs

This is the meat and potatoes of our custom timeline stuff—it handles taking in our list of clips and setting the appropriate mesh on the track's `MeshFilter`.

```csharp
using System;
using UnityEngine;
using UnityEngine.Playables;
using UnityEngine.Timeline;

public class MeshChangerMixer : PlayableBehaviour {
    private Mesh _defaultMesh;
    private MeshFilter _targetFilter;
    bool _firstFrameHappened;

    public override void ProcessFrame(Playable playable, FrameData info, object playerData) {
        _targetFilter = playerData as MeshFilter;
        if(_targetFilter == null)
            return;

        if(!_firstFrameHappened) {
            _defaultMesh = _targetFilter.sharedMesh;
            _firstFrameHappened = true;
        }

        int inputCount = playable.GetInputCount();
        float greatestWeight = 0f;
        int currentInputs = 0;

        for(int i = 0; i < inputCount; i++) {
            float inputWeight = playable.GetInputWeight(i);
            ScriptPlayable<MeshPlayable> inputPlayable =(ScriptPlayable<MeshPlayable>)playable.GetInput(i);
            MeshPlayable input = inputPlayable.GetBehaviour();

            if(inputWeight > greatestWeight) {
                _targetFilter.sharedMesh = input.TargetMesh;
                greatestWeight = inputWeight;
            }

            if(!Mathf.Approximately(inputWeight, 0f))
                currentInputs++;
        }

        if(currentInputs < 1) {
            _targetFilter.sharedMesh = _defaultMesh;
        }
    }

    public override void OnGraphStop(Playable playable) {
        if(_targetFilter != null) _targetFilter.sharedMesh = _defaultMesh;
        _firstFrameHappened = false;
    }
}
```

## MeshChangerTrack.cs

```csharp
using UnityEngine;
using UnityEngine.Playables;
using UnityEngine.Timeline;
using System.Collections.Generic;

[TrackColor(0.1394896f, 0.4411765f, 0.3413077f)]
[TrackClipType(typeof(MeshClip))]
[TrackBindingType(typeof(MeshFilter))]
public class MeshTrack : TrackAsset {
    public override Playable CreateTrackMixer(PlayableGraph graph, GameObject go, int inputCount) {
        return ScriptPlayable<MeshChangerMixer>.Create(graph, inputCount);
    }

    public override void GatherProperties(PlayableDirector director, IPropertyCollector driver) {
#if UNITY_EDITOR
        MeshRenderer trackBinding = director.GetGenericBinding(this) as MeshRenderer;
        if(trackBinding == null)
            return;

        var serializedObject = new UnityEditor.SerializedObject(trackBinding);
        var iterator = serializedObject.GetIterator();
        while(iterator.NextVisible(true)) {
            if(iterator.hasVisibleChildren)
                continue;

            driver.AddFromName<MeshRenderer>(trackBinding.gameObject, iterator.propertyPath);
        }
#endif
        base.GatherProperties(director, driver);
    }
}

```