
macro_rules! syn {
    ($(#[$($tt:tt)*])*$vis: vis struct $name: ident {$($(#[$($tt2:tt)*])*$field: ident: $ty: ident),* $(,)?}) => {
        mod highlighting {
            $(
                #[allow(nonstandard_style)]
                pub mod $field {
                    #[allow(nonstandard_style)]
                    #[derive(Debug, Default)]
                    pub struct $ty;
                }
            )*
        }
        $(#[$($tt)*])*
        $vis struct $name {
            $(
                $(#[$($tt2)*])*
                $field: highlighting::$field::$ty
            ),*
        }
    };
}

syn!(
    #[doc(hidden)]
    #[allow(nonstandard_style)]
    #[derive(Debug, Default)]
    pub struct aoui {
        /// Center of entity, default to `anchor`.
        center: Anchor,
        /// Anchor of entity, default is `Center`.
        anchor: Anchor,
        /// Offset from parent anchor, default is `[0, 0]`.
        offset: Size2,
        /// Rotation from `center`, default is `0.0`.
        rotation: f32,
        /// Scale from `center`, default is `[1, 1]`.
        scale: Vec2,
        /// Depth from parent, default is `0 + 8eps`.
        z: f32,
        /// Owned dimension of entity, default is `Copied`.
        dimension: Size2,
        /// Size of `Sprite`, `Text2dBounds`, etc.
        /// 
        /// Default value depand on the context.
        size: Vec2,
        /// Modify relative size. Default is `None`.
        em: SetEm,

        /// If specified, enable special behavior.
        widget: Ident,
        /// Extra non-dsl specific components.
        extra: Component,
        /// Add a child entity.
        child: Entity,
        /// Add a special children with naegative depth.
        background: Entity,
        /// Linebreak after this entity, default is `false`.
        linebreak: bool,
        /// Position in `SparseLayout`.
        position: UVec2,
        /// Enable transform 
        build_transform: bool,
        hitbox: HitboxShape,
        hitbox_size: Vec2,
        /// Alias for `extra: { EventMarker }`
        event: Ident,

        sprite: Expression,
        color: Color,
        rect: Rect,
        flip: BVec2,

        text: String,
        font: Expression,

        flex: Layout,
        margin: Size2,
        direction: FlexDir,
        stack: FlexDir,
        alignment: Alignment,
        row_dir: FlexDir,
        column_dir: FlexDir,
        row_align: Alignment,
        column_align: Alignment,
        cell_size: Vec2,
        cell_count: UVec2,
        columns: Columns,
        stretch: bool,

        scene: SparseLayout,
        x_axis: Direction,
        y_axis: Direction,
        origin: Vec2,
        cell_rect: Rect,
        scene_transform: Affine2,
    }
);