/**
 * 教程起源：https://mbuffett.com/posts/bevy-snake-tutorial/
 * 新增：
 * 1、增加左上角计分功能
 * 2、修复 Food 概率出现在蛇身上的问题
 * 3、修复快速操作下概率出现的 180度 转身问题
 */
pub mod snake_mod {
    use bevy::core::FixedTimestep;
    use bevy::prelude::*;
    use rand::prelude::random;

    const ARENA_WIDTH: u32 = 10;
    const ARENA_HEIGHT: u32 = 10;
    const SNAKE_HEAD_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);
    const SNAKE_SEGMENT_COLOR: Color = Color::rgb(0.3, 0.3, 0.3);
    const FOOD_COLOR: Color = Color::rgb(1.0, 0.0, 1.0);

    #[derive(Component)]
    struct SnakeHead {
        direction: Direction,
    }

    #[derive(Component)]
    struct SnakeSegment;
    #[derive(Default, Deref, DerefMut)]
    struct SnakeSegments(Vec<Entity>);

    #[derive(Component)]
    struct Food;

    #[derive(Component, Clone, Copy, PartialEq, Eq)]
    struct Position {
        x: i32,
        y: i32,
    }
    #[derive(Default)]
    struct LastTailPosition(Option<Position>);

    #[derive(Component)]
    struct Size {
        width: f32,
        height: f32,
    }
    impl Size {
        pub fn square(x: f32) -> Self {
            Self {
                width: x,
                height: x,
            }
        }
    }

    #[derive(PartialEq, Clone, Copy)]
    enum Direction {
        Left,
        Up,
        Right,
        Down,
    }
    impl Direction {
        fn opposite(self) -> Self {
            match self {
                Self::Left => Self::Right,
                Self::Right => Self::Left,
                Self::Up => Self::Down,
                Self::Down => Self::Up,
            }
        }
    }

    struct GrowthEvent;
    struct GameOverEvent;

    pub fn start_snake() {
        App::new()
            .add_plugins(DefaultPlugins)
            .insert_resource(WindowDescriptor {
                title: "Snake!".to_string(),
                width: 500.0,
                height: 500.0,
                ..default()
            })
            .add_startup_system(setup_camera)
            .add_startup_system(spawn_snake)
            // 每秒加一个 Food
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(1.0))
                    .with_system(food_spawner),
            )
            // fn 上的 before 等接口属于扩展实现的，方式详见：https://users.rust-lang.org/t/solved-implementing-a-trait-for-function-closure-types/9371
            // 孤儿规则：当你为某类型实现某 trait 的时候，必须要求类型或者 trait 至少有一个是在当前 crate 中定义的。你不能为第三方的类型实现第三方的 trait
            .add_system(snake_movement_input.before(snake_movement))
            // 将尺寸与位置的重计算放在 Update 阶段之后的 PostUpdate
            // sanke_movement 是在 Update 阶段，我们期望位置与尺寸的重计算在其之后
            .add_system_set_to_stage(
                CoreStage::PostUpdate,
                SystemSet::new()
                    .with_system(position_translation)
                    .with_system(size_scaling),
            )
            // 控制频率
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(0.150))
                    .with_system(snake_movement)
                    .with_system(snake_eating.after(snake_movement))
                    .with_system(snake_growth.after(snake_eating)),
            )
            .add_system(game_over.after(snake_movement))
            .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
            .insert_resource(SnakeSegments::default())
            .insert_resource(LastTailPosition::default())
            .add_event::<GrowthEvent>()
            .add_event::<GameOverEvent>()
            .run();
    }

    fn setup_camera(mut commands: Commands) {
        commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    }

    fn spawn_snake(mut commands: Commands, mut segments: ResMut<SnakeSegments>) {
        *segments = SnakeSegments(vec![
            commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: SNAKE_HEAD_COLOR,
                        ..default()
                    },
                    ..default()
                })
                .insert(SnakeHead {
                    direction: Direction::Up,
                })
                .insert(SnakeSegment)
                .insert(Position { x: 3, y: 3 })
                .insert(Size::square(0.8))
                .id(),
            spawn_segment(commands, Position { x: 3, y: 2 }),
        ]);
    }

    fn spawn_segment(mut commands: Commands, position: Position) -> Entity {
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: SNAKE_SEGMENT_COLOR,
                    ..default()
                },
                ..default()
            })
            .insert(SnakeSegment)
            .insert(position)
            .insert(Size::square(0.65))
            .id()
    }

    fn food_spawner(mut commands: Commands) {
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: FOOD_COLOR,
                    ..default()
                },
                ..default()
            })
            .insert(Food)
            .insert(Position {
                x: (random::<f32>() * ARENA_WIDTH as f32) as i32,
                y: (random::<f32>() * ARENA_HEIGHT as f32) as i32,
            })
            .insert(Size::square(0.8));
    }

    fn snake_movement_input(keyboard_input: Res<Input<KeyCode>>, mut heads: Query<&mut SnakeHead>) {
        if let Some(mut head) = heads.iter_mut().next() {
            let dir: Direction = if keyboard_input.pressed(KeyCode::Left)
                || keyboard_input.pressed(KeyCode::A)
            {
                Direction::Left
            } else if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
                Direction::Right
            } else if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
                Direction::Down
            } else if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
                Direction::Up
            } else {
                head.direction
            };
            if dir != head.direction.opposite() {
                head.direction = dir;
            }
        }
    }

    /**
     * 比如 Query<&mut Transform, With<SnakeHead>>
     * 该语句可以通过 Query 遍历同时具有 SnakeHead 与 Transform 的 Component
     * Bevy 会通过 ECS 的架构实现，自动完成 Query 的创建以及指定 Component 的筛选
     * 如果我们实际操作中不会对 SnakeHead 做操作，那么这里也可以采用 With 特性
     * 在 Bevy 中每个 System 中访问越少的 Component 就会带来越好的并发性
     */
    fn snake_movement(
        mut last_tail_position: ResMut<LastTailPosition>,
        mut game_over_writer: EventWriter<GameOverEvent>,
        segments: ResMut<SnakeSegments>,
        mut heads: Query<(Entity, &SnakeHead)>,
        mut positions: Query<&mut Position>,
    ) {
        if let Some((head_entity, head)) = heads.iter_mut().next() {
            let segment_positions = segments
                .iter()
                .map(|e| *positions.get_mut(*e).unwrap())
                .collect::<Vec<Position>>();
            let mut head_pos = positions.get_mut(head_entity).unwrap();
            match &head.direction {
                Direction::Left => {
                    head_pos.x -= 1;
                }
                Direction::Right => {
                    head_pos.x += 1;
                }
                Direction::Up => {
                    head_pos.y += 1;
                }
                Direction::Down => {
                    head_pos.y -= 1;
                }
            };
            if head_pos.x < 0
                || head_pos.y < 0
                || head_pos.x as u32 >= ARENA_WIDTH
                || head_pos.y as u32 >= ARENA_HEIGHT
            {
                game_over_writer.send(GameOverEvent);
            }
            if segment_positions.contains(&head_pos) {
                game_over_writer.send(GameOverEvent);
            }
            // 跳过蛇头，用前一个节点的位置作为后一个的节点的位置
            segment_positions
                .iter()
                .zip(segments.iter().skip(1))
                .for_each(|(pos, segment)| {
                    *positions.get_mut(*segment).unwrap() = *pos;
                });
            // 移动之后原本最后一个图块的位置就是当吃到食物时的位置
            *last_tail_position = LastTailPosition(Some(*segment_positions.last().unwrap()));
        }
    }

    /**
     * 将尺寸缩放换算到 ARENA_WIDTH * ARENA_HEIGHT 的空间里面
     */
    fn size_scaling(windows: Res<Windows>, mut q: Query<(&Size, &mut Transform)>) {
        let window = windows.get_primary().unwrap();
        for (sprite_size, mut transform) in q.iter_mut() {
            transform.scale = Vec3::new(
                sprite_size.width / ARENA_WIDTH as f32 * window.width() as f32,
                sprite_size.height / ARENA_HEIGHT as f32 * window.height() as f32,
                1.0,
            );
        }
    }

    /**
     * 假设物体的 x 坐标是 5，我们的游戏系统宽度是 10，实际窗口宽度是 200，那么这个坐标会按如下公示计算：
     * 5 / 10 * 200 - 200 / 2 = 0
     * 减去一半的窗口宽度是因为我们的图块是从屏幕中心开始的，而不是从左下角
     * 加上一半的图块宽或高（tile_size）是因为我们希望图块自身坐标原点是在左下角而不是中心点
     */
    fn position_translation(windows: Res<Windows>, mut q: Query<(&Position, &mut Transform)>) {
        fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
            let tile_size = bound_window / bound_game;
            pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
        }
        let window = windows.get_primary().unwrap();
        for (pos, mut transform) in q.iter_mut() {
            transform.translation = Vec3::new(
                convert(pos.x as f32, window.width() as f32, ARENA_WIDTH as f32),
                convert(pos.y as f32, window.height() as f32, ARENA_HEIGHT as f32),
                0.0,
            );
        }
    }

    fn snake_eating(
        mut commands: Commands,
        mut growth_writer: EventWriter<GrowthEvent>,
        food_positions: Query<(Entity, &Position), With<Food>>,
        head_positions: Query<&Position, With<SnakeHead>>,
    ) {
        for head_pos in head_positions.iter() {
            for (ent, food_pos) in food_positions.iter() {
                if food_pos == head_pos {
                    commands.entity(ent).despawn();
                    growth_writer.send(GrowthEvent);
                }
            }
        }
    }

    fn snake_growth(
        commands: Commands,
        last_tail_position: Res<LastTailPosition>,
        mut segments: ResMut<SnakeSegments>,
        mut growth_reader: EventReader<GrowthEvent>,
    ) {
        if growth_reader.iter().next().is_some() {
            segments.push(spawn_segment(commands, last_tail_position.0.unwrap()))
        }
    }

    fn game_over(
        mut commands: Commands,
        mut reader: EventReader<GameOverEvent>,
        segments_res: ResMut<SnakeSegments>,
        food: Query<Entity, With<Food>>,
        segments: Query<Entity, With<SnakeSegment>>,
    ) {
        if reader.iter().next().is_some() {
            for entity in food.iter().chain(segments.iter()) {
                commands.entity(entity).despawn();
            }
            spawn_snake(commands, segments_res);
        }
    }
}
