// Original collect-square SWF fixture. MIT OR Apache-2.0.
use swf::avm1::types::{Action, Push, Value};

fn fixture(extra: Option<Action<'static>>) -> Vec<u8> {
    let mut frames = Vec::new();
    for frame in 0..240 {
        let mut bytes = Vec::new();
        let mut writer = swf::avm1::write::Writer::new(&mut bytes, 7);
        let name = swf::SwfStr::from_utf8_str("s_score");
        let input = swf::SwfStr::from_utf8_str("s_dx");
        let mut actions = vec![Action::Push(Push {
            values: vec![Value::Str(input)],
        })];
        // Real Flash Key.isDown calls: right minus left, eight pixels/frame.
        for key in [39, 37] {
            actions.extend([
                Action::Push(Push {
                    values: vec![
                        Value::Int(key),
                        Value::Int(1),
                        Value::Str(swf::SwfStr::from_utf8_str("Key")),
                    ],
                }),
                Action::GetVariable,
                Action::Push(Push {
                    values: vec![Value::Str(swf::SwfStr::from_utf8_str("isDown"))],
                }),
                Action::CallMethod,
                Action::ToNumber,
            ]);
        }
        actions.extend([
            Action::Subtract,
            Action::Push(Push {
                values: vec![Value::Int(8)],
            }),
            Action::Multiply,
            Action::SetVariable,
            Action::Push(Push {
                values: vec![Value::Str(name)],
            }),
        ]);
        if frame == 0 {
            actions.push(Action::Push(Push {
                values: vec![Value::Int(0)],
            }));
        } else {
            actions.extend([
                Action::Push(Push {
                    values: vec![Value::Str(name)],
                }),
                Action::GetVariable,
                Action::Push(Push {
                    values: vec![Value::Str(input)],
                }),
                Action::GetVariable,
                Action::Add,
                Action::Push(Push {
                    values: vec![Value::Int(100)],
                }),
                Action::RandomNumber,
                Action::Add,
            ]);
        }
        actions.extend([
            Action::SetVariable,
            Action::Push(Push {
                values: vec![Value::Str(name)],
            }),
            Action::GetVariable,
            Action::Trace,
        ]);
        // Move the actual native MovieClip, not a stand-in world representation.
        actions.extend([
            Action::Push(Push {
                values: vec![
                    Value::Str(swf::SwfStr::from_utf8_str("clip_avatar")),
                    Value::Int(0),
                    Value::Str(swf::SwfStr::from_utf8_str("clip_avatar")),
                    Value::Int(0),
                ],
            }),
            Action::GetProperty,
            Action::Push(Push {
                values: vec![Value::Str(input)],
            }),
            Action::GetVariable,
            Action::Add,
            Action::Push(Push {
                values: vec![Value::Int(640)],
            }),
            Action::Add,
            Action::Push(Push {
                values: vec![Value::Int(640)],
            }),
            Action::Modulo,
            Action::SetProperty,
        ]);
        // Collect the gold square on overlap; the real AVM RNG chooses its next x.
        let text = |s: &'static str| Value::Str(swf::SwfStr::from_utf8_str(s));
        let push = |values| Action::Push(Push { values });
        if frame == 0 {
            actions.extend([
                push(vec![text("s_collected"), Value::Int(0)]),
                Action::SetVariable,
            ]);
        }
        actions.extend([
            push(vec![text("s_delta"), text("clip_avatar"), Value::Int(0)]),
            Action::GetProperty,
            push(vec![text("clip_goal"), Value::Int(0)]),
            Action::GetProperty,
            Action::Subtract,
            Action::SetVariable,
            push(vec![text("s_hit"), text("s_delta")]),
            Action::GetVariable,
            push(vec![text("s_delta")]),
            Action::GetVariable,
            Action::Multiply,
            push(vec![Value::Int(1024)]),
            Action::Less,
            Action::ToNumber,
            Action::SetVariable,
            push(vec![text("s_collected"), text("s_collected")]),
            Action::GetVariable,
            push(vec![text("s_hit")]),
            Action::GetVariable,
            Action::Add,
            Action::SetVariable,
            push(vec![text("s_collected")]),
            Action::GetVariable,
            Action::Trace,
            push(vec![
                text("clip_goal"),
                Value::Int(0),
                text("clip_goal"),
                Value::Int(0),
            ]),
            Action::GetProperty,
            push(vec![Value::Int(1), text("s_hit")]),
            Action::GetVariable,
            Action::Subtract,
            Action::Multiply,
            push(vec![Value::Int(500)]),
            Action::RandomNumber,
            push(vec![Value::Int(50)]),
            Action::Add,
            push(vec![text("s_hit")]),
            Action::GetVariable,
            Action::Multiply,
            Action::Add,
            Action::SetProperty,
        ]);
        if let Some(action) = &extra {
            actions.push(action.clone());
        }
        actions.push(Action::End);
        for action in actions {
            writer.write_action(&action).unwrap();
        }
        frames.push(bytes);
    }
    let mut tags = vec![
        swf::Tag::SetBackgroundColor(swf::Color {
            r: 20,
            g: 27,
            b: 45,
            a: 255,
        }),
        swf::Tag::DefineShape(Box::new(square(
            1,
            swf::Color {
                r: 64,
                g: 220,
                b: 210,
                a: 255,
            },
        ))),
        swf::Tag::DefineShape(Box::new(square(
            3,
            swf::Color {
                r: 255,
                g: 185,
                b: 70,
                a: 255,
            },
        ))),
        swf::Tag::DefineSprite(swf::Sprite {
            id: 2,
            num_frames: 1,
            tags: vec![place(1, 1, None, 0, 0), swf::Tag::ShowFrame],
        }),
        swf::Tag::DefineSprite(swf::Sprite {
            id: 4,
            num_frames: 1,
            tags: vec![place(3, 1, None, 0, 0), swf::Tag::ShowFrame],
        }),
        place(2, 1, Some("clip_avatar"), 64, 180),
        place(4, 2, Some("clip_goal"), 96, 180),
    ];
    for actions in &frames {
        tags.extend([swf::Tag::DoAction(actions), swf::Tag::ShowFrame]);
    }

    let mut header = swf::Header::default_with_swf_version(7);
    header.num_frames = frames.len() as u16;
    header.frame_rate = swf::Fixed8::from_f32(24.0);
    header.stage_size = swf::Rectangle {
        x_min: swf::Twips::ZERO,
        y_min: swf::Twips::ZERO,
        x_max: swf::Twips::from_pixels(640.0),
        y_max: swf::Twips::from_pixels(360.0),
    };
    let mut bytes = Vec::new();
    swf::write_swf(&header, &tags, &mut bytes).unwrap();
    bytes
}

fn square(id: u16, color: swf::Color) -> swf::Shape {
    use swf::{PointDelta, ShapeRecord, Twips};
    let bounds = swf::Rectangle {
        x_min: Twips::ZERO,
        y_min: Twips::ZERO,
        x_max: Twips::new(640),
        y_max: Twips::new(640),
    };
    swf::Shape {
        version: 3,
        id,
        shape_bounds: bounds,
        edge_bounds: bounds,
        flags: swf::ShapeFlag::empty(),
        styles: swf::ShapeStyles {
            fill_styles: vec![swf::FillStyle::Color(color)],
            line_styles: vec![],
        },
        shape: vec![
            ShapeRecord::StyleChange(Box::new(swf::StyleChangeData {
                move_to: Some(swf::Point::default()),
                fill_style_0: Some(1),
                fill_style_1: None,
                line_style: None,
                new_styles: None,
            })),
            ShapeRecord::StraightEdge {
                delta: PointDelta {
                    dx: Twips::new(640),
                    dy: Twips::ZERO,
                },
            },
            ShapeRecord::StraightEdge {
                delta: PointDelta {
                    dx: Twips::ZERO,
                    dy: Twips::new(640),
                },
            },
            ShapeRecord::StraightEdge {
                delta: PointDelta {
                    dx: Twips::new(-640),
                    dy: Twips::ZERO,
                },
            },
            ShapeRecord::StraightEdge {
                delta: PointDelta {
                    dx: Twips::ZERO,
                    dy: Twips::new(-640),
                },
            },
        ],
    }
}

fn place(id: u16, depth: u16, name: Option<&'static str>, x: i32, y: i32) -> swf::Tag<'static> {
    swf::Tag::PlaceObject(Box::new(swf::PlaceObject {
        version: 2,
        action: swf::PlaceObjectAction::Place(id),
        depth,
        matrix: Some(swf::Matrix {
            tx: swf::Twips::new(x * 20),
            ty: swf::Twips::new(y * 20),
            ..Default::default()
        }),
        color_transform: None,
        ratio: None,
        name: name.map(swf::SwfStr::from_utf8_str),
        clip_depth: None,
        class_name: None,
        filters: None,
        background_color: None,
        blend_mode: None,
        clip_actions: None,
        has_image: false,
        is_bitmap_cached: None,
        is_visible: None,
        amf_data: None,
    }))
}

