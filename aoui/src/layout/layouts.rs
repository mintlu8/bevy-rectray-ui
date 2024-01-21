use std::fmt::Debug;
use std::marker::PhantomData;

use bevy::ecs::entity::Entity;
use bevy::prelude::{Vec2, UVec2};
use bevy::reflect::std_traits::ReflectDefault;
use bevy::reflect::Reflect;
use downcast_rs::{impl_downcast, Downcast};
use crate::{Size2, SizeUnit, Size};

use super::{util::*, LayoutInfo, LayoutRange};

// asserts layout is object safe
const _: Option<Box<dyn Layout>> = None;

/// A layout that accepts a one dimensional sequence of widgets.
///
/// The `Container` is usually a dynamic sized widget,
/// meaning it will update its size based on the size occupied by its children.
/// You can parent it to an anchor of
/// a fixed sized widget for alignment.
pub trait Layout: Downcast + Debug + Send + Sync + 'static {
    /// Place sprites in the layout.
    fn place(&self, parent: &LayoutInfo, entities: Vec<LayoutItem>, range: &mut LayoutRange) -> LayoutOutput;
    /// Clone the layout.
    fn dyn_clone(&self) -> Box<dyn Layout>;
}

impl_downcast!(Layout);

#[derive(Debug, Reflect)]
#[reflect(Default)]
pub struct LayoutObject(#[reflect(ignore)]Box<dyn Layout>);

impl Default for LayoutObject {
    fn default() -> Self {
        Self(Box::new(BoundsLayout::PADDING))
    }
}

impl LayoutObject {
    pub fn new(layout: impl Layout) -> Self {
        Self(Box::new(layout))
    }

    pub fn place(&self, parent: &LayoutInfo, entities: Vec<LayoutItem>, range: &mut LayoutRange) -> LayoutOutput {
        self.0.place(parent, entities, range)
    }

    pub fn downcast<T: Layout>(&self) -> Option<&T>{
        self.0.downcast_ref()
    }

    pub fn into_downcast<T: Layout>(self) -> Result<T, Self>{
        match self.0.downcast() {
            Ok(x) => Ok(*x),
            Err(x) => Err(Self(x)),
        }
    }
}

impl Clone for LayoutObject {
    fn clone(&self) -> Self {
        Self(self.0.dyn_clone())
    }
}

impl<T> From<T> for LayoutObject where T: Layout {
    fn from(value: T) -> Self {
        Self(Box::new(value))
    }
}


/// Output of a layout, containing anchors of entities, and the computed dimension of the layout.
#[derive(Debug)]
pub struct LayoutOutput {
    pub entity_anchors: Vec<(Entity, Vec2)>,
    pub dimension: Vec2,
    /// Maximum value for the layout.
    pub max_count: usize,
}

impl LayoutOutput {
    pub fn normalized(mut self) -> Self{
        self.entity_anchors.iter_mut().for_each(|(_, x)| *x = *x / self.dimension - 0.5);
        self
    }
    pub fn with_max(mut self, max: usize) -> Self{
        self.max_count = max;
        self
    }
}

/// A dynamic dimensioned `Frame` that by default have size equal
/// to the maximum of its children.
///
/// This layout usually should contain only one child with no offset.
#[derive(Debug, Clone, Copy, Reflect)]
pub struct BoundsLayout {
    /// If set, use `Dimension` on that axis.
    pub fixed: [bool; 2],
    /// Minimum bounds.
    pub min: Size2,
    /// Maximum bounds.
    pub max: Size2,
}

impl BoundsLayout {
    /// Ignore constraints and use `BoundsLayout` as padding.
    pub const PADDING: Self = Self {
        fixed: [false; 2],
        min: Size2::ZERO,
        max: Size2::MAX
    };

    pub const fn from_max(max: Size2) -> Self{
        BoundsLayout {
            fixed: [false; 2],
            min: Size2::MAX,
            max,
        }
    }

    pub const fn from_min(min: Size2) -> Self{
        BoundsLayout {
            fixed: [false; 2],
            min,
            max: Size2::MAX
        }
    }

    pub const fn x_bounds(min: Size, max: Size) -> Self{
        BoundsLayout {
            fixed: [false, true],
            min: Size2::splat(min),
            max: Size2::splat(max),
        }
    }

    pub const fn y_bounds(min: Size, max: Size) -> Self{
        BoundsLayout {
            fixed: [true, false],
            min: Size2::splat(min),
            max: Size2::splat(max),
        }
    }


}

impl Default for BoundsLayout {
    fn default() -> Self {
        Self::PADDING
    }
}

impl Layout for BoundsLayout {
    fn place(&self, info: &LayoutInfo, entities: Vec<LayoutItem>, range: &mut LayoutRange) -> LayoutOutput {
        let mut max_dim = Vec2::ZERO;
        range.resolve(entities.len());
        let entity_anchors: Vec<_> = entities[range.to_range(entities.len())].iter().map(|x| {
            max_dim = max_dim.max(x.dimension);
            (x.entity, x.anchor.as_vec())
        }).collect();

        let min = self.min.as_pixels(info.dimension, info.em, info.rem);
        let max = self.max.as_pixels(info.dimension, info.em, info.rem);

        let dim = max_dim.clamp(min, max);

        let dimension = Vec2::new(
            if !self.fixed[0] {dim.x} else {info.dimension.x},
            if !self.fixed[1] {dim.y} else {info.dimension.y},
        );
        LayoutOutput { entity_anchors, dimension, max_count: entities.len() }
    }

    fn dyn_clone(&self) -> Box<dyn Layout> {
        Box::new(*self)
    }
}

/// A size agnostic mono-directional container.
#[derive(Debug, Default, Reflect)]
pub struct StackLayout<D: Direction = X>(#[reflect(ignore)] PhantomData<D>);

impl<D: Direction> Copy for StackLayout<D> {}
impl<D: Direction> Clone for StackLayout<D> {
    fn clone(&self) -> Self {
        *self
    }
}

impl StackLayout {
    /// A left to right layout.
    pub const HSTACK: StackLayout<X> = StackLayout(PhantomData);
    /// A top to bottom layout.
    pub const VSTACK: StackLayout<Rev<Y>> = StackLayout(PhantomData);
}

impl<D: Direction> StackLayout<D> {
    pub fn new() -> Self {
        StackLayout(PhantomData)
    }
}


/// A fix-sized mono-directional container.
#[derive(Debug, Default, Reflect)]
pub struct SpanLayout<D: StretchDir = X>(#[reflect(ignore)] PhantomData<D>);

impl<D: StretchDir> Copy for SpanLayout<D> {}
impl<D: StretchDir> Clone for SpanLayout<D> {
    fn clone(&self) -> Self {
        *self
    }
}

impl SpanLayout {
    /// A left to right layout with fixed dimension.
    pub const HBOX: SpanLayout<X> = SpanLayout(PhantomData);
    /// A top to bottom layout with fixed dimension.
    pub const VBOX: SpanLayout<Rev<Y>> = SpanLayout(PhantomData);
}

impl<D: StretchDir> SpanLayout<D> {
    pub fn new() -> Self {
        SpanLayout(PhantomData)
    }

    pub fn with_stretch(self) -> SpanLayout<Stretch<D>> {
        SpanLayout(PhantomData)
    }
}



/// A multiline version of the `span` layout, similar to the layout of a paragraph.
#[derive(Debug, Default, Reflect)]
pub struct ParagraphLayout<D1: StretchDir=X, D2: Direction=Rev<Y>>(#[reflect(ignore)] PhantomData<(D1, D2)>) where (D1, D2): DirectionPair;

impl<D1: StretchDir, D2: Direction> Copy for ParagraphLayout<D1, D2> where (D1, D2): DirectionPair {}
impl<D1: StretchDir, D2: Direction> Clone for ParagraphLayout<D1, D2> where (D1, D2): DirectionPair {
    fn clone(&self) -> Self {
        *self
    }
}

impl ParagraphLayout {
    /// A left to right, top to bottom paragraph, similar to the default layout of a webpage.
    pub const PARAGRAPH: Self = Self(PhantomData);
}

impl<D1: StretchDir, D2: Direction> ParagraphLayout<D1, D2> where (D1, D2): DirectionPair {
    pub fn new() -> Self {
        Self(PhantomData)
    }

    pub fn with_stretch(self) -> ParagraphLayout<Stretch<D1>, D2> where (Stretch<D1>, D2): DirectionPair {
        ParagraphLayout::<Stretch<D1>, D2>(PhantomData)
    }
}

/// A 2D grid wih even pre-subdivided cells.
///
/// # Panics
///
/// * If `row_dir` is not orthogonal to `column_dir`.
#[derive(Debug, Clone, Copy, Reflect)]
pub struct SizedGridLayout {
    /// Determines the size of a cell.
    pub cell_size: Size2,
    /// The order of which continuous items are placed.
    pub row_dir: LayoutDir,
    /// The order of which rows are placed.
    pub column_dir: LayoutDir,
    /// How items in a incomplete row are aligned.
    ///
    /// Significant when an early linebreak occurs.
    pub alignment: Alignment,
    /// if specified, adjust cell size to fill the grid without changing cell count.
    pub stretch: bool,
}

/// A 2D grid wih even pre-subdivided cells.
///
/// # Panics
///
/// * If `row_dir` is not orthogonal to `column_dir`.
#[derive(Debug, Clone, Copy, Reflect)]
pub struct FixedGridLayout {
    /// Determines the number of cells
    pub cells: UVec2,
    /// The order of which continuous items are placed.
    pub row_dir: LayoutDir,
    /// The order of which rows are placed.
    pub column_dir: LayoutDir,
    /// How items in a incomplete row are aligned.
    ///
    /// Significant when an early linebreak occurs.
    pub alignment: Alignment,
}

/// A 2D grid with unevenly subdivided cells.
///
/// # Panics
///
/// * If `row_dir` is not orthogonal to `column_dir`.
#[derive(Debug, Clone, Reflect)]
pub struct DynamicTableLayout {
    /// Determines the number of columns, use a large number for infinite.
    pub columns: usize,
    /// The order of which continuous items are placed.
    pub row_dir: LayoutDir,
    /// The order of which rows are placed.
    pub column_dir: LayoutDir,
    /// If specified, adjust row margin to fill the table.
    pub stretch: bool,
}

/// A 2D grid with unevenly subdivided cells.
///
/// # Panics
///
/// * If `row_dir` is not orthogonal to `column_dir`.
#[derive(Debug, Clone, Reflect)]
pub struct TableLayout {
    /// Determines the number and size of columns
    pub columns: Vec<(SizeUnit, f32)>,
    /// The order of which continuous items are placed.
    pub row_dir: LayoutDir,
    /// The order of which rows are placed.
    pub column_dir: LayoutDir,
    /// If specified, adjust row margin to fill the table.
    pub stretch: bool,
}

impl TableLayout {
    pub fn from_columns(columns: impl Into<Vec<(SizeUnit, f32)>>) -> Self {
        Self {
            columns: columns.into(),
            row_dir: LayoutDir::LeftToRight,
            column_dir: LayoutDir::TopToBottom,
            stretch: false,
        }
    }
}
